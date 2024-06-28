/*  Copyright 2024-25, Juspay India Pvt Ltd
This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License
as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program
is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details. You should have received a copy of
the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use crate::{
    common::{
        types::{TimeStamp, UrlShortCode},
        utils::from_maybe,
    },
    domain::types::internal::crud::*,
    environment::AppState,
    redis::commands::set_base_url_for_short_code,
    tools::error::AppError,
};
use actix_web::web::Data;
use chrono::{Duration, Utc};
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use reqwest::Url;
use shared::redis::types::RedisConnectionPool;
use tracing::*;

pub async fn generate_url(
    app_state: Data<AppState>,
    req: GenerateShortUrlRequest,
) -> Result<GenerateShortUrlResponse, AppError> {
    info!("Generate short url req: {:?}", req);

    let base_url = Url::parse(&req.base_url)
        .map_err(|error| AppError::InvalidRequest(format!("URL parsing failed: {}", error)))?;

    info!("Parsed URL: {:?}", base_url);

    let expiry_seconds: Option<u32> = req
        .expiry_in_hours
        .map(|hours| 3600 * Into::<u32>::into(hours));
    let redis_expiry_in_s = from_maybe(expiry_seconds, app_state.redis_expiry);

    let UrlShortCode(final_short_code) = match req.custom_short_code {
        Some(custom_short_code) => {
            set_custom_code(
                &base_url,
                &custom_short_code,
                redis_expiry_in_s,
                &app_state.redis_pool,
            )
            .await?;
            custom_short_code
        }
        None => {
            let final_short_code = set_base_url(&base_url, redis_expiry_in_s, &app_state).await?;
            final_short_code.ok_or_else(|| {
                AppError::InternalError(format!(
                    "Failed to generate unique short code after {} retries",
                    app_state.max_retries_for_shortening
                ))
            })?
        }
    };

    let url_expiry = TimeStamp(Utc::now() + Duration::seconds(redis_expiry_in_s.into()));
    let short_url = format!("{}/{}", app_state.shortened_base_url, final_short_code);
    info!(
        "Generated short url: {} with expiry ts: {:?}",
        short_url, url_expiry.0
    );

    Ok(GenerateShortUrlResponse {
        short_url,
        url_expiry,
    })
}

async fn set_custom_code(
    base_url: &Url,
    short_code: &UrlShortCode,
    redis_expiry: u32,
    persistent_redis: &RedisConnectionPool,
) -> Result<(), AppError> {
    let is_key_set =
        set_base_url_for_short_code(base_url, short_code.clone(), persistent_redis, redis_expiry)
            .await?;

    if !is_key_set {
        let UrlShortCode(code) = short_code;
        Err(AppError::InvalidRequest(format!(
            "Short code: {code} already exists"
        )))
    } else {
        Ok(())
    }
}

async fn set_base_url(
    base_url: &Url,
    redis_expiry: u32,
    app_state: &Data<AppState>,
) -> Result<Option<UrlShortCode>, AppError> {
    let mut rng = thread_rng();
    let mut retries_rem = app_state.max_retries_for_shortening;
    let mut final_short_code: Option<UrlShortCode> = None;
    let short_code_len = app_state.short_code_length.into();
    while retries_rem > 0 {
        let short_code = UrlShortCode(Alphanumeric.sample_string(&mut rng, short_code_len));
        let is_key_set = set_base_url_for_short_code(
            base_url,
            short_code.clone(),
            &app_state.redis_pool,
            redis_expiry,
        )
        .await?;

        if is_key_set {
            final_short_code = Some(short_code);
            break;
        } else {
            retries_rem -= 1;
        }
    }

    Ok(final_short_code)
}
