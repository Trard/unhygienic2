//! A dead simple macro to erase macro hygiene.
//! This stringifies the input, then reparses it in the caller's context.
//! First [unhygienic](https://github.com/Lymia/unhygienic) based on [proc_macro_hack](https://github.com/dtolnay/proc-macro-hack).
//! Since 1.45 Rust have native support [proc_macro]. Therefore, proc_macro_hack isn't neeeded.
//! 
//! # Example
//! 
//! ```
//! use unhygienic2::unhygienic;
//!
//! macro_rules! declare {
//!     ($exp:expr) => {
//!         unhygienic! {
//!             fn func() -> i32 {
//!                 let a = 5;
//!
//!                 $exp
//!             }
//!         }
//!     }
//! }
//!
//! declare!({ a + 1 });
//!
//! fn main() {
//!     assert_eq!(func(), 6);
//! }
//! ```
use proc_macro::TokenStream;

#[proc_macro]
/// Macro to erase macro hygiene.
///  # Example
/// ```
/// use unhygienic2::unhygienic;
///
/// macro_rules! declare {
///     ($exp:expr) => {
///         unhygienic! {
///             fn func() -> i32 {
///                 let a = 5;
///
///                 $exp
///             }
///         }
///     }
/// }
///
/// declare!({ a + 1 });
///
/// fn main() {
///     assert_eq!(func(), 6);
/// }
/// ```
pub fn unhygienic(tokens: TokenStream) -> TokenStream {
    tokens.to_string().parse().unwrap()
}