use serde_json::{Map, Value};

use serde::Serialize;

//https://developers.brevo.com/docs/marketing-webhooks

#[derive(Serialize, Debug)]
pub struct DiscordEmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Serialize, Debug)]
pub struct DiscordEmbed {
    pub title: String,
    pub color: u32,
    pub fields: Vec<DiscordEmbedField>,
}

#[derive(Copy, Clone)]
enum EmbedColors {
    SUCCESS = 0x00CC00,
    WARN = 0xFFA500,
    ERROR = 0xCC0000,
    DEFAULT = 0x550088,
}

pub fn embed_builder(event: String, info: &Map<String, Value>) -> DiscordEmbed {
    // Check tags for transac message. If not present, marketing

    if info.get("tags") != None {
        match event.as_str() {
            "request" => default_embed("Send", info, EmbedColors::DEFAULT),
            "click" => default_embed("Clicked", info, EmbedColors::SUCCESS),
            "deffered" => default_embed("Deffered", info, EmbedColors::DEFAULT),
            "complaint" => default_embed("Complaint", info, EmbedColors::WARN),
            "delivered" => default_embed("Delivered", info, EmbedColors::SUCCESS),
            "soft_bounce" => default_embed("Soft Bounced", info, EmbedColors::DEFAULT),
            "hard_bounce" => default_embed("Hard Bounced", info, EmbedColors::DEFAULT),
            "unique_opened" => default_embed("First opening", info, EmbedColors::SUCCESS),
            "blocked" => default_embed("Blocked", info, EmbedColors::ERROR),
            "error" => default_embed("Error", info, EmbedColors::ERROR),
            "unsubscribed" => default_embed("Unsubscribed", info, EmbedColors::WARN),
            "proxy_open" => default_embed("Proxy open", info, EmbedColors::DEFAULT),
            _ => default_embed(event.as_str(), info, EmbedColors::DEFAULT),
        }
    } else {
        match event.as_str() {
            "spam" => default_embed("Marked as Spam", info, EmbedColors::WARN),
            "opened" => default_embed("Opened", info, EmbedColors::SUCCESS),
            "click" => default_embed("Clicked", info, EmbedColors::SUCCESS),
            "hard_bounce" => default_embed("Hard Bounced", info, EmbedColors::DEFAULT),
            "soft_bounce" => default_embed("Soft Bounced", info, EmbedColors::DEFAULT),
            "delivered" => default_embed("Delivered", info, EmbedColors::DEFAULT),
            "unsubscribed" => default_embed("Unsubscribed", info, EmbedColors::WARN),
            "contact_deleted" => default_embed("Contact deleted", info, EmbedColors::WARN),
            "contact_updated" => default_embed("Contact updated", info, EmbedColors::DEFAULT),
            "list_addition" => default_embed("Contact added to list", info, EmbedColors::SUCCESS),
            _ => default_embed(event.as_str(), info, EmbedColors::DEFAULT),
        }
    }
}

fn default_embed(title: &str, info: &Map<String, Value>, color: EmbedColors) -> DiscordEmbed {
    let mut embed = DiscordEmbed {
        title: title.to_string(),
        color: color as u32,
        fields: vec![],
    };

    for (key, value) in info.into_iter() {
        embed.fields.push(DiscordEmbedField {
            name: key.to_string(),
            value: value.to_string(),
            inline: false,
        })
    }

    embed
}
