use std::collections::HashMap;

use events::MeansOfDeath;
use r#match::{parse_matches, Match};

use serde::Serialize;

pub mod events;
pub mod r#match;
pub mod parser;

#[derive(Serialize)]
pub struct MatchKills<'a> {
    total_kills: u32,
    players: Vec<&'a str>,
    kills: HashMap<&'a str, i32>,
}

#[derive(Serialize)]
pub struct MatchByMeans {
    kills_by_means: HashMap<MeansOfDeath, i32>,
}

impl<'a> From<Match<'a>> for (MatchKills<'a>, MatchByMeans) {
    fn from(value: Match<'a>) -> Self {
        let total_kills = value.total;

        let kills = value
            .kills
            .into_iter()
            .map(|(id, kills)| (*value.players.get(&id).unwrap(), kills))
            .collect();

        let match_kills = MatchKills {
            total_kills,
            kills,
            players: value.players.into_values().collect(),
        };

        (
            match_kills,
            MatchByMeans {
                kills_by_means: value.kill_by_mean,
            },
        )
    }
}

#[derive(Serialize)]
pub struct Report<'a> {
    pub match_kills: Vec<MatchKills<'a>>,
    pub match_by_means: Vec<MatchByMeans>,
}

pub fn generate_report(str: &str) -> Report<'_> {
    let matches = parse_matches(str);

    let (match_kills, match_by_means) = matches.into_iter().map(|x| x.into()).unzip();

    Report {
        match_kills,
        match_by_means,
    }
}
