extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn function_to_string(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // parse input function to ItemFn
    let input_fn: ItemFn = parse_macro_input!(input as ItemFn);

    // convert the input function to String
    let function_string: String = format!("{}", input_fn.to_token_stream());

    // create a new function with the same signature of the provided function
    let function_identifier: proc_macro2::Ident = input_fn.sig.ident;
    let function_generics: syn::Generics = input_fn.sig.generics;
    let function_types = input_fn.sig.inputs;

    let output: proc_macro2::TokenStream = quote! {
        pub fn #function_identifier #function_generics(#function_types) -> &'static str {
            #function_string
        }
    };

    output.into()
}