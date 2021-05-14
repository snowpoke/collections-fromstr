use collections_fromstr::FromStr;

#[derive(FromStr)]
#[item_separator = ","]
struct NewVec(Vec<i32>);

fn main(){}