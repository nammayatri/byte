/*  Copyright 2024-25, Juspay India Pvt Ltd
This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License
as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program
is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details. You should have received a copy of
the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/
use actix_web::web::{Data, Redirect};
use crate::{
    common::types::UrlShortCode, environment::AppState, 
    redis::commands::*, 
    tools::error::AppError
};

pub async fn redirect_to_url(
    app_state: Data<AppState>,
    url_short_code: UrlShortCode,
) -> Result<Redirect, AppError> {
    println!("redirect request to url with short code: {:?}", url_short_code);

    let mb_base_url = get_base_url_by_short_code(&url_short_code, &app_state.redis_pool).await?;

    match mb_base_url {
        Some(base_url) => {
            println!("redirecting to: {}", base_url);
            Ok(Redirect::to(base_url.to_string()))
        },
        None => Err(AppError::
            InvalidRequest(format!("No URL found for short code: {}", url_short_code.0))
        )
    }
}
