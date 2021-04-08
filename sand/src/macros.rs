#[cfg(any(feature = "compiler", feature = "vm"))]
#[macro_export]
#[doc(hidden)]
macro_rules! unwrap_action (
    ($token:expr) => {
        match $token {
            Ok(v) => v,
            Err(e) => return e
        }
    }
);
