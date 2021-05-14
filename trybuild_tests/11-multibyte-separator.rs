use collections_fromstr::FromStr;
use derive_more::From;

#[derive(From, FromStr, PartialEq)]
#[item_separator = "🔥"]
struct NewVec(Vec<i32>);

fn main(){
    use assert2::assert;
    use std::str::FromStr;

    static VALUES: &str = "1🔥2🔥3🔥-3🔥-2🔥-1";
    assert!(NewVec::from_str(VALUES).unwrap() == NewVec(vec![1,2,3,-3,-2,-1]));
}