mod builder;
mod raw_builder;

use proc_macro::TokenStream;
use raw_builder::BuilderContext;

#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    "fn hello() { println!(\"Hello world!\"); }"
        .parse()
        .unwrap()
}

#[proc_macro_derive(RawBuilder)]
pub fn derive_raw_builder(input: TokenStream) -> TokenStream {
    //println!("{:#?}", input);
    //TokenStream::default()
    BuilderContext::render(input).unwrap().parse().unwrap()
}
