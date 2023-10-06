use super::brevo_events::embed_builder;
use actix_web::{post, web, web::ServiceConfig, HttpResponse, Responder};
use log::info;
use std::collections::HashMap;
use std::env;

use crate::security::ip_filtering::IPFiltering;

pub fn register_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/webhook").service(webhook).wrap(IPFiltering));

    info!("webhook routes loaded");
}

#[post("/event")]
async fn webhook(info: web::Json<serde_json::Value>) -> impl Responder {
    // Parse info and create discord embed

    log::debug!("Enter webhook : info is {info}");

    let info = info.as_object().unwrap();

    let event = match info.get("event") {
        Some(value) => value.to_string(),
        None => "Not found".to_string(),
    };

    log::debug!("Event type is {event}");

    let embed = embed_builder(event, info);
    let mut result = true;
    if embed.is_some() {
        let embed = embed.unwrap();
        let mut map = HashMap::new();
        map.insert("embeds", vec![embed]);

        let map_str = serde_json::to_string(&map).unwrap();

        log::debug!("{map_str}");

        let url = env::var("DISCORD_WEBHOOK_TOKEN").expect("$DISCORD_WEBHOOK_TOKEN is not set");

        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        let response = client.post(url).json(&map).send().await;
        result = match response {
            Ok(_) => true,
            Err(error) => {
                log::info!("{error}");
                false
            }
        };
    }
    match result {
        true => HttpResponse::Ok().body("Ok"),
        false => HttpResponse::InternalServerError().body("Error while processing requests"),
    }
}
