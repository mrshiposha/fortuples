use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Punct, Span, TokenTree};
use quote::TokenStreamExt;
use std::{path::PathBuf, vec::Vec};
use syn::{parse_macro_input, Attribute, Generics, Type};

mod expand;
mod parse;

#[proc_macro]
pub fn fortuples(item: TokenStream) -> TokenStream {
    let info = parse_macro_input!(item as FortuplesInfo);

    match info.expand() {
        Ok(tokens) => tokens,
        Err(e) => e.into_compile_error(),
    }
    .into()
}

#[derive(Default)]
struct FortuplesInfo {
    attrs: Vec<Attribute>,
    min_size: Option<(usize, Span)>,
    max_size: Option<(usize, Span)>,
    member_type: Option<(Type, Span)>,
    tuple_name: Option<(String, Span)>,
    member_name: Option<(String, Span)>,
    debug_expand: Option<(DebugExpand, Span)>,
    generics: Generics,
    template: Template,
}

impl FortuplesInfo {
    fn min_size(&self) -> usize {
        self.min_size.map(|(s, _)| s).unwrap_or(0)
    }

    fn max_size(&self) -> usize {
        self.max_size.map(|(s, _)| s).unwrap_or(16)
    }

    fn tuple_name(&self) -> &str {
        self.tuple_name
            .as_ref()
            .map(|(n, _)| n.as_str())
            .unwrap_or("Tuple")
    }

    fn member_name(&self) -> &str {
        self.member_name
            .as_ref()
            .map(|(n, _)| n.as_str())
            .unwrap_or("Member")
    }
}

enum DebugExpand {
    Stdout,
    File((PathBuf, Span)),
}

trait TemplatePush {
    fn push_token(&mut self, tt: TokenTree);
}

type Template = Vec<TemplateElement>;

#[derive(Debug)]
struct Repetition {
    template: Template,
    separator: Option<Punct>,
}

impl Repetition {
    fn new(template: Template, separator: Option<Punct>) -> Self {
        Self {
            template,
            separator,
        }
    }
}

#[derive(Debug)]
enum TemplateElement {
    Tuple,
    TupleLen,
    Member,
    Var(proc_macro2::Ident),
    Repetition(Repetition),
    Raw(proc_macro2::TokenStream),
    Group {
        delim: Delimiter,
        template: Template,
    },
}

impl From<Repetition> for TemplateElement {
    fn from(r: Repetition) -> Self {
        Self::Repetition(r)
    }
}

impl TemplatePush for Template {
    fn push_token(&mut self, tt: TokenTree) {
        match self.last_mut() {
            Some(TemplateElement::Raw(element)) => element.append(tt),
            _ => self.push(TemplateElement::Raw(tt.into())),
        }
    }
}
