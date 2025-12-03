mod conversion;
mod r#impl;
pub mod settings;

use anyhow::Result;

use crate::settings::CoreSettings;
use {{ prefix_name }}_{{ suffix_name }}_persistence::{{ PrefixName }}{{ SuffixName }}Persistence;

pub mod proto {
    tonic::include_proto!("{{ prefix_name }}_{{ suffix_name }}");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("{{ prefix_name }}_{{ suffix_name }}");
}

#[derive(Clone, Debug)]
pub struct {{ PrefixName }}{{ SuffixName }}Core {
    persistence: {{ PrefixName }}{{ SuffixName }}Persistence,
}

impl {{ PrefixName }}{{ SuffixName }}Core {
    pub fn builder(persistence: {{ PrefixName }}{{ SuffixName }}Persistence) -> Builder {
        Builder::new(persistence)
    }
}

pub struct Builder {
    persistence: {{ PrefixName }}{{ SuffixName }}Persistence,
    settings: CoreSettings,
}

impl Builder {
    pub fn new(persistence: {{ PrefixName }}{{ SuffixName }}Persistence) -> Self {
        Self {
            persistence,
            settings: Default::default(),
        }
    }

    pub fn with_settings(mut self, settings: &CoreSettings) -> Self {
        self.settings = settings.clone();
        self
    }

    pub async fn build(self) -> Result<{{ PrefixName }}{{ SuffixName }}Core> {
        Ok({{ PrefixName }}{{ SuffixName }}Core {
            persistence: self.persistence,
        })
    }
}