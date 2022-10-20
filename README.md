# fortuples
[![](https://docs.rs/fortuples/badge.svg)](https://docs.rs/fortuples/) [![](https://img.shields.io/crates/v/fortuples.svg)](https://crates.io/crates/fortuples) [![](https://img.shields.io/crates/d/fortuples.svg)](https://crates.io/crates/fortuples)

Procedural macros to generalize inherent and trait implementations over tuples.

* [Introduction](#introduction)
* [Differences from `impl_trait_for_tuples`](#differences-from-impl_trait_for_tuples)
* [Examples](#examples)

## Introduction

When it is a need to implement either a trait or a generalized type for a combination of tuples,
Rust requires separate implementations to be provided for each tuple variety manually.

This crate provides a proc-macro `fortuples!` to write code templates similar to the [`quote!`](https://github.com/dtolnay/quote) macro.
This macro will expand the provided code template for each tuple variety.

Also, an attribute macro `#[auto_impl]` that implements a given trait for tuple combinations in a completely automatic way.

_This crate is inspired by the [`impl_trait_for_tuples`](https://github.com/bkchr/impl-trait-for-tuples)._

----

## Differences from `impl_trait_for_tuples`

##### You can write inherent implementations
```rust
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

Instead, the `fortuples!` macro follows the [`quote!`](https://github.com/dtolnay/quote)-like syntax without extra tokens.

```rust
trait Trait {
    type Ret;
    type Arg;

    fn test(arg: Self::Arg) -> Self::Ret;
}
```

###### impl_trait_for_tuples

```rust
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

```rust
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

```rust
#[impl_for_tuples(5)]
trait Notify {
    fn notify(&self);
}
```

###### fortuples::auto_impl

```rust
#[fortuples::auto_impl]
#[tuples::max_size(5)] // <-- optional, default = 16
trait Notify {
    fn notify(&self);
}
```

----

## Examples

#### `fortuples!` proc-macro

Here is commented example of `fortuples!` usage.

You can also view the example [without comments](#fortuples-proc-macro-without-comments) to see how the macro could look in the wild.

_See the [`fortuples!`](https://docs.rs/fortuples/latest/fortuples/macro.fortuples.html) macro documentation to learn about the macro settings (like `#[tuples::min_size]`)._

```rust
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
    // +----- ^^^^^^^^^^^
    // | The `fortuples!` macro will generate implementations starting with the empty tuple.
    // |
    // | Due to the `min_size` setting,
    // | the implementations will start from the `(Member0,)` tuple.

    impl Trait for #Tuple
    // +----------- ^^^^^
    // | a meta-variable that will expand to
    // | `(Member0,)`, `(Member0, Member1)`, and so on.

    where
        #(#Member: Trait<FixedType = i32>),*
    //  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    // | A repetition -- the code inside the `#(...),*`
    // | will expand as many times as many elements are in the current #Tuple.
    // |
    // | Inside the i-th code fragment, the #Member meta-variable will be substituted
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
        // | Note that a `+` sign separates the `VALUE`s.

        const LENGTH: usize = #len(Tuple);
        // +----------------- ^^^^^^^^^^^
        // | This expands to the current #Tuple length.

        type FixedType = i32;

        fn test_assoc_fn(arg: Self::Arg) -> Self::Ret {
            ( #(#Member::test_assoc_fn(#arg)),* )
            // +----------------------- ^^^
            // | Any identifier after the `#` sign that is neither
            // | #Tuple, #Member, nor #len(Tuple)
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
            // | where `N` equals `#len(Tuple)`
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

#### `fortuples!` proc-macro (without comments)

```rust
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


#### `auto_impl` attribute

There is an option to implement a trait
in a completely automatic way using the `auto_impl` attribute.

This attribute will automatically generate implementations of the given trait
for tuple combinations.

_See the [`auto_impl`](https://docs.rs/fortuples/latest/fortuples/attr.auto_impl.html) documentation to learn about the
attribute's settings and limitations._

```rust
#[fortuples::auto_impl]
trait AutoImplTrait {
    fn test(&self, a: i32, b: &f32);
}
```

