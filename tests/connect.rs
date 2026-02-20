use bb8_lapin::prelude::*;
use std::sync::Arc;

lazy_static::lazy_static! {
    static ref AMQP_URL: String = {
        dotenv::dotenv().ok();
        std::env::var("TEST_AMQP_URL").unwrap_or_else(|_| "amqp://guest:guest@127.0.0.1:5672//".to_string())
    };
}

#[tokio::test]
async fn can_connect() {
    let builder = ConnectionBuilder::new_with_runtime(Runtime::tokio_current()).with_uri_str(AMQP_URL.to_owned());
    let manager = LapinConnectionManager::new(builder);
    let pool = Arc::new(
        bb8::Pool::builder()
            .max_size(2)
            .test_on_check_out(true)
            .build(manager)
            .await
            .expect("Should create pool"),
    );
    let n_tasks = 100;
    let (tx, mut rx) = tokio::sync::mpsc::channel(n_tasks);
    for i in 0..n_tasks {
        let pool = pool.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            let delay_ms = n_tasks - i;
            tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms as u64)).await;
            let conn = pool.get().await.expect("Should get connection");
            tx.send(conn.create_channel().await).await.unwrap();
        });
    }
    for _ in 0..n_tasks {
        if let Some(create_channel_result) = rx.recv().await {
            assert!(create_channel_result.is_ok());
        }
    }
}
