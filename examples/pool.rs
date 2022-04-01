use bb8_lapin::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let amqp_url = std::env::var("TEST_AMQP_URL").unwrap_or_else(|_| "amqp://guest:guest@127.0.0.1:5672//".to_string());

    let manager = LapinConnectionManager::new(&amqp_url, ConnectionProperties::default());
    let pool = bb8::Pool::builder().max_size(15).build(manager).await.unwrap();

    for _ in 0..20 {
        let pool = pool.clone();
        tokio::spawn(async move {
            let _conn = pool.get().await.unwrap();
            // use the connection
            // it will be returned to the pool when it falls out of scope.
        });
    }
}
