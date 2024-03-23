use super::*;

pub struct Settings {
    amount: SlimeAmount,
}

impl FromSettings for Settings {
    fn from_settings(settings: protocol::SettingValues) -> Self {
        let amount = if let protocol::SettingValue::SlimeAmount(amount) = settings
            .iter()
            .find(|setting| setting.0 == "amount")
            .unwrap()
            .1
        {
            amount
        } else {
            panic!()
        };

        Self {
            amount: SlimeAmount::from_protocol(amount),
        }
    }
}

pub struct LiquislimeUnit {
    amount: SlimeAmount,
}

impl LiquislimePlugin for LiquislimeUnit {
    type Settings = Settings;

    fn new(settings: Self::Settings) -> Self {
        Self {
            amount: settings.amount,
        }
    }

    fn update(&mut self, time_elapsed: TimeInterval) {
        if let Some(position) = Mouse::is_pressed_in_bounds_at() {
            position
                .to_tile_position()
                .add_own_slime_amount(self.amount * time_elapsed.to_seconds());
        }
    }
}
