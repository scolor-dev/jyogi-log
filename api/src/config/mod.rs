pub mod env;

use std::{fmt, net::SocketAddr};

use crate::error::ConfigError;

#[derive(Clone)]
pub struct Config {
    pub app_host: String,
    pub app_port: u16,
    pub rust_log: String,
    pub database_url: String,
    pub db_max_connections: u32,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("app_host", &self.app_host)
            .field("app_port", &self.app_port)
            .field("rust_log", &self.rust_log)
            .field("database_url", &"[redacted]")
            .field("db_max_connections", &self.db_max_connections)
            .finish()
    }
}

impl Config {
    #[must_use]
    pub fn new(
        app_host: String,
        app_port: u16,
        rust_log: String,
        database_url: String,
        db_max_connections: u32,
    ) -> Self {
        Self { app_host, app_port, rust_log, database_url, db_max_connections }
    }

    /// プロセス環境変数からアプリケーション設定を読み込みます。
    ///
    /// # Errors
    ///
    /// 環境変数が設定されていて、その値が不正な場合にエラーを返します。
    pub fn from_env() -> Result<Self, ConfigError> {
        env::load()
    }

    /// HTTP サーバーが bind するソケットアドレスを組み立てます。
    ///
    /// # Errors
    ///
    /// 設定された host/port の組が有効なソケットアドレスでない場合にエラーを返します。
    pub fn listen_addr(&self) -> Result<SocketAddr, ConfigError> {
        format!("{}:{}", self.app_host, self.app_port)
            .parse()
            .map_err(|source| ConfigError::InvalidListenAddress {
                host: self.app_host.clone(),
                port: self.app_port,
                source,
            })
    }
}
