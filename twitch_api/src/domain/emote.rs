use serde::Deserialize;

use types::twitch;

#[derive(Deserialize, Debug)]
pub struct Emote {
    id: String,
    name: String,
    scale: Vec<String>,
    theme_mode: Vec<String>,
}

impl Emote {
    pub fn to_twitch_emote(&self, template: &str) -> twitch::Emote {
        twitch::Emote {
            id: self.id.clone(),
            name: self.name.clone(),
            image: self.image(template),
        }
    }

    fn image(&self, template: &str) -> String {
        template
            .replace("{{id}}", &self.id)
            .replace("{{format}}", "default")
            .replace("{{theme_mode}}", self.theme_mode.first().unwrap())
            .replace("{{scale}}", self.scale.last().unwrap())
    }
}