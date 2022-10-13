use std::{
    io::{Read, Write},
    process::Stdio,
};

use crate::{DebugExpand, FortuplesInfo, Repetition, Template, TemplateElement};
use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};
use quote::{quote, TokenStreamExt};
use syn::{parse_str, Error, Expr, GenericParam, Ident, Result, Type, TypePath};

impl FortuplesInfo {
    pub fn expand(self) -> Result<TokenStream> {
        let min_size = self.min_size();
        let max_size = self.max_size();
        let member_name = self.member_name();

        let members = (0..=max_size)
            .map(|i| match self.member_type.as_ref() {
                Some((ty, _)) => ty.clone(),
                None => Type::Path(TypePath {
                    qself: None,
                    path: Ident::new(&format!("{}{}", member_name, i), Span::call_site()).into(),
                }),
            })
            .collect::<Vec<_>>();
        let members = members.as_slice();

        let mut tokens = TokenStream::new();

        for i in min_size..=max_size {
            let members = &members[..i];

            self.expand_instance(&self.template, members, &mut tokens)?;
        }

        self.debug_expand(&tokens)?;

        Ok(tokens)
    }

    fn expand_instance(
        &self,
        template: &Template,
        members: &[Type],
        tokens: &mut TokenStream,
    ) -> Result<()> {
        let tuple = if members.is_empty() {
            quote!(())
        } else {
            quote!((#(#members),*,))
        };

        let attrs = &self.attrs;
        let mut generics = self.generics.clone();

        if self.member_type.is_none() {
            for i in 0..members.len() {
                generics.params.push(GenericParam::Type(
                    Ident::new(
                        format!("{}{}", self.member_name(), i).as_str(),
                        Span::call_site(),
                    )
                    .into(),
                ))
            }
        }

        let (impl_generics, _, _) = generics.split_for_impl();

        tokens.extend(quote! {
            #(#attrs)*
            impl #impl_generics
        });

        let rep_idx = None;
        self.expand_impl(template, members, &tuple, rep_idx, tokens)?;

        Ok(())
    }

    fn expand_impl(
        &self,
        template: &Template,
        members: &[Type],
        tuple: &TokenStream,
        rep_idx: Option<usize>,
        tokens: &mut TokenStream,
    ) -> Result<()> {
        use TemplateElement::*;

        for element in template {
            match element {
                Tuple => tokens.extend(tuple.clone()),
                Member => match rep_idx {
                    Some(rep_idx) => {
                        tokens.extend(Self::instantiate_member(members, rep_idx));
                    }
                    None => {
                        panic!("Unexpected member repetition. This is a bug, please report this")
                    }
                },
                Var(var_name) => match rep_idx {
                    Some(rep_idx) => {
                        tokens.extend(Self::instantiate_var(var_name, rep_idx)?);
                    }
                    None => {
                        panic!("Unexpected variable repetition. This is a bug, please report this")
                    }
                },
                Raw(stream) => tokens.extend(stream.clone()),
                Repetition(rep) => self.expand_repetition(rep, members, tuple, tokens)?,
                Group { delim, template } => {
                    self.expand_group(template, members, tuple, delim, rep_idx, tokens)?;
                }
            }
        }

        Ok(())
    }

    fn expand_group(
        &self,
        template: &Template,
        members: &[Type],
        tuple: &TokenStream,
        delim: &Delimiter,
        rep_idx: Option<usize>,
        tokens: &mut TokenStream,
    ) -> Result<()> {
        let mut group_stream = TokenStream::new();

        self.expand_impl(template, members, tuple, rep_idx, &mut group_stream)?;
        let group = TokenTree::Group(proc_macro2::Group::new(*delim, group_stream));

        tokens.append(group);

        Ok(())
    }

    fn expand_repetition(
        &self,
        rep: &Repetition,
        members: &[Type],
        tuple: &TokenStream,
        tokens: &mut TokenStream,
    ) -> Result<()> {
        use TemplateElement::*;

        for i in 0..members.len() {
            for element in rep.template.iter() {
                match element {
                    Tuple => tokens.extend(tuple.clone()),
                    Member => tokens.extend(Self::instantiate_member(members, i)),
                    Var(var_name) => tokens.extend(Self::instantiate_var(var_name, i)?),
                    Raw(stream) => tokens.extend(stream.clone()),
                    Repetition(rep) => self.expand_repetition(rep, members, tuple, tokens)?,
                    Group { delim, template } => {
                        self.expand_group(template, members, tuple, delim, Some(i), tokens)?;
                    }
                }
            }

            if let Some(ref separator) = rep.separator {
                if i < members.len() - 1 {
                    tokens.append(separator.clone());
                }
            }
        }

        Ok(())
    }

    fn instantiate_member(members: &[Type], idx: usize) -> TokenStream {
        let member = &members[idx];
        quote!(#member)
    }

    fn instantiate_var(var_name: &Ident, idx: usize) -> Result<TokenStream> {
        let var = parse_str::<Expr>(format!("{}.{}", var_name, idx).as_str())?;
        Ok(quote!(#var))
    }

    fn debug_expand(&self, tokens: &TokenStream) -> Result<()> {
        if let Some((ref debug_expand, span)) = self.debug_expand {
            let result = match debug_expand {
                DebugExpand::Stdout => pretty_print_tokenstream(std::io::stdout(), tokens),
                DebugExpand::File(path) => {
                    let file = std::fs::File::create(path).map_err(|err| {
                        Error::new(
                            span,
                            format!("unable to open the debug expand file: {}", err),
                        )
                    })?;

                    pretty_print_tokenstream(file, tokens)
                }
            };

            result.map_err(|err| Error::new(span, err.to_string()))?;
        }

        Ok(())
    }
}

fn pretty_print_tokenstream<W: Write>(
    mut out: W,
    tokens: &TokenStream,
) -> std::result::Result<(), std::io::Error> {
    use std::io::{Error, ErrorKind};

    let code = tokens.to_string();
    let mut child = std::process::Command::new("rustfmt")
        .args(&["--edition", "2021"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    child
        .stdin
        .take()
        .ok_or_else(|| Error::new(ErrorKind::Other, "unable to write to rustfmt"))?
        .write_all(code.as_bytes())?;

    let mut formatted = String::new();
    child
        .stdout
        .take()
        .ok_or_else(|| Error::new(ErrorKind::Other, "unable to read rustfmt output"))?
        .read_to_string(&mut formatted)?;

    child.wait()?;

    let output_code = if formatted.is_empty() {
        code
    } else {
        formatted
    };

    out.write_all(output_code.as_bytes())
}
