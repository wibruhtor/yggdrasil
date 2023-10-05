use serde::Deserialize;

use types::twitch;

use crate::domain::Badge;

#[derive(Deserialize, Debug)]
pub struct BadgeSet {
    set_id: String,
    versions: Vec<Badge>,
}

impl BadgeSet {
    pub fn to_twitch_badges(&self) -> Vec<twitch::Badge> {
        self.versions.iter()
            .map(|badge| {
                badge.to_twitch_badge(&self.set_id)
            })
            .collect()
    }
}