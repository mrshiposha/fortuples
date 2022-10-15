use proc_macro::TokenStream;
use syn::parse_macro_input;

mod expand;
mod parse;
mod types;

use types::FortuplesInfo;

#[proc_macro]
pub fn fortuples(item: TokenStream) -> TokenStream {
    let info = parse_macro_input!(item as FortuplesInfo);

    match info.expand() {
        Ok(tokens) => tokens,
        Err(e) => e.into_compile_error(),
    }
    .into()
}
