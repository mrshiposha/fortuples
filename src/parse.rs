use std::iter::Peekable;

use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};

use syn::{
    buffer::Cursor,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Attribute, Error, Ident, Lit, LitInt, MetaNameValue, Result, Token, Type,
};

use crate::types::{
    AutoImplInfo, CommonInfo, DebugExpand, FortuplesInfo, RefsMutability, Repetition, Template,
    TemplateElement, TemplatePush,
};

enum FortuplesAttr {
    MinSize((usize, Span)),
    MaxSize((usize, Span)),
    MemberType(Box<(Type, Span)>),
    TupleName((String, Span)),
    MemberName((String, Span)),
    DebugExpand((DebugExpand, Span)),
    RefsTuple((RefsMutability, Span)),
    External(Attribute),
}

impl TryFrom<Attribute> for FortuplesAttr {
    type Error = Error;

    fn try_from(attr: Attribute) -> Result<Self> {
        let segments = attr.path.segments.iter().collect::<Vec<_>>();

        let setting = if let &[tuples, setting, ref rest @ ..] = segments.as_slice() {
            if tuples.ident != "tuples" {
                return Ok(Self::External(attr));
            }

            if !rest.is_empty() {
                return Err(Error::new(
                    attr.path.span(),
                    "unknown setting - the setting path is too long",
                ));
            }

            setting
        } else {
            return Ok(Self::External(attr));
        };

        match setting.ident.to_string().as_str() {
            "min_size" => Ok(Self::MinSize((
                attr.parse_args::<LitInt>()?.base10_parse()?,
                setting.ident.span(),
            ))),
            "max_size" => Ok(Self::MaxSize((
                attr.parse_args::<LitInt>()?.base10_parse()?,
                setting.ident.span(),
            ))),
            "member_type" => Ok(Self::MemberType(Box::new((
                attr.parse_args()?,
                setting.ident.span(),
            )))),
            "tuple_name" => Ok(Self::TupleName((
                attr.parse_args::<Ident>()?.to_string(),
                setting.ident.span(),
            ))),
            "member_name" => Ok(Self::MemberName((
                attr.parse_args::<Ident>()?.to_string(),
                setting.ident.span(),
            ))),
            "debug_expand" => {
                let expand = if attr.tokens.is_empty() {
                    DebugExpand::Stdout
                } else {
                    let name_value = attr.parse_args::<MetaNameValue>()?;
                    let ident = name_value
                        .path
                        .get_ident()
                        .ok_or_else(|| Error::new(name_value.path.span(), "`path` is expected"))?;

                    if ident != "path" {
                        return Err(Error::new(ident.span(), "`path` is expected"));
                    }

                    let value = name_value.lit;

                    match value {
                        Lit::Str(path) => DebugExpand::File((path.value().into(), path.span())),
                        _ => return Err(Error::new(value.span(), "a filepath is expected")),
                    }
                };

                Ok(Self::DebugExpand((expand, setting.ident.span())))
            }
            "refs_tuple" => {
                let arg = attr.tokens.to_string();

                let mutability = if arg.is_empty() {
                    RefsMutability::Immutable
                } else if arg == "(mut)" {
                    RefsMutability::Mutable
                } else {
                    return Err(Error::new(attr.tokens.span(), "unexpected arguments"));
                };

                Ok(Self::RefsTuple((mutability, setting.ident.span())))
            }
            _ => Err(Error::new(setting.ident.span(), "unknown setting")),
        }
    }
}

macro_rules! unique {
    ($info:ident.$opt:ident = $v:expr) => {
        set_unique_optional(&mut $info.$opt, $v, stringify![$opt])?
    };
}

impl Parse for CommonInfo {
    fn parse(input: ParseStream) -> Result<Self> {
        use FortuplesAttr::*;

        let mut info = CommonInfo::default();

        let attrs = Attribute::parse_outer(input)?;
        for attr in attrs {
            match FortuplesAttr::try_from(attr)? {
                MinSize(min_sz) => unique!(info.min_size = min_sz),
                MaxSize(max_sz) => unique!(info.max_size = max_sz),
                MemberType(member_ty) => unique!(info.member_type = *member_ty),
                TupleName(tuple_name) => unique!(info.tuple_name = tuple_name),
                MemberName(member_name) => unique!(info.member_name = member_name),
                DebugExpand(dbg_expand) => unique!(info.debug_expand = dbg_expand),
                RefsTuple(refs_tuple) => unique!(info.refs_tuple = refs_tuple),
                External(attr) => info.attrs.push(attr),
            }
        }

        if info.min_size() >= info.max_size() {
            let min_size_span = info
                .min_size
                .map(|(_, span)| span)
                .unwrap_or_else(Span::call_site);

            let note_min_size_default = match info.min_size {
                Some(_) => "".into(),
                None => format!(" Note: the default min size is {}", info.min_size()),
            };

            let note_max_size_default = match info.max_size {
                Some(_) => "".into(),
                None => format!(" Note: the default max size is {}", info.max_size()),
            };

            return Err(Error::new(
                min_size_span,
                format!(
                    "`min_size` should be strictly less than `max_size`.{}{}",
                    note_min_size_default, note_max_size_default
                ),
            ));
        }

        Ok(info)
    }
}

impl Parse for FortuplesInfo {
    fn parse(input: ParseStream) -> Result<Self> {
        let common = input.parse()?;
        let mut info = FortuplesInfo {
            common,
            ..Default::default()
        };

        input.parse::<Token![impl]>()?;
        info.generics = input.parse()?;

        let stream = input.step(|cursor| Ok((cursor.token_stream(), Cursor::empty())))?;

        let mut template = vec![];
        parse_impl_template(&info, &mut template, stream)?;

        info.template = template;
        Ok(info)
    }
}

impl Parse for AutoImplInfo {
    fn parse(input: ParseStream) -> Result<Self> {
        let info = input.parse()?;
        let item_trait = input.parse()?;

        AutoImplInfo::new(info, item_trait)
    }
}

fn set_unique_optional<T>(
    opt: &mut Option<(T, Span)>,
    (value, span): (T, Span),
    option_name: &str,
) -> Result<()> {
    opt.replace((value, span))
        .map(|(_, old_span)| {
            let mut err = Error::new(span, format!("`{}` should be set only once", option_name));
            err.combine(Error::new(old_span, "the first definition was here"));

            Err(err)
        })
        .unwrap_or(Ok(()))
}

fn parse_impl_template(
    info: &FortuplesInfo,
    template: &mut Template,
    stream: TokenStream,
) -> Result<()> {
    parse_impl_template_impl(info, template, stream, false)
}

fn parse_impl_template_impl(
    info: &FortuplesInfo,
    template: &mut Template,
    stream: TokenStream,
    is_repetition: bool,
) -> Result<()> {
    let mut iter = stream.into_iter().peekable();
    while let Some(tt) = iter.next() {
        match &tt {
            TokenTree::Punct(p) if p.as_char() == '#' && iter.peek().is_some() => {
                let next_tt = iter.next().unwrap();

                match next_tt {
                    TokenTree::Ident(ident) => {
                        parse_metavar(&mut iter, info, template, ident, is_repetition)?
                    }
                    TokenTree::Group(group) => {
                        parse_metavar_repetition(&mut iter, info, template, group)?
                    }
                    _ => {
                        template.push_token(tt);
                        template.push_token(next_tt);
                    }
                }
            }
            TokenTree::Group(group) => {
                let mut group_template = vec![];
                parse_impl_template_impl(info, &mut group_template, group.stream(), is_repetition)?;
                template.push(TemplateElement::Group {
                    delim: group.delimiter(),
                    template: group_template,
                });
            }
            _ => template.push_token(tt),
        }
    }

    Ok(())
}

fn parse_metavar<I>(
    iter: &mut Peekable<I>,
    info: &FortuplesInfo,
    template: &mut Template,
    ident: proc_macro2::Ident,
    is_repetition: bool,
) -> Result<()>
where
    I: Iterator<Item = TokenTree>,
{
    if ident == info.common.tuple_name() {
        template.push(TemplateElement::Tuple);
    } else if ident == info.common.member_name() {
        if is_repetition {
            template.push(TemplateElement::Member);
        } else {
            return Err(Error::new(
                ident.span(),
                format!(
                    concat! [
                        "Attempting to expand the tuple member types without the repetition context.\n",
                        "Try rewrite this like the following: `#(#{}),*`"
                    ],
                    ident,
                ),
            ));
        }
    } else {
        if ident == "len" {
            if let Some(TokenTree::Group(group)) = iter.peek() {
                if let Delimiter::Parenthesis = group.delimiter() {
                    if group.stream().to_string() == info.common.tuple_name() {
                        iter.next().unwrap();
                        template.push(TemplateElement::TupleLen);
                        return Ok(());
                    }
                }
            }
        }

        if is_repetition {
            template.push(TemplateElement::Var(ident));
        } else {
            return Err(Error::new(
                ident.span(),
                format!(
                    concat![
                        "Attempting to expand a tuple variable without the repetition context.\n",
                        "Try rewrite this like the following: `#(#{}),*`"
                    ],
                    ident,
                ),
            ));
        }
    }

    Ok(())
}

fn parse_metavar_repetition<I>(
    iter: &mut Peekable<I>,
    info: &FortuplesInfo,
    template: &mut Template,
    group: proc_macro2::Group,
) -> Result<()>
where
    I: Iterator<Item = TokenTree>,
{
    let separator = iter
        .next()
        .ok_or_else(|| Error::new(group.span_close(), "separator is expected"))
        .map(|tt| match tt {
            TokenTree::Punct(p) => Ok((p.as_char() != '*').then(|| p)),
            _ => Err(Error::new(tt.span(), "punctuation token is expected")),
        })??;

    if let Some(ref separator) = separator {
        iter.next()
            .ok_or_else(|| Error::new(separator.span(), "`*` is expected"))
            .map(|tt| match tt {
                TokenTree::Punct(p) if p.as_char() == '*' => Ok(()),
                _ => Err(Error::new(tt.span(), "`*` is expected")),
            })??;
    }

    let mut rep_template = vec![];
    parse_impl_template_impl(info, &mut rep_template, group.stream(), true)?;
    template.push(Repetition::new(rep_template, separator).into());

    Ok(())
}
