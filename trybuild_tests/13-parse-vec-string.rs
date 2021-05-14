#[derive(derive_more::From, collections_fromstr::FromStr, PartialEq)]
#[item_separator = ","]
struct NewVec(std::vec::Vec<std::string::String>);

fn main(){
    use assert2::assert;
    use std::str::FromStr;

    static VALUES: &str = "Apple🍎,Banana🍌,Carrot🥕";
    assert!(NewVec::from_str(VALUES).unwrap() == NewVec(vec!["Apple🍎", "Banana🍌", "Carrot🥕"].into_iter().map(String::from).collect()));
}