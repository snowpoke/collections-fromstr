use collections_fromstr::FromStr;
use derive_more::From;
use std::collections::HashSet;

#[derive(From, FromStr, PartialEq)]
#[item_separator = ","]
struct NewSet(HashSet<i32>);

fn main(){
    use assert2::assert;
    use std::str::FromStr;
    use maplit::hashset;

    static VALUES: &str = "42,100";
    assert!(NewSet::from_str(VALUES).unwrap() == NewSet(hashset!{42,100}));
}