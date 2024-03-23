use bevy::prelude::Component;
use derive_more::{From, Into};

use crate::api::Faction;

#[derive(Clone, Debug, Component, From, Into)]
pub struct FactionComponent(pub Faction);
