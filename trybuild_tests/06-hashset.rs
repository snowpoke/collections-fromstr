use collections_fromstr::FromStr;
use derive_more::From;
use std::collections::HashSet;

#[derive(From, FromStr)]
#[item_separator = ","]
struct NewSet(HashSet<i32>);

fn main(){}