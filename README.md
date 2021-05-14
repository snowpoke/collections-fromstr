# collections-fromstr

This crate provides a `FromStr` derive for newtypes on iterable collections on single types, such `Vec<T>` or `HashSet<T>`. The element is parsed by splitting the input along a pattern that you specify though the `#[item_separator]` attribute. The individual items are then parsed using their respective `FromStr` implementations.

## Requirements
- The macro will take the first field in your struct that has a type with a path argument `<T>` and assume that it is the inner type. It will assume that `T` in this example is the item type.
- The item type `T` must not be an actual type (no trait type) that implements `FromStr`.
- As inner type you can use any type that implements `FromIterator<Item = T>`  (like `Vec<T>` or `HashSet<T>`).
- `From` needs to be implemented between your newtype and its inner type. Crates like `derive_more` take easy care of that.

## Example
```rust
use collections_fromstr::FromStr;
use derive_more::From;
use std::str::FromStr;

#[derive(From, FromStr, PartialEq)]
#[item_separator = ","]
struct NewVec(Vec<i32>);

fn main(){
    static VALUES: &str = "1,2,3,-3,-2,-1";
    assert!(NewVec::from_str(VALUES).unwrap() == NewVec(vec![1,2,3,-3,-2,-1]));
}
```

### Wait... you know that I could just use `Iterator::split` for this, right?

Okay, hear me out. Say you've got data like this:
```text
1-3:B,I,U//43-60:I//83-87:I,U//99-104: B,I// etc.
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
It's. So. Clean. 🥺 You'll also be less likely to produce bugs, like forgetting about the case of an empty input string.


## License

`collections-fromstr` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See `LICENSE-APACHE` and `LICENSE-MIT` for details.