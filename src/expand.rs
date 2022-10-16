use std::str::FromStr;

use crate::{
    types::{AutoImplInfo, CommonInfo, DebugExpand, Repetition, Template, TemplateElement},
    FortuplesInfo,
};
use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};
use quote::{quote, TokenStreamExt};
use syn::{parse_str, Expr, FnArg, GenericParam, Ident, Pat, Result, Signature, Type, TypePath};

#[cfg(feature = "debug")]
use {
    std::{
        io::{Read, Write},
        process::Stdio,
    },
    syn::Error,
};

impl FortuplesInfo {
    pub fn expand(self) -> Result<TokenStream> {
        let min_size = self.common.min_size();
        let max_size = self.common.max_size();

        let members = (0..=max_size)
            .map(|i| match self.common.member_type.as_ref() {
                Some((ty, _)) => ty.clone(),
                None => Type::Path(TypePath {
                    qself: None,
                    path: Ident::new(
                        &format!("{}{}", self.common.member_name(), i),
                        Span::call_site(),
                    )
                    .into(),
                }),
            })
            .collect::<Vec<_>>();
        let members = members.as_slice();

        let mut tokens = TokenStream::new();

        for i in min_size..=max_size {
            let members = &members[..i];

            self.expand_instance(&self.template, members, &mut tokens)?;
        }

        self.common.debug_expand(&tokens)?;

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

        let attrs = &self.common.attrs;
        let mut generics = self.generics.clone();

        if self.common.member_type.is_none() {
            for i in 0..members.len() {
                generics.params.push(GenericParam::Type(
                    Ident::new(
                        format!("{}{}", self.common.member_name(), i).as_str(),
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
                TupleLen => tokens.extend(Self::tuple_len(members)),
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
                    TupleLen => tokens.extend(Self::tuple_len(members)),
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

    fn tuple_len(members: &[Type]) -> TokenStream {
        let len = members.len();
        quote!(#len)
    }
}

impl AutoImplInfo {
    pub fn expand(self) -> Result<TokenStream> {
        let mut tokens = TokenStream::new();

        let attrs = &self.common.attrs;
        let item_trait = &self.item_trait;

        tokens.extend(quote! {
            #(#attrs)*
            #item_trait
        });

        self.expand_trait_impl(&mut tokens);

        Ok(tokens)
    }

    fn expand_trait_impl(&self, tokens: &mut TokenStream) {
        let methods = self.expand_trait_impl_methods();

        let pound = TokenStream::from_str("#").unwrap();

        let min_size = self.common.min_size();
        let max_size = self.common.max_size();
        let member_type_attr = self
            .common
            .member_type
            .clone()
            .map(|(ty, _)| quote!(#[tuples::member_type(#ty)]));

        let debug_expand_attr =
            self.common
                .debug_expand
                .clone()
                .map(|(debug_expand, _)| match debug_expand {
                    DebugExpand::Stdout => quote!(#[tuples::debug_expand]),
                    DebugExpand::File((path, _)) => {
                        let path_str = path.to_str().expect(
                            "The path should be already checked. This is a bug, please report this",
                        );
                        quote!(#[tuples::debug_expand(path = #path_str)])
                    }
                });

        let trait_name = &self.item_trait.ident;
        let tuple_metavar = TokenStream::from_str("#Tuple").unwrap();
        let (impl_generics, ty_generics, where_clause) = self.item_trait.generics.split_for_impl();

        let where_clause = if self.common.member_type.is_none() {
            let bounds = quote!(#pound (#pound Member: #trait_name),*);

            where_clause
                .map(|where_clause| quote!(#where_clause, #bounds))
                .unwrap_or_else(|| quote!(where #bounds))
        } else {
            quote!(#where_clause)
        };

        tokens.extend(quote! {
            ::fortuples::fortuples! {
                #[tuples::min_size(#min_size)]
                #[tuples::max_size(#max_size)]
                #member_type_attr
                #debug_expand_attr
                impl #impl_generics #trait_name #ty_generics for #tuple_metavar
                #where_clause
                {
                    #methods
                }
            }
        });
    }

    fn expand_trait_impl_methods(&self) -> TokenStream {
        let mut tokens = TokenStream::new();

        for signature in self.updated_signatures.iter() {
            self.expand_trait_impl_method(signature, &mut tokens);
        }

        tokens
    }

    fn expand_trait_impl_method(&self, signature: &Signature, tokens: &mut TokenStream) {
        let pound = TokenStream::from_str("#").unwrap();

        let self_var_or_ty = match signature.inputs.first() {
            Some(FnArg::Receiver(_)) => quote!(#pound self.),
            _ => quote!(#pound Member::),
        };

        let args_pass = signature.inputs.iter().filter_map(|arg| match arg {
            FnArg::Typed(p) => {
                let arg_name = match p.pat.as_ref() {
                    Pat::Ident(ident) => ident,
                    _ => panic!("Unexpected arg pattern. This is a bug, please report this"),
                };

                match p.ty.as_ref() {
                    Type::Reference(_) => Some(quote!(#arg_name)),
                    _ => Some(quote!(#arg_name.clone())),
                }
            }
            FnArg::Receiver(_) => None,
        });

        let method_name = &signature.ident;
        tokens.extend(quote! {
            #signature {
                #pound (
                    #self_var_or_ty #method_name(#(#args_pass),*);
                )*
            }
        });
    }
}

impl CommonInfo {
    #[cfg(feature = "debug")]
    fn debug_expand(&self, tokens: &TokenStream) -> Result<()> {
        if let Some((ref debug_expand, span)) = self.debug_expand {
            let result = match debug_expand {
                DebugExpand::Stdout => pretty_print_tokenstream(std::io::stdout(), tokens),
                DebugExpand::File((path, span)) => {
                    let file = std::fs::File::create(path).map_err(|err| {
                        Error::new(
                            span.clone(),
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

    #[cfg(not(feature = "debug"))]
    fn debug_expand(&self, _tokens: &TokenStream) -> Result<()> {
        Ok(())
    }
}

#[cfg(feature = "debug")]
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
