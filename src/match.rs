//! This file generates the statistics using all of the events.

use std::collections::HashMap;

use crate::{
    events::{EventKind, MeansOfDeath},
    parser::parse,
};

pub const WORLD_ID: u32 = 1022;

#[derive(Default, Debug)]
pub struct Match<'a> {
    pub total: u32,
    pub players: HashMap<u32, &'a str>,
    pub kills: HashMap<u32, i32>,
    pub kill_by_mean: HashMap<MeansOfDeath, i32>,
}

impl<'a> Match<'a> {
    pub fn add_player(&mut self, id: u32, name: &'a str) {
        self.players.insert(id, name);
    }

    pub fn killed(&mut self, killer: u32) {
        let entry = self.kills.entry(killer).or_default();
        *entry += 1;
    }

    pub fn killed_by_world(&mut self, victim: u32) {
        let entry = self.kills.entry(victim).or_default();
        *entry -= 1;
    }

    pub fn kill_by_mean(&mut self, mean: MeansOfDeath) {
        let entry = self.kill_by_mean.entry(mean).or_default();
        self.total += 1;
        *entry += 1
    }
}

pub fn parse_matches(input: &str) -> Vec<Match> {
    let mut matches = Vec::new();
    let mut current = Match::default();

    for line in input.lines() {
        let event = parse(line);

        if let Some(event) = event {
            match event.kind {
                EventKind::ClientUserInfoChanged(user) => {
                    current.add_player(user.id, user.nickname)
                }
                EventKind::Kill(kill) => {
                    current.kill_by_mean(kill.mean);

                    if kill.killer == WORLD_ID {
                        current.killed_by_world(kill.victim)
                    } else {
                        current.killed(kill.killer)
                    }
                }
                EventKind::ShutdownGame => matches.push(std::mem::take(&mut current)),
                _ => (),
            }
        }
    }

    matches
}
