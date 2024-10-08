/*  Copyright 2024-25, Juspay India Pvt Ltd
This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License
as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program
is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details. You should have received a copy of
the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/
use crate::{
    common::types::UrlShortCode, environment::AppState, redis::commands::*, tools::error::AppError,
};
use actix_web::web::{Data, Redirect};
use tracing::*;

pub async fn redirect_to_url(
    app_state: Data<AppState>,
    url_short_code: UrlShortCode,
    url_category: Option<String>,
) -> Result<Redirect, AppError> {
    info!(
        "redirect request to url with short code: {:?}",
        url_short_code
    );

    let mb_base_url =
        get_base_url_by_short_code(url_short_code.clone(), &app_state.redis_pool).await?;

    match mb_base_url {
        Some(base_url) => {
            info!("redirecting to: {}", base_url);
            Ok(Redirect::to(base_url.to_string()))
        }
        None => {
            error!("No URL found for short code: {}", url_short_code.0);
            let fallback_url = url_category
                .and_then(|category| {
                    app_state
                        .expired_short_code_fallback_url_hashmap
                        .get(&category)
                        .cloned()
                })
                .unwrap_or(app_state.default_fallback_url.clone());
            Ok(Redirect::to(fallback_url.clone()))
        }
    }
}
