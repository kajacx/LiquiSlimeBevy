#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SS<T>(pub T);

#[cfg(target_arch = "wasm32")]
unsafe impl<T> Send for SS<T> {}
#[cfg(target_arch = "wasm32")]
unsafe impl<T> Sync for SS<T> {}
