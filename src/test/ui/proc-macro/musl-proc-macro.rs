// Test proc-macro crate can be built without addtional RUSTFLAGS
// on musl target

// build-pass
// only-musl
#![crate_type = "proc-macro"]

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(Foo)]
pub fn derive_foo(input: TokenStream) -> TokenStream {
    input
}
