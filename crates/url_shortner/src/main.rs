/*  Copyright 2024-25, Juspay India Pvt Ltd
    This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License
    as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program
    is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
    or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details. You should have received a copy of
    the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use actix_web::{web, App, HttpServer};
use tracing::error;
use tracing_actix_web::TracingLogger;

use std::net::Ipv4Addr;
use url_shortner::{
    domain::api,
    environment::{AppConfig, AppState},
    middleware::{
        CheckContentLength, DomainRootSpanBuilder, IncomingRequestMetrics, LogIncomingRequestBody,
        RequestTimeout,
    },
    tools::{error::AppError, logger::setup_tracing, prometheus::prometheus_metrics},
};

fn read_dhall_config(config_path: &str) -> Result<AppConfig, String> {
    let config = serde_dhall::from_file(config_path).parse::<AppConfig>();
    match config {
        Ok(config) => Ok(config),
        Err(e) => Err(format!("Dhall config parsing failed: {e}")),
    }
}

#[actix_web::main]
async fn start_server() -> std::io::Result<()> {
    let dhall_config_path = "./dhall-configs/dev/url-shortner.dhall".to_string();
    let app_config: AppConfig = read_dhall_config(&dhall_config_path).unwrap_or_else(|err| {
        println!("{:?}", err);
        std::process::exit(1);
    });

    println!("Starting server appConfig: {:?}", app_config.clone());

    let _guard = setup_tracing(app_config.logger_cfg);

    std::panic::set_hook(Box::new(|panic_info| {
        println!("Panic occurred: {:?}", panic_info);
        error!("Panic occurred: {:?}", panic_info);
    }));

    let port = app_config.port;
    let workers = app_config.workers;
    let max_allowed_req_size = app_config.max_allowed_req_size;

    let app_state = AppState::new(app_config).await;
    println!("App state created");

    let data = web::Data::new(app_state);

    let prometheus = prometheus_metrics();

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .app_data(
                web::JsonConfig::default()
                    .limit(max_allowed_req_size)
                    .error_handler(|err, _| AppError::UnprocessibleRequest(err.to_string()).into()),
            )
            .app_data(web::PayloadConfig::default().limit(max_allowed_req_size))
            .wrap(RequestTimeout)
            .wrap(CheckContentLength)
            .wrap(LogIncomingRequestBody)
            .wrap(IncomingRequestMetrics)
            .wrap(TracingLogger::<DomainRootSpanBuilder>::new())
            .wrap(prometheus.clone())
            .configure(api::handler)
    })
    .workers(workers.into())
    .bind((Ipv4Addr::UNSPECIFIED, port))?
    .run()
    .await?;

    Ok(())
}

fn main() {
    start_server().expect("Failed to start server");
}
