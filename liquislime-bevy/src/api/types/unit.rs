use crate::components::UnitId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ApiUnit(pub UnitId);
