//! Lapin support for the `bb8` connection pool.
#![deny(missing_docs, missing_debug_implementations)]

pub use bb8;
pub use lapin;

/// Basic types to create a `LapinConnectionManager` instance.
pub mod prelude;

use async_rs::traits::RuntimeKit;
use lapin::protocol::{AMQPError, AMQPErrorKind, AMQPHardError};
use lapin::types::ShortString;
use lapin::ConnectionBuilder;

/// A `bb8::ManageConnection` implementation for `lapin::Connection`s.
///
/// # Example
/// ```no_run
/// use bb8_lapin::prelude::*;
///
/// async fn example() {
///     let manager = LapinConnectionManager::new(
///         DefaultConnectionBuilder::new()
///             .unwrap()
///             .with_uri_str("amqp://guest:guest@127.0.0.1:5672//".to_string())
///     );
///     let pool = bb8::Pool::builder()
///         .max_size(15)
///         .build(manager)
///         .await
///         .unwrap();
///
///     for _ in 0..20 {
///         let pool = pool.clone();
///         tokio::spawn(async move {
///             let conn = pool.get().await.unwrap();
///             // use the connection
///             // it will be returned to the pool when it falls out of scope.
///         });
///     }
/// }
/// ```
#[derive(Debug)]
pub struct LapinConnectionManager<RK: RuntimeKit + Send + Sync + Clone + 'static> {
    conn_builder: ConnectionBuilder<RK>,
}

impl<RK: RuntimeKit + Send + Sync + Clone + 'static> LapinConnectionManager<RK> {
    /// Initialize the connection manager with the data needed to create new connections.
    /// Refer to the documentation of [`lapin::ConnectionBuilder`](https://docs.rs/lapin/latest/lapin/struct.ConnectionBuilder.html)
    /// for further details on connection settings.
    ///
    /// # Example
    /// ```
    /// # tokio_test::block_on(async {
    /// let manager = bb8_lapin::LapinConnectionManager::new(
    ///     lapin::DefaultConnectionBuilder::new().unwrap()
    ///         .with_uri_str("amqp://guest:guest@127.0.0.1:5672//".to_string())
    /// );
    /// # })
    /// ```
    pub fn new(conn_builder: ConnectionBuilder<RK>) -> Self {
        Self { conn_builder }
    }
}

impl<RK: RuntimeKit + Send + Sync + Clone + 'static> bb8::ManageConnection for LapinConnectionManager<RK> {
    type Connection = lapin::Connection;
    type Error = lapin::ErrorKind;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.conn_builder.connect().await.map_err(|e| e.kind().to_owned())
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let conn_status = conn.status();
        if !conn_status.closing() && !conn_status.closed() && !conn_status.errored() {
            Ok(())
        } else {
            Err(lapin::ErrorKind::ProtocolError(AMQPError::new(
                AMQPErrorKind::Hard(AMQPHardError::CONNECTIONFORCED),
                ShortString::from("Invalid connection"),
            )))
        }
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        let conn_status = conn.status();
        conn_status.closed() || conn_status.errored()
    }
}
