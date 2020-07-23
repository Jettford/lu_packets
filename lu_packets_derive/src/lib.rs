mod from_variants;
mod game_message;
mod gm_type;
mod variant_tests;

use proc_macro::TokenStream;

#[proc_macro_derive(FromVariants)]
pub fn derive_from_variants(input: TokenStream) -> TokenStream {
	from_variants::derive(input)
}

#[proc_macro_derive(GameMessage, attributes(default))]
pub fn derive_game_message_deserialize(input: TokenStream) -> TokenStream {
	game_message::derive(input)
}

#[proc_macro_derive(GmParam)]
pub fn derive_gm_type(input: TokenStream) -> TokenStream {
	gm_type::derive(input)
}

#[proc_macro_derive(VariantTests, attributes(test_params))]
pub fn derive_varant_tests(input: TokenStream) -> TokenStream {
	variant_tests::derive(input)
}
