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

pub fn embed_builder(event: String, info: &Map<String, Value>) -> Option<DiscordEmbed> {
    // Check tags for transac message. If not present, marketing

    if info.get("tags") != None {
        match event.as_str() {
            "request" => Some(default_embed("Send", info, EmbedColors::DEFAULT)),
            "click" => Some(default_embed("Clicked", info, EmbedColors::SUCCESS)),
            "deffered" => Some(default_embed("Deffered", info, EmbedColors::DEFAULT)),
            "complaint" => Some(default_embed("Complaint", info, EmbedColors::WARN)),
            "delivered" => Some(default_embed("Delivered", info, EmbedColors::SUCCESS)),
            "soft_bounce" => Some(default_embed("Soft Bounced", info, EmbedColors::DEFAULT)),
            "hard_bounce" => Some(default_embed("Hard Bounced", info, EmbedColors::DEFAULT)),
            "unique_opened" => Some(default_embed("First opening", info, EmbedColors::SUCCESS)),
            "blocked" => Some(default_embed("Blocked", info, EmbedColors::ERROR)),
            "error" => Some(default_embed("Error", info, EmbedColors::ERROR)),
            "unsubscribed" => Some(default_embed("Unsubscribed", info, EmbedColors::WARN)),
            "proxy_open" => Some(default_embed("Proxy open", info, EmbedColors::DEFAULT)),
            _ => Some(default_embed(event.as_str(), info, EmbedColors::DEFAULT)),
        }
    } else {
        match event.as_str() {
            "spam" => Some(default_embed("Marked as Spam", info, EmbedColors::WARN)),
            "opened" => Some(default_embed("Opened", info, EmbedColors::SUCCESS)),
            "click" => Some(default_embed("Clicked", info, EmbedColors::SUCCESS)),
            "hard_bounce" => Some(default_embed("Hard Bounced", info, EmbedColors::DEFAULT)),
            "soft_bounce" => Some(default_embed("Soft Bounced", info, EmbedColors::DEFAULT)),
            "delivered" => Some(default_embed("Delivered", info, EmbedColors::DEFAULT)),
            "unsubscribed" => Some(default_embed("Unsubscribed", info, EmbedColors::WARN)),
            "contact_deleted" => Some(default_embed("Contact deleted", info, EmbedColors::WARN)),
            "contact_updated" => Some(default_embed("Contact updated", info, EmbedColors::DEFAULT)),
            "list_addition" => Some(default_embed(
                "Contact added to list",
                info,
                EmbedColors::SUCCESS,
            )),
            _ => Some(default_embed(event.as_str(), info, EmbedColors::DEFAULT)),
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
