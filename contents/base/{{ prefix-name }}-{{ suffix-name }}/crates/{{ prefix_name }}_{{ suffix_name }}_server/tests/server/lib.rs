use anyhow::Result;
use {{ prefix_name }}_{{ suffix_name }}_client::proto::{{ prefix_name }}_{{ suffix_name }}_client::{{ PrefixName }}{{ SuffixName }}Client;
use {{ prefix_name }}_{{ suffix_name }}_client::proto::Create{{ PrefixName }}Request;
use {{ prefix_name }}_{{ suffix_name }}_core::{{ PrefixName }}{{ SuffixName }}Core;
use {{ prefix_name }}_{{ suffix_name }}_persistence::{{ PrefixName }}{{ SuffixName }}Persistence;
use {{ prefix_name }}_{{ suffix_name }}_server::{{ PrefixName }}{{ SuffixName }}Server;
use tonic::transport::Channel;
use tonic::Request;

#[tokio::test]
async fn test_core() -> Result<()> {
    let (mut client, _) = init().await?;

    let request = Request::new(Create{{ PrefixName }}Request {
        contents: "Contents".to_string(),
    });

    let response = client.create_{{ prefix_name }}(request).await?;
    let response = response.into_inner();
    assert_eq!(response.record.unwrap().contents, "Contents".to_owned());

    Ok(())
}

async fn init() -> Result<({{ PrefixName }}{{ SuffixName }}Client<Channel>, {{ PrefixName }}{{ SuffixName }}Server)> {
    let persistence = {{ PrefixName }}{{ SuffixName }}Persistence::builder()
        .with_temp_db()
        .build()
        .await?;
    let core = {{ PrefixName }}{{ SuffixName }}Core::builder(persistence)
        .build()
        .await?;
    let server = {{ PrefixName }}{{ SuffixName }}Server::builder(core)
        .with_random_port()
        .build()
        .await?;

    let server_clone = server.clone();

    tokio::spawn(async move {
        let _ = server_clone.serve().await;
    });

    let addr = format!("http://localhost:{}", server.service_port());
    let client = {{ PrefixName }}{{ SuffixName }}Client::connect(addr).await?;

    Ok((client, server))
}