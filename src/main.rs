mod cfir;
mod pass;
mod analysis;
mod codegen;

use cfir::richir::parser::parse::file_parse as rparse;
use cfir::graphir::parser::parse::file_parse as gparse;

use crate::analysis::graphir::live_analysis::RootLiveAnalysis;

fn main() {
    test_live_ana();
    // test_parse_richir();
    // test_parse_graphir();
}

fn test_live_ana() {
    let r = gparse(include_str!("./demo.g.cfir")).unwrap();
    for r in r {
        for (_, fun) in &r.function_defs {
            for (label, used) in fun.live_analysis() {
                println!("{}:\n", (label.0).0.as_str());
                for (v, used) in used {
                    println!("\t{}: {}", (v.0).0.as_str(), used);
                }
            }
        }
    }
}

fn test_parse_richir() {
    let r = rparse(include_str!("./demo.r.cfir"));
    println!("{:?}", r);
}

fn test_parse_graphir() {
    let r = gparse(include_str!("./demo.g.cfir"));
    println!("{:?}", r);
}
