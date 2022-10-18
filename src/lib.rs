#![doc = include_str!("../doc/header.md")]
/*!
Procedural macros to generalize inherent and trait implementations over tuples.

* [Introduction](#introduction)
* [Differences from `impl_trait_for_tuples`](#differences-from-impl_trait_for_tuples)
* [Examples](#examples)

## Introduction

When it is a need to implement either a trait
or a generalized type for a combination of tuples,
Rust requires separate implementations to be provided for each tuple variety manually.

This crate provides a proc-macro [`fortuples!`] to write code templates similar to the [`quote!`](https://docs.rs/quote/latest/quote/) macro.
This macro will expand the provided code template for each tuple variety.

Also, there is an attribute macro [`#[auto_impl]`](macro@auto_impl) that implements a given trait
for tuple combinations in a full automatic way.

_This crate is inspired by the [`impl_trait_for_tuples`](https://docs.rs/impl-trait-for-tuples/latest/impl_trait_for_tuples/)._

----

## Differences from `impl_trait_for_tuples`

##### You can write inherent implementations
```
# use fortuples::fortuples;
struct Vector<T>(T);

fortuples! {
    #[tuples::member_type(f32)]
    #[tuples::min_size(2)]
    #[tuples::max_size(3)]
    #[tuples::tuple_name(Coords)]
    impl Vector<#Coords> {
        fn length(&self) -> f32 {
            let coords = &self.0;

            (#(#coords * #coords)+*).sqrt()
        }
    }
}
```

----

##### You don't need to use a custom keyword `for_tuples!` inside the implementation body

Instead, the [`fortuples!`] macro follows the [`quote!`](https://docs.rs/quote/latest/quote/)-like syntax without extra tokens.

```
trait Trait {
    type Ret;
    type Arg;

    fn test(arg: Self::Arg) -> Self::Ret;
}
```

###### impl_trait_for_tuples

```
# use impl_trait_for_tuples::impl_for_tuples;
# trait Trait {
#    type Ret;
#    type Arg;
#
#    fn test(arg: Self::Arg) -> Self::Ret;
# }

#[impl_for_tuples(5)]
impl Trait for Tuple {
    for_tuples!( type Ret = ( #( Tuple::Ret ),* ); );
    for_tuples!( type Arg = ( #( Tuple::Arg ),* ); );

    fn test(arg: Self::Arg) -> Self::Ret {
        for_tuples!( ( #( Tuple::test(arg.Tuple) ),* ) )
    }
}
```

###### fortuples

```
# use fortuples::fortuples;
# trait Trait {
#    type Ret;
#    type Arg;
#
#    fn test(arg: Self::Arg) -> Self::Ret;
# }

fortuples! {
    #[tuples::max_size(5)] // <-- optional, default = 16
    impl Trait for #Tuple
    where
        #(#Member: Trait),*
    {
        type Ret = ( #(#Member::Ret),* );
        type Arg = ( #(#Member::Arg),* );

        fn test(arg: Self::Arg) -> Self::Ret {
            ( #(#Member::test(#arg)),* )
        }
    }
}
```

----

##### Separate attribute macro for full-automatic implementation

###### impl_trait_for_tuples

```
# use impl_trait_for_tuples::impl_for_tuples;
#[impl_for_tuples(5)]
trait Notify {
    fn notify(&self);
}
```

###### fortuples::auto_impl

```
#[fortuples::auto_impl]
#[tuples::max_size(5)] // <-- optional, default = 16
trait Notify {
    fn notify(&self);
}
```

----

## Examples

#### [`fortuples!`] proc-macro

Here is commented example of [`fortuples!`] usage.

You can also view the example [without comments](#fortuples-proc-macro-without-comments).

And the example's [macro expansion](#fortuples-proc-macro-expansion).

_See the [fortuples!] macro documentation to learn about the macro settings (like `#[tuples::min_size]`)._

```
# use fortuples::fortuples;

trait Trait {
    type Ret;

    type Arg;

    type FixedType;

    const VALUE: i32;

    const LENGTH: usize;

    fn test_assoc_fn(arg: Self::Arg) -> Self::Ret;

    fn test_self_fn(&self) -> Result<(), ()>;
}

fortuples! {
#   #[tuples::debug_expand(path = "doc/expand/fortuples.rs")]
    #[tuples::min_size(1)]
    // +----- ^^^^^^^^^^^
    // | The fortuples! macro will start from the empty tuple by default.
    // | Now it will start from the `(Member0,)` tuple.

    impl Trait for #Tuple
    // +----------- ^^^^^
    // | a meta variable that will expand to
    // | `(Member0,)`, `(Member0, Member1)`, and so on.

    where
        #(#Member: Trait<FixedType = i32>),*
    //  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    // | A repetition -- the code inside the `#(...),*`
    // | will expand as many times as the current #Tuple size.
    // |
    // | Inside the i-th code fragment, the #Member meta variable will be substituted
    // | by the i-th member type of the current #Tuple.
    {
        // The `Ret` type will be a tuple consisting of the `Ret` types
        // from the current #Tuple member types
        type Ret = (#(#Member::Ret),*);

        // The `Arg` type will be a tuple consisting of the `Arg` types
        // from the current #Tuple member types
        type Arg = (#(#Member::Arg),*);

        // The `VALUE` will be a sum of all `VALUE`s of the #Tuple member types.
        const VALUE: i32 = #(#Member::VALUE)+*;
        // +------------------------------- ^
        // | Note that the `VALUE`s are separated by a `+` sign.

        const LENGTH: usize = #len(Tuple);
        // +----------------- ^^^^^^^^^^^
        // | This will be expanded to the current #Tuple length.

        type FixedType = i32;

        fn test_assoc_fn(arg: Self::Arg) -> Self::Ret {
            ( #(#Member::test_assoc_fn(#arg)),* )
            // +----------------------- ^^^
            // | Any identifier after the `#` sign that is neither
            // | #Tuple, #Member nor #len(Tuple)
            // | is interpreted as a tuple variable.
            // |
            // | So the above code will expand like this:
            // | ```
            // |    (
            // |        Member0::test_assoc_fn(arg.0),
            // |        Member1::test_assoc_fn(arg.1),
            // |        ...
            // |        MemberN::test_assoc_fn(arg.N),
            // |    )
            // | ```
            // | where `N` = `#len(Tuple)`
        }

        fn test_self_fn(&self) -> Result<(), ()> {
            #(#self.test_self_fn()?;)*
            // +-------------------- ^
            // | Note that there is no separator here.

            Ok(())
        }
    }
}
```

<details>
<summary>Show the example without comments</summary>

#### [`fortuples!`] proc-macro (without comments)

```
# use fortuples::fortuples;

trait Trait {
    type Ret;

    type Arg;

    type FixedType;

    const VALUE: i32;

    const LENGTH: usize;

    fn test_assoc_fn(arg: Self::Arg) -> Self::Ret;

    fn test_self_fn(&self) -> Result<(), ()>;
}

fortuples! {
    #[tuples::min_size(1)]
    impl Trait for #Tuple
    where
        #(#Member: Trait<FixedType = i32>),*
    {
        type Ret = (#(#Member::Ret),*);

        type Arg = (#(#Member::Arg),*);

        const VALUE: i32 = #(#Member::VALUE)+*;

        const LENGTH: usize = #len(Tuple);

        type FixedType = i32;

        fn test_assoc_fn(arg: Self::Arg) -> Self::Ret {
            ( #(#Member::test_assoc_fn(#arg)),* )
        }

        fn test_self_fn(&self) -> Result<(), ()> {
            #(#self.test_self_fn()?;)*

            Ok(())
        }
    }
}
```

</details>
*/


/*!
 <details>
 <summary>Show the macro expansion</summary>

#### [`fortuples!`] proc-macro expansion

```
# trait Trait {
#    type Ret;
#
#    type Arg;
#
#    type FixedType;
#
#    const VALUE: i32;
#
#    const LENGTH: usize;
#
#    fn test_assoc_fn(arg: Self::Arg) -> Self::Ret;
#
#    fn test_self_fn(&self) -> Result<(), ()>;
# }
*/

#![doc = include_str!("../doc/expand/fortuples.rs")]
//!```
//!
//!</details>
//!
//! ----

/*!

#### [`auto_impl`](macro@auto_impl) attribute

There is an option to implement a trait
in a full automatic way using the [`auto_impl`](macro@auto_impl) attribute.

This attribute will automatically generate implementations of the given trait
for tuple combinations.

To view the example's macro expansion click [here](#auto_impl-proc-macro-expansion).

_See the [`auto_impl`](macro@auto_impl) documentation to learn about the
attribute's settings and limitations._

```
#[fortuples::auto_impl]
# #[tuples::debug_expand(path = "doc/expand/auto_impl.rs")]
trait AutoImplTrait {
    fn test(&self, a: i32, b: &f32);
}
```
*/

/*!
 <details>
 <summary>Show the macro expansion</summary>

#### [`auto_impl`](macro@auto_impl) proc-macro expansion

```
# trait AutoImplTrait {
#    fn test(&self, a: i32, b: &f32);
# }
*/
#![doc = include_str!("../doc/expand/auto_impl.rs")]
//!```
//!</details>

use proc_macro::TokenStream;
use syn::{parse_macro_input, spanned::Spanned, Error};

mod expand;
mod parse;
mod types;

use types::{AutoImplInfo, FortuplesInfo};

#[proc_macro]
/// 🚧📝 Documentation coming soon 📝🚧
pub fn fortuples(item: TokenStream) -> TokenStream {
    let info = parse_macro_input!(item as FortuplesInfo);

    match info.expand() {
        Ok(tokens) => tokens,
        Err(e) => e.into_compile_error(),
    }
    .into()
}

#[proc_macro_attribute]
/// 🚧📝 Documentation coming soon 📝🚧
pub fn auto_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return Error::new(
            proc_macro2::TokenStream::from(attr).span(),
            "`auto_impl` doesn't take arguments",
        )
        .into_compile_error()
        .into();
    }

    let info = parse_macro_input!(item as AutoImplInfo);

    match info.expand() {
        Ok(tokens) => tokens,
        Err(e) => e.into_compile_error(),
    }
    .into()
}
