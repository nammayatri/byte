/*  Copyright 2024-25, Juspay India Pvt Ltd
    This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License
    as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program
    is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
    or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details. You should have received a copy of
    the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/
use actix_web::{
    post,
    web::{Data, Json},
    HttpRequest,
};
use crate::{
    domain::{
        action::internal::crud, 
        types::internal::crud::*
    }, 
    environment::AppState, tools::{auth::authenticate, error::AppError}
};

#[post("/internal/generateShortUrl")]
async fn generate_url(
    data: Data<AppState>,
    req: HttpRequest,
    param_obj: Json<GenerateShortUrlRequest>,
) -> Result<Json<GenerateShortUrlResponse>, AppError> {

    authenticate(&data.internal_auth_api_key, req)?;

    let req_body = param_obj.into_inner();
    
    Ok(Json(crud::generate_url(data, req_body).await?))
}