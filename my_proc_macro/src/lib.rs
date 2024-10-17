extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}


// custom attribute macro

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path = attr.to_string();
    let func = item.to_string();

    let expanded = quote! {
        fn #func {
            println!("Routing to {}", #path);
        }
    };

    TokenStream::from(expanded)
}


// function like macro

#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let expanded = quote! {
        fn generated_function() {
            println!("Generated function called with input: {}", #input);
        }
    };

    TokenStream::from(expanded)
}