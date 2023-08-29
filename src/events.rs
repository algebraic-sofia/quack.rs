//! This is the specification of the events that appear on the quake log. Each one of the events are
//! splitted into multiple structs for better manipulation than putting inside a single event enum.

use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(FromPrimitive, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MeansOfDeath {
    ModUnknown = 0,
    ModShotgun,
    ModGauntlet,
    ModMachinegun,
    ModGrenade,
    ModGrenadeSplash,
    ModRocket,
    ModRocketSplash,
    ModPlasma,
    ModPlasmaSplash,
    ModRailgun,
    ModLightning,
    ModBfg,
    ModBfgSplash,
    ModWater,
    ModSlime,
    ModLava,
    ModCrush,
    ModTelefrag,
    ModFalling,
    ModSuicide,
    ModTargetLaser,
    ModTriggerHurt,
    ModGrapple,
}

impl Serialize for MeansOfDeath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl ToString for MeansOfDeath {
    fn to_string(&self) -> String {
        match self {
            MeansOfDeath::ModUnknown => "MOD_UNKNOWN".to_string(),
            MeansOfDeath::ModShotgun => "MOD_SHOTGUN".to_string(),
            MeansOfDeath::ModGauntlet => "MOD_GAUNTLET".to_string(),
            MeansOfDeath::ModMachinegun => "MOD_MACHINEGUN".to_string(),
            MeansOfDeath::ModGrenade => "MOD_GRENADE".to_string(),
            MeansOfDeath::ModGrenadeSplash => "MOD_GRENADE_SPLASH".to_string(),
            MeansOfDeath::ModRocket => "MOD_ROCKET".to_string(),
            MeansOfDeath::ModRocketSplash => "MOD_ROCKET_SPLASH".to_string(),
            MeansOfDeath::ModPlasma => "MOD_PLASMA".to_string(),
            MeansOfDeath::ModPlasmaSplash => "MOD_PLASMA_SPLASH".to_string(),
            MeansOfDeath::ModRailgun => "MOD_RAILGUN".to_string(),
            MeansOfDeath::ModLightning => "MOD_LIGHTNING".to_string(),
            MeansOfDeath::ModBfg => "MOD_BFG".to_string(),
            MeansOfDeath::ModBfgSplash => "MOD_BFG_SPLASH".to_string(),
            MeansOfDeath::ModWater => "MOD_WATER".to_string(),
            MeansOfDeath::ModSlime => "MOD_SLIME".to_string(),
            MeansOfDeath::ModLava => "MOD_LAVA".to_string(),
            MeansOfDeath::ModCrush => "MOD_CRUSH".to_string(),
            MeansOfDeath::ModTelefrag => "MOD_TELEFRAG".to_string(),
            MeansOfDeath::ModFalling => "MOD_FALLING".to_string(),
            MeansOfDeath::ModSuicide => "MOD_SUICIDE".to_string(),
            MeansOfDeath::ModTargetLaser => "MOD_TARGET_LASER".to_string(),
            MeansOfDeath::ModTriggerHurt => "MOD_TRIGGER_HURT".to_string(),
            MeansOfDeath::ModGrapple => "MOD_GRAPPLE".to_string(),
        }
    }
}

impl Default for MeansOfDeath {
    fn default() -> Self {
        Self::ModMachinegun
    }
}

#[derive(Debug)]
pub struct ClientUserInfoChanged<'a> {
    pub id: u32,
    pub nickname: &'a str,
}

#[derive(Debug)]
pub struct Kill {
    pub victim: u32,
    pub killer: u32,
    pub mean: MeansOfDeath,
}

#[derive(Debug)]
pub enum EventKind<'a> {
    InitGame,
    ClientUserInfoChanged(ClientUserInfoChanged<'a>),
    Kill(Kill),
    ShutdownGame,
    Irrelevant,
}

#[derive(Debug)]
pub struct Event<'a> {
    pub date: (u32, u32),
    pub kind: EventKind<'a>,
}
