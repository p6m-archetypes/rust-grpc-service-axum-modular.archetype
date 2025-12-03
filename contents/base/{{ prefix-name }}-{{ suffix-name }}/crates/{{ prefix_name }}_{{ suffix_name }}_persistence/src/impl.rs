use crate::sea_orm::entity::prelude::*;
use crate::{{'{'}}{{ PrefixName }}{{ SuffixName }}Persistence, DbResult};

use crate::entities::*;
use crate::page::Page;

impl {{ PrefixName }}{{ SuffixName }}Persistence {
    pub async fn find_{{ prefix_name }}(
        &self,
        id: Uuid,
    ) -> DbResult<Option<{{ prefix_name }}::Model>> {
        let record = {{ prefix_name }}::Entity::find_by_id(id).one(self.connection()).await?;
        Ok(record)
    }

    pub async fn insert_{{ prefix_name }}(
        &self,
        {{ prefix_name }}_record: {{ prefix_name }}::ActiveModel,
    ) -> DbResult<{{ prefix_name }}::Model> {
        let result = {{ prefix_name }}_record.insert(self.connection()).await?;
        Ok(result)
    }

    pub async fn update_{{ prefix_name }}(
        &self,
        {{ prefix_name }}_record: {{ prefix_name }}::ActiveModel,
    ) -> DbResult<{{ prefix_name }}::Model> {
        let result = {{ prefix_name }}_record.update(self.connection()).await?;
        Ok(result)
    }

    pub async fn get_{{ prefix_name }}_list(
        &self,
        page_index: u32,
        page_size: u32,
    ) -> DbResult<Page<{{ prefix_name }}::Model>> {
        let page_size = page_size.min(100);
        let paginator =
            {{ prefix_name }}::Entity::find().paginate(self.connection(), page_size as usize);

        let records = paginator.fetch_page(page_index as usize).await?;
        let total_records = paginator.num_items().await? as u64;

        Ok(Page::new(records, page_index, page_size, total_records))
    }
}