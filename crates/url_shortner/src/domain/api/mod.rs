/*  Copyright 2024-25, Juspay India Pvt Ltd
    This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License
    as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program
    is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
    or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details. You should have received a copy of
    the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/
pub mod internal;
pub mod public_api;

use actix_web::web::ServiceConfig;

pub fn handler(config: &mut ServiceConfig) {
    config
        .service(internal::crud::generate_url)
        .service(public_api::health_check)
        .service(public_api::redirect_to_url)
        .service(public_api::redirect_to_url_via_query_params);
}
