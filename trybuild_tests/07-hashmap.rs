use collections_fromstr::FromStr;
use derive_more::From;
use std::collections::HashMap;

#[derive(From, FromStr)]
#[item_separator = ","]
struct NewMap(HashMap<i32, String>);

fn main(){}