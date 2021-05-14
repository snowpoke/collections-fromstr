#[derive(derive_more::From, collections_fromstr::FromStr)]
#[item_separator = ","]
struct NewVec(std::vec::Vec<std::string::String>);

fn main(){}