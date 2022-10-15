use proc_macro2::{Delimiter, Punct, Span, TokenTree};
use quote::TokenStreamExt;
use std::{path::PathBuf, vec::Vec};
use syn::{Attribute, Generics, Type};

#[derive(Default)]
pub struct FortuplesInfo {
    pub attrs: Vec<Attribute>,
    pub min_size: Option<(usize, Span)>,
    pub max_size: Option<(usize, Span)>,
    pub member_type: Option<(Type, Span)>,
    pub tuple_name: Option<(String, Span)>,
    pub member_name: Option<(String, Span)>,
    pub debug_expand: Option<(DebugExpand, Span)>,
    pub generics: Generics,
    pub template: Template,
}

impl FortuplesInfo {
    pub fn min_size(&self) -> usize {
        self.min_size.map(|(s, _)| s).unwrap_or(0)
    }

    pub fn max_size(&self) -> usize {
        self.max_size.map(|(s, _)| s).unwrap_or(16)
    }

    pub fn tuple_name(&self) -> &str {
        self.tuple_name
            .as_ref()
            .map(|(n, _)| n.as_str())
            .unwrap_or("Tuple")
    }

    pub fn member_name(&self) -> &str {
        self.member_name
            .as_ref()
            .map(|(n, _)| n.as_str())
            .unwrap_or("Member")
    }
}

pub enum DebugExpand {
    Stdout,
    File((PathBuf, Span)),
}

pub trait TemplatePush {
    fn push_token(&mut self, tt: TokenTree);
}

pub type Template = Vec<TemplateElement>;

#[derive(Debug)]
pub struct Repetition {
    pub template: Template,
    pub separator: Option<Punct>,
}

impl Repetition {
    pub fn new(template: Template, separator: Option<Punct>) -> Self {
        Self {
            template,
            separator,
        }
    }
}

#[derive(Debug)]
pub enum TemplateElement {
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
