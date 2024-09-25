use std::collections::HashMap;

/*  Copyright 2024-25, Juspay India Pvt Ltd
    This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License
    as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program
    is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
    or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details. You should have received a copy of
    the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/
use actix_web::{
    get,
    web::{Data, Path, Query, Redirect},
    HttpRequest,
};

use crate::{
    common::types::*,
    domain::action::public_api,
    environment::AppState,
    tools::{error::AppError, validator},
};

#[get("/{urlShortCode}")]
async fn redirect_to_url(
    app_state: Data<AppState>,
    path: Path<String>,
) -> Result<Redirect, AppError> {
    let url_short_code = UrlShortCode(path.into_inner());
    public_api::redirect_to_url(app_state, url_short_code, None).await
}

#[get("/")]
async fn redirect_to_url_via_query_params(
    app_state: Data<AppState>,
    req: HttpRequest,
) -> Result<Redirect, AppError> {
    let query_params =
        Query::<HashMap<String, String>>::from_query(req.query_string()).map_err(|_| {
            AppError::InternalError("Unable to deserialize query params string".to_string())
        })?;
    let (url_category, url_short_code) =
        query_params
            .0
            .into_iter()
            .next()
            .ok_or(AppError::InvalidRequest(
                "No query params found".to_string(),
            ))?;

    validator::is_valid_url_category(url_category.clone(), app_state.clone())?;

    public_api::redirect_to_url(app_state, UrlShortCode(url_short_code), Some(url_category)).await
}

#[get("/healthCheck")]
async fn health_check() -> Result<String, AppError> {
    Ok("Service is Up".to_string())
}
