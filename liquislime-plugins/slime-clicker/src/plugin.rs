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

    fn update(&mut self, time_elapsed: TimeInterval) {
        if let Some(position) = Mouse::is_pressed_in_bounds_at() {
            position
                .to_tile_position()
                .add_own_slime_amount(self.settings.amount * time_elapsed.to_seconds());
        }
    }
}
