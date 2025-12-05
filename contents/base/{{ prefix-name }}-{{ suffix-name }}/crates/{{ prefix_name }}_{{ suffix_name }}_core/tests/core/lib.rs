{% if persistence != 'None' %}use anyhow::Result;
use {{ prefix_name }}_{{ suffix_name }}_core::proto::{{ prefix_name }}_{{ suffix_name }}_server::{{ PrefixName }}{{ SuffixName }};
use {{ prefix_name }}_{{ suffix_name }}_core::proto::{
    Create{{ PrefixName }}Request, Create{{ PrefixName }}Response, Get{{ PrefixName }}ListRequest,
    Get{{ PrefixName }}ListResponse,
};
use {{ prefix_name }}_{{ suffix_name }}_core::{{ PrefixName }}{{ SuffixName }}Core;
use {{ prefix_name }}_{{ suffix_name }}_persistence::{{ PrefixName }}{{ SuffixName }}Persistence;
use tonic::Request;

#[tokio::test]
async fn test_create_{{ prefix_name }}() -> Result<()> {
    let core = core().await?;

    let response = core
        .get_{{ prefix_name }}_list(Request::new(Get{{ PrefixName }}ListRequest { page_size: 0, page: 0 }))
        .await?;
    let Get{{ PrefixName }}ListResponse { records, total_pages } = response.into_inner();
    assert_eq!(records.len(), 0);
    assert_eq!(total_pages, 0);

    let response = core
        .create_{{ prefix_name }}(Request::new(Create{{ PrefixName }}Request {
            contents: "Contents".to_string(),
        }))
        .await?;
    let Create{{ PrefixName }}Response { record } = response.into_inner();
    let record = record.expect("Record Expected");
    assert_eq!(&record.contents, "Contents");

    let response = core
        .get_{{ prefix_name }}_list(Request::new(Get{{ PrefixName }}ListRequest { page_size: 0, page: 0 }))
        .await?;
    let Get{{ PrefixName }}ListResponse { records, total_pages } = response.into_inner();
    assert_eq!(records.len(), 1);
    assert_eq!(total_pages, 1);

    Ok(())
}

async fn core() -> Result<{{ PrefixName }}{{ SuffixName }}Core> {
    let persistence = {{ PrefixName }}{{ SuffixName }}Persistence::builder()
        .with_temp_db()
        .build()
        .await?;
    let core = {{ PrefixName }}{{ SuffixName }}Core::builder(persistence)
        .build()
        .await?;
    Ok(core)
}
{% else %}use anyhow::Result;
use {{ prefix_name }}_{{ suffix_name }}_core::{{ PrefixName }}{{ SuffixName }}Core;

#[tokio::test]
async fn test_core_builds() -> Result<()> {
    let _core = core().await?;
    Ok(())
}

async fn core() -> Result<{{ PrefixName }}{{ SuffixName }}Core> {
    let core = {{ PrefixName }}{{ SuffixName }}Core::builder()
        .build()
        .await?;
    Ok(core)
}
{% endif %}