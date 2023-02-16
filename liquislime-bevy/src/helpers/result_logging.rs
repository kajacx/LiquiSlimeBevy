use std::fmt::Debug;

use bevy::prelude::warn;

pub trait ResultLogger<R> {
    fn log_err(self);

    fn log_err_or(self, or: R) -> R;

    fn log_err_or_else(self, or_else: impl FnOnce() -> R) -> R;
}

impl<R, E> ResultLogger<R> for Result<R, E>
where
    E: Debug,
{
    fn log_err(self) {
        if let Err(err) = self {
            print_err(err);
        }
    }

    fn log_err_or(self, or: R) -> R {
        self.log_err_or_else(move || or)
    }

    fn log_err_or_else(self, or_else: impl FnOnce() -> R) -> R {
        match self {
            Ok(value) => value,
            Err(err) => {
                print_err(err);
                or_else()
            }
        }
    }
}

fn print_err(error: impl Debug) {
    warn!("Logging result error: {:?}", error);
}

impl<T> ResultLogger<T> for Option<T> {
    fn log_err(self) {
        if self.is_none() {
            print_none();
        }
    }

    fn log_err_or(self, or: T) -> T {
        self.log_err_or_else(move || or)
    }

    fn log_err_or_else(self, or_else: impl FnOnce() -> T) -> T {
        match self {
            Some(value) => value,
            None => {
                print_none();
                or_else()
            }
        }
    }
}

fn print_none() {
    warn!("Option was none");
}
