use collections_fromstr::FromStr;
use derive_more::From;

#[derive(From, FromStr)]
struct NewVec(Vec<i32>);

fn main(){}