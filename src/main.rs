mod cfir;
mod pass;
mod codegen;

use cfir::richir::parser::parse::file_parse;

fn main() {
    test_parse_file()
}

fn test_parse_file() {
    let r = file_parse(include_str!("./demo.r.cfir"));
    println!("{:?}", r);
}
