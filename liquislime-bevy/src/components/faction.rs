use bevy::prelude::Component;
use derive_more::{From, Into};

use crate::api::ApiFaction;

#[derive(Clone, Debug, Component, From, Into)]
pub struct FactionComponent(pub ApiFaction);
