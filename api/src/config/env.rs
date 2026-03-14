use crate::error::ConfigError;

use super::Config;

/// 環境変数からアプリケーション設定を読み込みます。
///
/// # Errors
///
/// `APP_PORT` が設定されていて、それを `u16` に変換できない場合にエラーを返します。
pub fn load() -> Result<Config, ConfigError> {
    let app_host = std::env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    let app_port = std::env::var("APP_PORT")
        .map_or(Ok(3000), |value| {
            value
                .parse::<u16>()
                .map_err(|source| ConfigError::InvalidPort { value, source })
        })?;

    let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

    Ok(Config {
        app_host,
        app_port,
        rust_log,
    })
}
