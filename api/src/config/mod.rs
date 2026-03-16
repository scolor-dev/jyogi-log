pub mod env;

use std::net::SocketAddr;

use crate::error::ConfigError;

#[derive(Debug, Clone)]
pub struct Config {
    pub app_host: String,
    pub app_port: u16,
    pub rust_log: String,
}

impl Config {
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
