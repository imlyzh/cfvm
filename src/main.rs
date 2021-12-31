mod cfir;
mod pass;
mod analysis;
mod codegen;
use cfir::base::Env;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use cfir::richir::parser::parse::file_parse as rparse;
use cfir::graphir::parser::parse::file_parse as gparse;

use crate::analysis::graphir::live_analysis::RootLiveAnalysis;

fn main() {
    log_init(Level::DEBUG);
    control_flow_graph();
    // test_live_ana();

    // test_parse_richir();
    // test_parse_graphir();
}

fn log_init(l: Level) {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(l)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}

fn control_flow_graph() {
    let mut env = Env::new();
    gparse(include_str!("./demo.g.cfir"), &mut env).unwrap();
    for (_, fun) in &env.function_defs {
        for (source, target) in fun.make_control_flow_graph() {
            println!("{} -> {}", (source.0).0.as_str(), (target.0).0.as_str());
        }
    }
}

fn test_live_ana() {
    let mut env = Env::new();
    gparse(include_str!("./demo.g.cfir"), &mut env).unwrap();
    for (_, fun) in &env.function_defs {
        for (label, used) in fun.live_analysis() {
            println!("{}:\n", (label.0).0.as_str());
            for (v, used) in used {
                println!("\t{}: {}", (v.0).0.as_str(), used);
            }
        }
    }
}

fn test_parse_richir() {
    let mut env = Env::new();
    rparse(include_str!("./demo.r.cfir"), &mut env);
    println!("{:?}", env);
}

fn test_parse_graphir() {
    let mut env = Env::new();
    gparse(include_str!("./demo.g.cfir"), &mut env).unwrap();
    println!("{:?}", env);
}
