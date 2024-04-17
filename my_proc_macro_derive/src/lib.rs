extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn};

// ------------ 派生宏
#[proc_macro_derive(StructInfo)]
pub fn struct_info_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {
        impl StructInfo for #name {
            fn i_name(&self) -> String {
               return stringify!(#name).to_string();
            }
        }
    };
    TokenStream::from(expanded)
}

// ------------ 属性宏宏
#[proc_macro_attribute]
pub fn aop(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = input.sig.ident;
    let block = input.block;
    let output = quote! {
        fn #name() {
            println!("Before the fn");
            #block
            println!("After the fn");
        }
    };
    output.into()
}

// ------------ 函数宏
#[proc_macro]
pub fn add(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let a = iter.next().unwrap();
    let _ = iter.next().unwrap(); // ,号
    let b = iter.next().unwrap();

    let result = format!("{} + {}", a, b);
    result.parse().unwrap()
}
