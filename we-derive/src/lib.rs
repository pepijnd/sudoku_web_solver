use std::iter::repeat;

use html_parser::{Dom, Node};
use proc_macro2::{Ident, LineColumn, TokenStream, TokenTree};

use quote::{format_ident, quote, quote_spanned};

fn parse_dom(input: &[TokenTree]) -> html_parser::Result<Dom> {
    let mut html = String::new();
    let mut end: Option<LineColumn> = None;
    let mut offset = None;
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

fn gen_element(ident: Ident, dom: Dom) -> TokenStream {
    let mut refs: Vec<(String, String)> = Vec::new();
    if dom.children.len() != 1 {
        return quote! {
            compile_error!("DOM should contain 1 root")
        }
    }
    let elements = walk_dom(&dom.children, &mut refs);
    let root = elements.first().unwrap();
    let ref_name = refs.iter().map(|(s, _)| format_ident!("{}", s));
    let ref_member = ref_name.clone();
    let ref_value = refs.iter().map(|(s, _)| format_ident!("_m_{}", s));
    let ref_decl = ref_value.clone();
    let token = quote!(
        struct #ident {
            root: Element,
            #( #ref_name: Element, )*
        }

        impl #ident {
            fn new() -> Self {
                #( let mut #ref_decl = None; )*
                let _e_root = {#root};
                Self {
                    root: _e_root,
                    #( #ref_member: #ref_value.unwrap(),)*
                }
            }
        }
    );
    token
}

#[proc_macro]
pub fn we_element(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input).into_iter().collect::<Vec<_>>();
    if let [ident_tt, sep_tt, dom_tt @ ..] = input.as_slice() {
        let ident = if let TokenTree::Ident(ident) = ident_tt {
            ident.clone()
        } else {
            return quote_spanned! {
                ident_tt.span() => compile_error!("expected Identifier");
            }
            .into();
        };
        if let TokenTree::Punct(punct) = sep_tt {
            if punct.as_char() != ',' {
                return quote_spanned! {
                    sep_tt.span() => compile_error!("expected ','");
                }
                .into();
            }
        } else {
            return quote_spanned! {
                sep_tt.span() => compile_error!("expected ','");
            }
            .into();
        }

        if dom_tt.is_empty() {
            return quote! {
                compile_error!("expected 2 arguments");
            }
            .into();
        }

        let dom_start = dom_tt.first().unwrap().span();
        let dom_end = dom_tt.last().unwrap().span();
        let dom_span = dom_start.join(dom_end).unwrap();

        match parse_dom(dom_tt) {
            Ok(dom) => {
                let tokens = gen_element(ident, dom);
                println!("{}", tokens.to_string());
                tokens.into()
            }
            Err(e) => {
                let e = format!("Error parsing html: {}", e);
                let tokens = quote_spanned! {
                    dom_span => compile_error!(#e);
                };
                tokens.into()
            }
        }
    } else {
        let tokens = quote! {
            compile_error!("expected 2 arguments");
        };
        tokens.into()
    }
}

#[cfg(test)]
mod tests {}
