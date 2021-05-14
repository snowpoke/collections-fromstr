use collections_fromstr::FromStr;
use derive_more::From;

#[derive(From, FromStr)]
#[item_separator = ""]
struct NewVec(Vec<i32>);

fn main(){}