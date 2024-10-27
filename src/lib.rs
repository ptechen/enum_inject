extern crate proc_macro;

use std::str::FromStr;
use proc_macro2::{Ident, Span};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use serde::Deserialize;
use syn::{parse_macro_input, ItemEnum, Expr, Variant, Fields, ExprLit, Lit, LitInt};
use syn::token::Eq;

#[proc_macro_derive(EnumInjector, attributes(skip))]
pub fn enum_display_derive(_: TokenStream) -> TokenStream {
    TokenStream::from(quote! {})
}

#[derive(Deserialize)]
struct AttrArgs {
    pub args: Vec<AttrArg>,
    pub derives: Vec<String>,
}

#[derive(Deserialize)]
struct AttrArg {
    /// 前缀
    prefix: String,
    /// 后缀
    suffix: String,
    /// 计算
    compute: Compute,
}

impl AttrArg {
    fn get_index(&self, index: isize) -> isize {
        match &self.compute {
            Compute::Add(v) => index + v,
            Compute::Sub(v) => index - v,
            Compute::Mul(v) => index * v,
        }
    }
}

#[derive(Deserialize)]
enum Compute {
    /// 加
    Add(isize),
    /// 减
    Sub(isize),
    /// 乘
    Mul(isize),
}

#[proc_macro_attribute]
pub fn enum_injector(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_args(attr);
    let mut derives = vec![];
    for derive in args.derives {
        let ident = proc_macro2::TokenStream::from_str(&derive).unwrap();
        derives.push(quote! {#ident});
    }
    let input = parse_macro_input!(item as ItemEnum);
    let pub_str = &input.vis;
    let name = &input.ident;
    let mut enum_fields = vec![];
    for variant in &input.variants {
        enum_fields.push(variant.to_token_stream());
    }
    let mut index = 0;
    for variant in &input.variants {
        let ident = &variant.ident;
        if let Some((_, Expr::Lit(expr))) = &variant.discriminant {
            index = expr.lit.to_token_stream().to_string().parse::<isize>().unwrap();
        };
        let mut flag = true;
        for attr in variant.attrs.iter() {
            if attr.path().is_ident("skip") {
                flag = false;
            }
        }
        if flag {
            for arg in &args.args {
                let index_str = arg.get_index(index).to_string();
                let new_ident = format!("{}{}{}", arg.prefix, ident.to_string(), arg.suffix);
                let new_ident = Ident::new(&new_ident, Span::call_site());
                let expr = Expr::Lit(ExprLit{ attrs: vec![], lit: Lit::from(LitInt::new(index_str.as_str(), Span::call_site())) });
                let new_variant = Variant{
                    attrs: vec![],
                    ident: new_ident,
                    fields: Fields::Unit,
                    discriminant: Some((Eq::default(), expr)),
                };
                enum_fields.push(new_variant.to_token_stream())
            };
        }
        index += 1;
    }
    let token = quote! {
        #(#derives)*
        #pub_str enum #name {
            #(#enum_fields),*
        }
    };
    // eprintln!("{}", token.to_token_stream());
    TokenStream::from(token)
}

fn parse_args(attr: TokenStream) -> AttrArgs {
    serde_json::from_str::<AttrArgs>(attr.to_string().as_str()).unwrap()
}