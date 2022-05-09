/// Profiling macro for feature "puffin"
#[cfg(not(target_arch = "wasm32"))]
macro_rules! profile_function {
    ($($arg: tt)*) => {
        #[cfg(feature = "puffin")]
        puffin::profile_function!($($arg)*);
    };
}
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use profile_function;

/// Profiling macro for feature "puffin"
#[cfg(not(target_arch = "wasm32"))]
macro_rules! profile_scope {
    ($($arg: tt)*) => {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!($($arg)*);
    };
}
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use profile_scope;