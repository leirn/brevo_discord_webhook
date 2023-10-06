pub mod brevo_events;
pub mod ip;
pub mod webhook;

use log::info;

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
    ip::register_routes(cfg);
    webhook::register_routes(cfg);

    info!("Routes loaded");
}
