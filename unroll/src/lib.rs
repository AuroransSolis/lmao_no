extern crate proc_macro;
#[macro_use] extern crate syn;
#[macro_use] extern crate quote;
#[macro use] extern crate sll;

use sll::SLL;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, ExprRange, ExprForLoop, Expr, Pat, Block, ExprWhile};

#[proc_macro]
pub fn unroll_for(input: TokenStream) -> TokenStream {

}

#[proc_macro]
pub fn unroll_while(input: TokenStream) -> TokenStream {

}