/*  Copyright 2024-25, Juspay India Pvt Ltd
    This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License
    as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program
    is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
    or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details. You should have received a copy of
    the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use crate::tools::logger::LoggerConfig;
use serde::Deserialize;
use shared::redis::types::{RedisConnectionPool, RedisSettings};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub workers: u8,
    pub logger_cfg: LoggerConfig,
    pub redis_cfg: RedisSettings,
    pub redis_expiry: u32,
    pub request_timeout: u64,
    pub internal_auth_api_key: String,
    pub short_code_length: u8,
    pub shortened_base_url: String,
    pub max_retries_for_shortening: u8,
    pub log_unprocessible_req_body: Vec<String>,
    pub max_allowed_req_size: usize,
    pub default_fallback_url: String,
    pub expired_short_code_fallback_url_map: HashMap<String, String>,
}

#[derive(Clone)]
pub struct AppState {
    pub port: u16,
    pub workers: u8,
    pub redis_pool: Arc<RedisConnectionPool>,
    pub logger_cfg: LoggerConfig,
    pub redis_expiry: u32,
    pub request_timeout: u64,
    pub internal_auth_api_key: String,
    pub short_code_length: u8,
    pub shortened_base_url: String,
    pub max_retries_for_shortening: u8,
    pub log_unprocessible_req_body: Vec<String>,
    pub max_allowed_req_size: usize,
    pub default_fallback_url: String,
    pub expired_short_code_fallback_url_hashmap: HashMap<String, String>,
}

impl AppState {
    pub async fn new(app_config: AppConfig) -> AppState {
        let persistent_redis = Arc::new(
            RedisConnectionPool::new(app_config.redis_cfg, None)
                .await
                .expect("Failed to create Redis connection pool"),
        );

        AppState {
            port: app_config.port,
            workers: app_config.workers,
            logger_cfg: app_config.logger_cfg,
            redis_pool: persistent_redis,
            redis_expiry: app_config.redis_expiry,
            request_timeout: app_config.request_timeout,
            internal_auth_api_key: app_config.internal_auth_api_key,
            short_code_length: app_config.short_code_length,
            shortened_base_url: app_config.shortened_base_url,
            max_retries_for_shortening: app_config.max_retries_for_shortening,
            log_unprocessible_req_body: app_config.log_unprocessible_req_body,
            max_allowed_req_size: app_config.max_allowed_req_size,
            default_fallback_url: app_config.default_fallback_url,
            expired_short_code_fallback_url_hashmap: app_config.expired_short_code_fallback_url_map,
        }
    }
}
