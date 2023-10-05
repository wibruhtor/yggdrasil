use serde::Deserialize;

use types::twitch;

use crate::domain::BadgeSet;

#[derive(Deserialize, Debug)]
pub struct GetBadgesResponse {
    data: Vec<BadgeSet>,
}

impl GetBadgesResponse {
    pub fn to_twitch_badges(&self) -> Vec<twitch::Badge> {
        let mut badges: Vec<twitch::Badge> = Vec::new();

        self.data.iter()
            .for_each(|set| {
                set.to_twitch_badges().iter().for_each(|badge| {
                    badges.push(badge.to_owned());
                })
            });

        badges
    }
}