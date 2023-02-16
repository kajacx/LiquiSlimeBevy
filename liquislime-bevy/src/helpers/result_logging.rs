use std::fmt::Debug;

use bevy::prelude::warn;

pub trait ResultLogger<R, E> {
    fn log_err(self);

    fn log_err_or(self, or: R) -> R;

    fn log_err_or_else(self, or_else: impl FnOnce() -> R) -> R;
}

impl<R, E> ResultLogger<R, E> for Result<R, E>
where
    E: Debug,
{
    fn log_err(self) {
        if let Err(err) = self {
            warn!("Logging result error: {:?}", err)
        }
    }

    fn log_err_or(self, or: R) -> R {
        self.log_err_or_else(move || or)
    }

    fn log_err_or_else(self, or_else: impl FnOnce() -> R) -> R {
        match self {
            Ok(value) => value,
            Err(err) => {
                warn!("Logging result error: {:?}", err);
                or_else()
            }
        }
    }
}
