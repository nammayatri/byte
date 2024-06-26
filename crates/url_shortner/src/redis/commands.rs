/*  Copyright 2024-25, Juspay India Pvt Ltd
    This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License
    as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program
    is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
    or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details. You should have received a copy of
    the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/
use crate::{common::types::*, redis::keys::*, tools::error::AppError};
use reqwest::Url;
use shared::redis::types::RedisConnectionPool;

pub async fn set_base_url_for_short_code(
    base_url: &Url,
    url_short_code: UrlShortCode,
    persistent_redis_pool: &RedisConnectionPool,
    redis_expiry: u32,
) -> Result<bool, AppError> {
    let is_key_set = persistent_redis_pool
        .setnx_with_expiry(
            &url_short_code_key(url_short_code),
            base_url.as_str(),
            redis_expiry.into(),
        )
        .await
        .map_err(|err| AppError::RedisError(err.to_string()))?;

    Ok(is_key_set)
}

pub async fn get_base_url_by_short_code(
    url_short_code: UrlShortCode,
    persistent_redis_pool: &RedisConnectionPool,
) -> Result<Option<Url>, AppError> {
    let base_url = persistent_redis_pool
        .get_key_as_str(&url_short_code_key(url_short_code))
        .await
        .map_err(|err| AppError::RedisError(err.to_string()))?;

    match base_url {
        Some(base_url) => Ok(Some(Url::parse(&base_url).map_err(|error| {
            AppError::InternalError(format!("URL parsing failed: {}", error))
        })?)),
        None => Ok(None),
    }
}
