/*  Copyright 2022-23, Juspay India Pvt Ltd
    This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License
    as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program
    is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
    or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details. You should have received a copy of
    the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use actix_web::{
    http::{header::ContentType, StatusCode}, HttpResponse, ResponseError
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBody {
    error_message: String,
    pub error_code: String,
}

#[macros::add_error]
pub enum AppError {
    InternalError(String),
    InvalidRequest(String),
    PanicOccured(String),
    RedisError(String),
    AuthFailed(String),
}

impl AppError {
    fn error_message(&self) -> ErrorBody {
        ErrorBody {
            error_message: self.message(),
            error_code: self.code(),
        }
    }

    pub fn message(&self) -> String {
        match self {
            AppError::InternalError(err) => err.to_string(),
            AppError::InvalidRequest(err) => err.to_string(),
            AppError::PanicOccured(reason) => {
                format!("Panic occured : {reason}")
            },
            AppError::RedisError(err) => err.to_string(),
            AppError::AuthFailed(err) => err.to_string(),
        }
    }

    fn code(&self) -> String {
        match self {
            AppError::InternalError(_) => "INTERNAL_ERROR",
            AppError::InvalidRequest(_) => "INVALID_REQUEST",
            AppError::PanicOccured(_) => "PANIC_OCCURED",
            AppError::RedisError(_) => "REDIS_ERROR",
            AppError::AuthFailed(_) => "AUTH_FAILED",
        }
        .to_string()
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(self.error_message())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PanicOccured(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            AppError::RedisError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::AuthFailed(_) => StatusCode::UNAUTHORIZED,
        }
    }
}
