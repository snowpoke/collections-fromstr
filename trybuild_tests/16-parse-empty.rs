use collections_fromstr::FromStr;

#[derive(FromStr, PartialEq)]
#[item_separator = ","]
struct NewVec(Vec<i32>);

impl From<Vec<i32>> for NewVec{
    fn from(v: Vec<i32>) -> Self {
        NewVec(v)
    }
}

fn main(){
    use assert2::assert;
    use std::str::FromStr;

    static VALUES: &str = "";
    assert!(NewVec::from_str(VALUES).unwrap() == NewVec(vec![]));
}