{% if persistence != 'None' %}mod conversion;
{% endif %}mod r#impl;
pub mod settings;

use anyhow::Result;

use crate::settings::CoreSettings;
{% if persistence != 'None' %}use {{ prefix_name }}_{{ suffix_name }}_persistence::{{ PrefixName }}{{ SuffixName }}Persistence;
{% endif %}
pub mod proto {
    tonic::include_proto!("{{ prefix_name }}_{{ suffix_name }}");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("{{ prefix_name }}_{{ suffix_name }}");
}

#[derive(Clone, Debug)]
pub struct {{ PrefixName }}{{ SuffixName }}Core {
{% if persistence != 'None' %}    persistence: {{ PrefixName }}{{ SuffixName }}Persistence,
{% endif %}    #[allow(dead_code)]
    settings: CoreSettings,
}

impl {{ PrefixName }}{{ SuffixName }}Core {
{% if persistence != 'None' %}    pub fn builder(persistence: {{ PrefixName }}{{ SuffixName }}Persistence) -> Builder {
        Builder::new(persistence)
    }
{% else %}    pub fn builder() -> Builder {
        Builder::new()
    }
{% endif %}}

pub struct Builder {
{% if persistence != 'None' %}    persistence: {{ PrefixName }}{{ SuffixName }}Persistence,
{% endif %}    settings: CoreSettings,
}

impl Builder {
{% if persistence != 'None' %}    pub fn new(persistence: {{ PrefixName }}{{ SuffixName }}Persistence) -> Self {
        Self {
            persistence,
            settings: Default::default(),
        }
    }
{% else %}    pub fn new() -> Self {
        Self {
            settings: Default::default(),
        }
    }
{% endif %}
    pub fn with_settings(mut self, settings: &CoreSettings) -> Self {
        self.settings = settings.clone();
        self
    }

    pub async fn build(self) -> Result<{{ PrefixName }}{{ SuffixName }}Core> {
        Ok({{ PrefixName }}{{ SuffixName }}Core {
{% if persistence != 'None' %}            persistence: self.persistence,
{% endif %}            settings: self.settings,
        })
    }
}