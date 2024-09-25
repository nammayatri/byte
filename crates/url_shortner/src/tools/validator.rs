/*  Copyright 2024-25, Juspay India Pvt Ltd
    This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License
    as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program
    is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
    or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details. You should have received a copy of
    the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use actix_web::web::Data;

use crate::environment::AppState;

use super::error::AppError;

pub fn is_valid_url_category(
    url_category: String,
    app_state: Data<AppState>,
) -> Result<(), AppError> {
    if !app_state
        .expired_short_code_fallback_url_hashmap
        .contains_key(&url_category)
    {
        Err(AppError::InvalidRequest(format!(
            "Invalid url_category: {}",
            url_category
        )))
    } else {
        Ok(())
    }
}
