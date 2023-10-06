use actix_web::{post, web, web::ServiceConfig, HttpResponse, Responder};
use log::info;
use serde::Serialize;
use std::env;

use std::collections::HashMap;

use crate::security::ip_filtering::IPFiltering;

pub fn register_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/webhook").service(webhook).wrap(IPFiltering));

    info!("webhook routes loaded");
}

#[derive(Serialize, Debug)]
struct DiscordEmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Serialize, Debug)]
struct DiscordEmbed {
    pub title: String,
    pub description: String,
    pub fields: Vec<DiscordEmbedField>,
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

    let mut embed = DiscordEmbed {
        title: event,
        description: "description".to_string(),
        fields: vec![],
    };

    for (key, value) in info.into_iter() {
        embed.fields.push(DiscordEmbedField {
            name: key.to_string(),
            value: value.to_string(),
            inline: false,
        })
    }

    let mut map = HashMap::new();
    map.insert("embeds", vec![embed]);

    let map_str = serde_json::to_string(&map).unwrap();

    log::debug!("{map_str}");

    let url = env::var("DISCORD_WEBHOOK_TOKEN").expect("$DISCORD_WEBHOOK_TOKEN is not set");

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let result = client.post(url).json(&map).send().await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Ok"),
        Err(error) => {
            log::info!("{error}");
            HttpResponse::InternalServerError().body("Error while processing requests")
        }
    }
}
