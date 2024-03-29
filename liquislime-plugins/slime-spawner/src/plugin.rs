use super::*;

#[derive(serde::Deserialize)]
pub struct UnitSettings {
    amount: SlimeAmount,
}

pub struct LiquislimeUnit {
    settings: UnitSettings,
}

impl LiquislimePlugin for LiquislimeUnit {
    type Settings = UnitSettings;

    fn new(settings: Self::Settings) -> Self {
        Self { settings }
    }

    fn change_settings(&mut self, settings: Self::Settings) {
        self.settings = settings
    }

    fn update(&mut self, _time_elapsed: TimeInterval) {
        TilePosition::own_position().set_own_slime_amount_at_least(self.settings.amount);
    }
}
