use proc_macro2::{Delimiter, Punct, Span, TokenTree};
use quote::TokenStreamExt;
use std::{path::PathBuf, vec::Vec};
use syn::{
    parse_quote, spanned::Spanned, Attribute, Error, FnArg, Generics, Ident, ItemTrait, Pat,
    Result, ReturnType, Signature, Token, TraitItem, Type,
};

#[derive(Default)]
pub struct CommonInfo {
    pub attrs: Vec<Attribute>,
    pub min_size: Option<(usize, Span)>,
    pub max_size: Option<(usize, Span)>,
    pub member_type: Option<(Type, Span)>,
    pub tuple_name: Option<(String, Span)>,
    pub member_name: Option<(String, Span)>,
    pub refs_tuple: Option<(RefsMutability, Span)>,
    pub debug_expand: Option<(DebugExpand, Span)>,
}

impl CommonInfo {
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

#[derive(Default)]
pub struct FortuplesInfo {
    pub common: CommonInfo,
    pub unsafety: Option<Token![unsafe]>,
    pub generics: Generics,
    pub template: Template,
}

pub struct AutoImplInfo {
    pub common: CommonInfo,
    pub item_trait: ItemTrait,
    pub updated_signatures: Vec<Signature>,
}

impl AutoImplInfo {
    pub fn new(common: CommonInfo, item_trait: ItemTrait) -> Result<Self> {
        if let Some((_, span)) = common.tuple_name {
            return Err(Error::new(span, "`auto_impl` won't use the `tuple_name`"));
        }

        if let Some((_, span)) = common.member_name {
            return Err(Error::new(span, "`auto_impl` won't use the `member_name`"));
        }

        let mut updated_signatures = vec![];

        for item in item_trait.items.iter() {
            match item {
                TraitItem::Method(method) => {
                    if let ReturnType::Type(..) = &method.sig.output {
                        return Err(Error::new(
                            method.sig.output.span(),
                            "`auto_impl` doesn't support return types",
                        ));
                    }

                    updated_signatures.push(Self::updated_signature(&method.sig)?);
                }
                _ => return Err(Error::new(item.span(), "`auto_impl` supports only methods")),
            }
        }

        Ok(Self {
            common,
            item_trait,
            updated_signatures,
        })
    }

    fn updated_signature(signature: &Signature) -> Result<Signature> {
        let mut signature = signature.clone();

        let mut id = 0;

        for arg in signature.inputs.iter_mut() {
            if let FnArg::Typed(arg) = arg {
                match arg.pat.as_mut() {
                    Pat::Ident(ident) => {
                        // To suppress warning in `impl Trait for ()`
                        let ident = Ident::new(format!("_{}", ident.ident).as_str(), ident.span());

                        *arg.pat = parse_quote!(#ident);
                    }
                    Pat::Wild(_) => {
                        let ident = Ident::new(
                            format!("fortuples_auto_impl_unique_arg_{}", id).as_str(),
                            Span::call_site(),
                        );

                        id += 1;

                        *arg.pat = parse_quote!(#ident);
                    }
                    _ => {
                        return Err(Error::new(
                            arg.span(),
                            "`auto_impl` supports only ident of wildcard arguments",
                        ))
                    }
                }
            }
        }

        Ok(signature)
    }
}

#[derive(Clone)]
pub enum RefsMutability {
    Immutable,
    Mutable,
}

#[derive(Clone)]
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
