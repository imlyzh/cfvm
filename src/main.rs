mod cfir;
mod pass;
mod codegen;

use cfir::richir::parser::parse::file_parse as rparse;
use cfir::graphir::parser::parse::file_parse as gparse;

fn main() {
    test_parse_richir();
    test_parse_graphir();
}

fn test_parse_richir() {
    let r = rparse(include_str!("./demo.r.cfir"));
    println!("{:?}", r);
}

fn test_parse_graphir() {
    let r = gparse(include_str!("./demo.g.cfir"));
    println!("{:?}", r);
}
