use std::sync::Arc;

use anyhow::Result;
pub use sea_orm;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use sea_schema::migration::migrator::MigratorTrait;
use testcontainers_async::{Container, Image};
use testcontainers_async::modules::mysql::{MySqlContainer, MySqlImage};

pub use page::Page;

use crate::settings::PersistenceSettings;

pub mod entities;
mod r#impl;
mod migrations;
mod page;
pub mod settings;

pub type DbResult<T> = core::result::Result<T, DbErr>;

#[derive(Clone, Debug)]
pub struct {{ PrefixName }}{{ SuffixName }}Persistence {
    connection: DatabaseConnection,
    #[allow(dead_code)]
    temp_db: Option<Arc<MySqlContainer>>,
}

impl {{ PrefixName }}{{ SuffixName }}Persistence {
    pub fn builder() -> Builder {
        Builder {
            settings: PersistenceSettings::default(),
        }
    }

    pub fn connection(&self) -> &DatabaseConnection {
        &self.connection
    }

    pub async fn migrate_up(&self, steps: Option<u32>) -> DbResult<()> {
        migrations::Migrator::up(self.connection(), steps).await
    }

    pub async fn migrate_down(&self, steps: Option<u32>) -> DbResult<()> {
        migrations::Migrator::down(self.connection(), steps).await
    }
}

pub struct Builder {
    settings: PersistenceSettings,
}

impl Builder {
    pub fn with_settings(mut self, settings: &PersistenceSettings) -> Self {
        self.settings = settings.clone();
        self
    }

    pub fn with_temp_db(mut self) -> Self {
        self.settings.set_temp_db(true);
        self
    }

    pub async fn build(self) -> Result<{{ PrefixName }}{{ SuffixName }}Persistence> {
        let (connect_url, temp_db) = if let Some(true) = self.settings.temp_db() {
            let temp_db = MySqlImage::default()
                .with_database("{{ prefix_name }}_{{ suffix_name }}")
                .with_username("test")
                .with_password("test")
                .start_container()
                .await?;

            let port = temp_db.host_port_for("3306/tcp").await?;
            let connect_url = format!("mysql://test:test@localhost:{}/{{ prefix_name }}_{{ suffix_name }}", port);
            tracing::info!("TestContainer MySQL URL: {connect_url}");

            (connect_url, Some(Arc::new(temp_db)))
        } else {
            (self.settings.database().url().to_string(), None)
        };

        let mut options = ConnectOptions::new(connect_url);
        if let Some(value) = self.settings.database().max_connections() {
            options.max_connections(value);
        }
        if let Some(value) = self.settings.database().min_connections() {
            options.min_connections(value);
        }
        if let Some(value) = self.settings.database().connect_timeout() {
            options.connect_timeout(value);
        }
        if let Some(value) = self.settings.database().idle_timeout() {
            options.idle_timeout(value);
        }
        if let Some(value) = self.settings.database().max_lifetime() {
            options.max_lifetime(value);
        }
        options.sqlx_logging(self.settings.database().log_sql());

        let connection: DatabaseConnection = Database::connect(options).await?;

        if self.settings.migrate().or(Some(false)).unwrap() || temp_db.is_some() {
            migrations::Migrator::up(&connection, None).await?;
        }

        Ok({{ PrefixName }}{{ SuffixName }}Persistence { connection, temp_db })
    }
}
