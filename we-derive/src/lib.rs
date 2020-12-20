use std::iter::repeat;

use html_parser::{Dom, Node};
use proc_macro2::{Ident, LineColumn, TokenStream, TokenTree};

use quote::{format_ident, quote, quote_spanned};
use syn::{parse::Parser, parse_macro_input, DeriveInput};

struct DomParsed {
    fields: Vec<syn::Field>,
    build: TokenStream,
    errors: TokenStream,
}

fn parse_args(args: TokenStream) -> DomParsed {
    let args: Vec<TokenTree> = args.into_iter().collect();
    let dom = parse_dom(&args);
    match dom {
        Ok(dom) => gen_element(dom),
        Err(e) => {
            let e = e.to_string();
            let dom_start = args.first().unwrap().span();
            let dom_end = args.last().unwrap().span();
            let dom_span = dom_start.join(dom_end).unwrap();
            DomParsed {
                fields: Default::default(),
                build: quote! {},
                errors: quote_spanned! {
                    dom_span => compile_error!(#e)
                },
            }
        }
    }
}

fn parse_dom(input: &[TokenTree]) -> html_parser::Result<Dom> {
    let mut html = String::new();
    let mut end: Option<LineColumn> = None;
    let mut offset: Option<usize> = None;
    for token in input {
        let span = token.span().start();
        if offset.is_none() {
            offset = Some(span.column);
        }
        if let Some(end) = end {
            if span.line > end.line {
                html.push('\n');
                html.push_str(
                    &repeat(' ')
                        .take(span.column.saturating_sub(offset.unwrap()))
                        .collect::<String>(),
                )
            } else {
                html.push_str(
                    &repeat(' ')
                        .take(span.column - end.column)
                        .collect::<String>(),
                )
            }
        } else {
            html.push_str(
                &repeat(' ')
                    .take(span.column.saturating_sub(offset.unwrap()))
                    .collect::<String>(),
            )
        }
        end = Some(token.span().end());
        html.push_str(&token.to_string());
    }
    Dom::parse(&html)
}

fn walk_dom(dom: &[Node], refs: &mut Vec<(String, String)>) -> Vec<TokenStream> {
    let mut elements = Vec::new();
    for node in dom {
        if let Node::Element(element) = node {
            let has_member = element.attributes.iter().find_map(|(k, v)| {
                if k == "member" {
                    if let Some(v) = v {
                        Some(v)
                    } else {
                        None
                    }
                } else {
                    None
                }
            });
            if let Some(member) = has_member {
                refs.push((member.to_owned(), element.name.to_owned()))
            }
            let children = walk_dom(&element.children, refs);
            let classes = &element.classes;
            let key = &element
                .attributes
                .iter()
                .filter(|&(n, _)| n != "member")
                .map(|s| s.0.clone())
                .collect::<Vec<_>>();
            let value = &element
                .attributes
                .iter()
                .filter(|&(n, _)| n != "member")
                .map(|s| -> String { s.1.clone().unwrap_or_else(|| "".to_owned()) })
                .collect::<Vec<_>>();
            let has_text = element.children.iter().find_map(|n| {
                if let Node::Text(t) = n {
                    Some(t)
                } else {
                    None
                }
            });
            let text = has_text.iter();
            let name = &element.name;
            let ident = format_ident!("_e_{}", name);
            let member_ident = has_member.iter().map(|s| format_ident!("_m_{}", s));
            let token = quote!(
                let mut #ident = Element::new(#name);
                #( #ident.append(&{#children});  )*
                #( #ident.add_class(#classes); )*
                #( #ident.set_attr(#key, #value); )*
                #( #ident.set_text(#text); )*
                #( #member_ident = Some(#ident.clone()); )*
                #ident
            );
            elements.push(token);
        }
    }
    elements
}

fn gen_element(dom: Dom) -> DomParsed {
    let mut refs: Vec<(String, String)> = Vec::new();
    let mut errors = quote! {};
    if dom.children.len() != 1 {
        errors = quote! {
            #errors
            compile_error!("DOM should contain 1 root")
        };
    }
    let elements = walk_dom(&dom.children, &mut refs);
    let root = elements.first().unwrap();
    let ref_name: Vec<Ident> = refs.iter().map(|(s, _)| format_ident!("{}", s)).collect();
    let ref_value: Vec<Ident> = refs
        .iter()
        .map(|(s, _)| format_ident!("_m_{}", s))
        .collect();
    let token = quote!(
        fn build() -> Self {
            #( let mut #ref_value = None; )*
            let _e_root = {#root};
            Self {
                root: _e_root,
                #( #ref_name: #ref_value.unwrap(),)*
            }
        }
    );
    DomParsed {
        fields: refs
            .iter()
            .map(|(s, _)| {
                let ident = format_ident!("{}", s);
                syn::Field::parse_named
                    .parse2(quote! { pub #ident: Element })
                    .unwrap()
            })
            .collect(),
        build: token,
        errors,
    }
}

#[proc_macro_attribute]
pub fn we_element(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let ident = ast.ident.clone();
    let DomParsed {
        fields,
        build,
        errors,
    } = parse_args(args.into());
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            if let syn::Fields::Named(s_fields) = &mut struct_data.fields {
                s_fields.named.push(syn::Field::parse_named
                    .parse2(quote! { pub root: Element })
                    .unwrap());
                for field in fields.iter() {
                    s_fields.named.push(field.clone())
                }
            }
            quote! {
                #ast

                impl WebElement for #ident {
                    #build
                }
            }
        }
        _ => {
            quote! {
                #errors
                compile_error!("`we_element` is only valid on structs")
            }
        }
    }.into()
}

#[cfg(test)]
mod tests {}
