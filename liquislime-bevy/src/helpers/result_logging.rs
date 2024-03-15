use std::fmt::Debug;

pub trait ResultLogger<R> {
    fn log_err(self, msg: &str);

    fn log_err_or(self, or: R, msg: &str) -> R;

    fn log_err_or_else(self, or_else: impl FnOnce() -> R, msg: &str) -> R;
}

impl<R, E> ResultLogger<R> for Result<R, E>
where
    E: Debug,
{
    fn log_err(self, msg: &str) {
        if let Err(err) = self {
            tracing::warn!("{msg}: {err:?}");
        }
    }

    fn log_err_or(self, or: R, msg: &str) -> R {
        self.log_err_or_else(move || or, msg)
    }

    fn log_err_or_else(self, or_else: impl FnOnce() -> R, msg: &str) -> R {
        match self {
            Ok(value) => value,
            Err(err) => {
                tracing::warn!("{msg}: {err:?}");
                or_else()
            }
        }
    }
}

impl<T> ResultLogger<T> for Option<T> {
    fn log_err(self, msg: &str) {
        if self.is_none() {
            tracing::warn!("{msg}");
        }
    }

    fn log_err_or(self, or: T, msg: &str) -> T {
        self.log_err_or_else(move || or, msg)
    }

    fn log_err_or_else(self, or_else: impl FnOnce() -> T, msg: &str) -> T {
        match self {
            Some(value) => value,
            None => {
                tracing::warn!("{msg}");
                or_else()
            }
        }
    }
}
