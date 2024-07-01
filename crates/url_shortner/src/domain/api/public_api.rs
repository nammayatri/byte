/*  Copyright 2024-25, Juspay India Pvt Ltd
    This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License
    as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program
    is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
    or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details. You should have received a copy of
    the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/
use actix_web::{
    get,
    web::{Data, Path, Redirect},
};

use crate::{
    common::types::*, domain::action::public_api, environment::AppState, tools::error::AppError,
};

#[get("/{urlShortCode}")]
async fn redirect_to_url(
    app_state: Data<AppState>,
    path: Path<String>,
) -> Result<Redirect, AppError> {
    let url_short_code = UrlShortCode(path.into_inner());
    public_api::redirect_to_url(app_state, url_short_code).await
}

#[get("/healthCheck")]
async fn health_check() -> Result<String, AppError> {
    Ok("Service is Up".to_string())
}
