use std::fmt::Debug;

pub trait ResultLogger<R> {
    fn log_err(self, msg: &str);

    fn log_err_or(self, msg: &str, or: R) -> R;

    fn log_err_or_else(self, msg: &str, or_else: impl FnOnce() -> R) -> R;
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

    fn log_err_or(self, msg: &str, or: R) -> R {
        self.log_err_or_else(msg, move || or)
    }

    fn log_err_or_else(self, msg: &str, or_else: impl FnOnce() -> R) -> R {
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

    fn log_err_or(self, msg: &str, or: T) -> T {
        self.log_err_or_else(msg, move || or)
    }

    fn log_err_or_else(self, msg: &str, or_else: impl FnOnce() -> T) -> T {
        match self {
            Some(value) => value,
            None => {
                tracing::warn!("{msg}");
                or_else()
            }
        }
    }
}
