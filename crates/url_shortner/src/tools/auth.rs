/*  Copyright 2022-23, Juspay India Pvt Ltd
    This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License
    as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program
    is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
    or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details. You should have received a copy of
    the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use actix_web::HttpRequest;

use super::error::AppError;

pub fn authenticate(internal_api_key: &str, req: HttpRequest) -> Result<(), AppError> {
    let token = req
        .headers()
        .get("x-api-key")
        .and_then(|header_val| header_val.to_str().ok())
        .map(|dm_str| dm_str.to_string())
        .ok_or(AppError::InvalidRequest(
            "x-api-key (Header) is missing".to_string(),
        ))?;

    if token != internal_api_key {
        Err(AppError::AuthFailed(format!(
            "Invalid x-api-key: {}",
            token
        )))
    } else {
        Ok(())
    }
}
