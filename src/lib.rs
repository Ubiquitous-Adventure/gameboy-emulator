extern crate proc_macro;
use proc_macro::TokenStream;

mod bitstring_matching;

/// This macros expands 8-bit patterns to all the possible u8 values that should be matched.
/// Example:
/// ```
/// # #[macro_use] extern crate gameboy_emulator;
/// # for byte in 0..=u8::MAX {
/// match byte /* u8 */ {
///     bits!(00001111) => assert_eq!(byte, 15),
///     bits!(_____101) => assert_eq!(byte % 8, 5),
///     bits!(00__1011) => assert_eq!(byte & 0b11001111, 0b00001011),
///     _ => {},
/// }
/// # }
///
#[proc_macro]
pub fn bits(token_stream: TokenStream) -> TokenStream {
    bitstring_matching::generate_all_bitstrings(token_stream)
}
