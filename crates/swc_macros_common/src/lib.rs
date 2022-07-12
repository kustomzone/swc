extern crate proc_macro;

#[cfg(procmacro2_semver_exempt)]
use pmutil::SpanExt;
use pmutil::{q, synom_ext::FromSpan, Quote, SpanExt};
use proc_macro2::Span;
use quote::ToTokens;
use syn::*;

pub mod binder;
pub mod derive;
pub mod prelude;
mod syn_ext;

pub fn call_site<T: FromSpan>() -> T {
    T::from_span(Span::call_site())
}

/// `Span::def_site().located_at(Span::call_site()).as_token()`
#[cfg(not(procmacro2_semver_exempt))]
pub fn def_site<T: FromSpan>() -> T {
    call_site()
}

/// `Span::def_site().located_at(Span::call_site()).as_token()`
#[cfg(procmacro2_semver_exempt)]
pub fn def_site<T: FromSpan>() -> T {
    Span::def_site().located_at(Span::call_site()).as_token()
}

/// `attr` - tokens inside `#[]`. e.g. `derive(EqIgnoreSpan)`, ast_node
pub fn print<T: Into<proc_macro2::TokenStream>>(
    attr: &'static str,
    t: T,
) -> proc_macro::TokenStream {
    use std::env;

    let tokens = t.into();

    match env::var("PRINT_GENERATED") {
        Ok(ref s) if s == "1" || attr == s => {}
        _ => return tokens.into(),
    }

    println!("\n\tOutput of #[{}]:\n\t {}", attr, tokens);
    tokens.into()
}

pub fn is_attr_name(attr: &Attribute, name: &str) -> bool {
    match *attr {
        Attribute {
            path:
                Path {
                    leading_colon: None,
                    ref segments,
                },
            ..
        } if segments.len() == 1 => segments.first().unwrap().ident == name,
        _ => false,
    }
}

/// Returns `None` if `attr` is not a doc attribute.
pub fn doc_str(attr: &Attribute) -> Option<String> {
    fn parse_tts(attr: &Attribute) -> String {
        let meta = attr.parse_meta().ok();
        match meta {
            Some(Meta::NameValue(MetaNameValue {
                lit: Lit::Str(s), ..
            })) => s.value(),
            _ => panic!("failed to parse {}", attr.tokens),
        }
    }

    if !is_attr_name(attr, "doc") {
        return None;
    }

    Some(parse_tts(attr))
}

/// Creates a doc comment.
pub fn make_doc_attr(s: &str) -> Attribute {
    Attribute {
        pound_token: def_site(),
        style: AttrStyle::Outer,
        bracket_token: def_site(),
        path: Ident::new("doc", def_site()).into(),
        tokens: q!(Vars { s },{ = s }).into(),
    }
}

pub fn access_field(obj: &dyn ToTokens, idx: usize, f: &Field) -> Expr {
    Expr::Field(ExprField {
        attrs: Default::default(),
        base: syn::parse2(obj.to_token_stream())
            .expect("swc_macros_common::access_field: failed to parse object"),
        dot_token: Span::call_site().as_token(),
        member: match &f.ident {
            Some(id) => Member::Named(id.clone()),
            _ => Member::Unnamed(Index {
                index: idx as _,
                span: Span::call_site(),
            }),
        },
    })
}

pub fn join_stmts(stmts: &[Stmt]) -> Quote {
    let mut q = Quote::new_call_site();

    for s in stmts {
        q.push_tokens(s);
    }

    q
}

/// fail! is a panic! with location reporting.
#[macro_export]
macro_rules! fail {
    ($($args:tt)+) => {{
        panic!("{}\n --> {}:{}:{}", format_args!($($args)*), file!(), line!(), column!());
    }};
}

#[macro_export]
macro_rules! unimplemented {
    ($($args:tt)+) => {{
        fail!("not yet implemented: {}", format_args!($($args)*));
    }};
}

#[macro_export]
macro_rules! unreachable {
    () => {{
        fail!("internal error: unreachable");
    }};
    ($($args:tt)+) => {{
        fail!("internal error: unreachable\n{}", format_args!($($args)*));
    }};
}
