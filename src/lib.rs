/*!
This crate provides a `FromStr` derive for newtypes on iterable collections on single types, such `Vec<T>` or `HashSet<T>`. The element is parsed by splitting the input along a pattern that you specify though the `#[item_separator]` attribute. The individual items are then parsed using their respective `FromStr` implementations.

# Requirements
- The macro will take the first field in your struct that has a type with a path argument `<T>` and assume that it is the inner type. It will assume that `T` in this example is the item type.
- The item type `T` must not be an actual type (no trait type) that implements `FromStr`.
- As inner type you can use any type that implements `FromIterator<Item = T>`  (like `Vec<T>` or `HashSet<T>`).
- `From` needs to be implemented between your newtype and its inner type. Crates like `derive_more` take easy care of that.

# Example
```rust
use collections_fromstr::FromStr;
use derive_more::From;
use std::str::FromStr;

#[derive(From, FromStr, PartialEq)]
#[item_separator = ","]
struct NewVec(Vec<i32>);

static VALUES: &str = "1,2,3,-3,-2,-1";
assert!(NewVec::from_str(VALUES).unwrap() == NewVec(vec![1,2,3,-3,-2,-1]));
```

## Wait... you know that I could just use `Iterator::split` for this, right?

Okay, hear me out. Say you've got data like this:
```text
1-3:B,I,U//43-60:I//83-87:I,U//99-104: B,I// [etc.]
```
Let's say this data represents text markup: You've got character ranges on the left of the colon `:`, and highlighting information on the right side (`B` = bold, `I` = italics, `U` = underline, but you'll probably expand it later once you get that hedgefund money), of which there might be multiple, separated by commas `,`. Each markup is separated by double slashes `//`.

... Now look at the magic of `FromStr` doing its thing:
```rust
use std::collections::HashSet;
use derive_more::From;
use std::ops::Range;

#[derive(parse_display::FromStr)]
#[display("{0.start}-{0.end}")]
struct CharRange(#[from_str(default)] Range<u32>);

#[derive(parse_display::FromStr, Hash, PartialEq, Eq)]
#[non_exhaustive]
enum MarkupOperation {
    #[display("B")]
    Bold,
    #[display("I")]
    Italics, 
    #[display("U")]
    Underline,
}

#[derive(From, collections_fromstr::FromStr)]
#[item_separator=","]
struct Operations(HashSet<MarkupOperation>);

#[derive(parse_display::FromStr)]
#[display("{range}:{operations}")]
struct Markup{
    range: CharRange,
    operations: Operations,
}

#[derive(From, collections_fromstr::FromStr)]
#[item_separator="//"]
struct MarkupVec(Vec<Markup>);
```

Look at this code.
It's. So. Clean. 🥺 And you'll be less likely to produce bugs, like forgetting about the case of an empty input string.


# License

`collections-fromstr` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See `LICENSE-APACHE` and `LICENSE-MIT` for details.
*/

#![deny(
    deprecated_in_future,
    exported_private_dependencies,
    future_incompatible,
    missing_copy_implementations,
    missing_crate_level_docs,
    missing_debug_implementations,
    missing_docs,
    private_in_public,
    rust_2018_compatibility,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_qualifications,
    trivial_casts,
    trivial_numeric_casts,
    unused_crate_dependencies,
    unused_lifetimes,
    variant_size_differences
)]
#![warn(clippy::pedantic)]

use proc_macro::TokenStream as TokenStream1;

mod derive_from_str;


#[proc_macro_derive(FromStr, attributes(item_separator))]
/// Derives `FromStr` for collections. The input will be split along a separator that is specified in a helper attribute like `#[item_separator=","]`. If the separator is a single character, it will be internally transformed into a `char`.
pub fn derive_from_str(input: TokenStream1) -> TokenStream1 {
    derive_from_str::derive_from_str_inner(input)
}


#[cfg(test)]
mod tests {
    use parse_display as _; // used in doctests

    #[test]
    fn compile_tests(){
        use derive_more as _;
        use assert2 as _;
        use maplit as _;

        let t = trybuild::TestCases::new();
        t.pass("trybuild_tests/01-newtype-vec.rs");
        t.compile_fail("trybuild_tests/02-from-trait-missing.rs");
        t.pass("trybuild_tests/03-derive-more.rs");
        t.pass("trybuild_tests/04-named-newtype.rs");
        t.pass("trybuild_tests/05-full-paths.rs");
        t.pass("trybuild_tests/06-hashset.rs");
        t.compile_fail("trybuild_tests/07-hashmap.rs");
        t.compile_fail("trybuild_tests/08-missing-separator.rs");
        t.compile_fail("trybuild_tests/09-empty-separator.rs");
        t.pass("trybuild_tests/10-parse-vec-i32.rs");
        t.pass("trybuild_tests/11-multibyte-separator.rs");
        t.pass("trybuild_tests/12-string-separator.rs");
        t.pass("trybuild_tests/13-parse-vec-string.rs");
        t.pass("trybuild_tests/14-parse-hashset.rs");
        t.pass("trybuild_tests/15-parse-single-value.rs");
        t.pass("trybuild_tests/16-parse-empty.rs");
    }
}