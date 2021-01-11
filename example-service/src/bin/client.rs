use example_service::{Input, Output};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_env("LOG_LEVEL"))
        .init();
    let client = persia_rpc::RpcClient::new("127.0.0.1:8080")?;
    let rpc_client = example_service::ServiceClient::new(client);

    for _ in 0..10 {
        let _result = rpc_client.large_body_rpc_test(&vec![0.; 20971520]).await.unwrap();
    }

    for _ in 0..10 {
        let _result: Output = dbg!(
            rpc_client
                .rpc_test(&Input { msg: "haha".into() })
                .await
        ).unwrap();
    }

    for _ in 0..10 {
        let _result: Output = dbg!(
            rpc_client
                .rpc_test_compressed(&Input { msg: "haha".into() })
                .await
        ).unwrap();
    }

    for _ in 0..10 {
        let _result: Output = dbg!(
            rpc_client
                .rpc_test_2(&())
                .await
        ).unwrap();
    }

    Ok(())
}
