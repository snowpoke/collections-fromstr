use collections_fromstr::FromStr;

#[derive(FromStr, PartialEq)]
#[item_separator = ","]
struct NewVec(Vec<i32>);

impl From<Vec<i32>> for NewVec{
    fn from(v: Vec<i32>) -> Self {
        NewVec(v)
    }
}

fn main(){}