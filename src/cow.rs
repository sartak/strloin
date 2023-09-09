#[cfg(feature = "beef")]
pub use beef::lean::Cow;
#[cfg(feature = "beef")]
#[allow(non_snake_case)]
pub fn Borrowed<'a>(val: &'a str) -> beef::lean::Cow<'a, str> {
    Cow::borrowed(val)
}
#[cfg(feature = "beef")]
#[allow(non_snake_case)]
pub fn Owned<'a>(val: String) -> beef::lean::Cow<'a, str> {
    Cow::owned(val)
}

#[cfg(not(feature = "beef"))]
pub use std::borrow::Cow::{self, Borrowed, Owned};
