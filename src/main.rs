// mod mc;
// mod isa;
// mod cfir;
// mod pass;
// mod analysis;
// mod codegen;
use libcfvm::analysis::graphir::live_analysis::RootLiveAnalysis;
use libcfvm::cfir::base::Env;
// use libcfvm::cfir::richir::parser::parse::file_parse as rparse;
use libcfvm::cfir::graphir::parser::parse::file_parse as gparse;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() {
  log_init(Level::DEBUG);
  control_flow_graph();
  // test_live_ana();

  // test_parse_richir();
  // test_parse_graphir();
}

fn log_init(l: Level) {
  let subscriber = FmtSubscriber::builder().with_max_level(l).finish();

  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

fn control_flow_graph() {
  let mut env = Env::new();
  gparse(include_str!("./demo.g.cfir"), &mut env).unwrap();
  for fun in env.function_defs.values() {
    for (source, target) in fun.make_control_flow_graph() {
      println!("{} -> {}", (source.0).0.as_str(), (target.0).0.as_str());
    }
  }
}

fn test_live_ana() {
  let mut env = Env::new();
  gparse(include_str!("./demo.g.cfir"), &mut env).unwrap();
  for fun in env.function_defs.values() {
    for (label, used) in fun.live_analysis() {
      println!("{}:\n", (label.0).0.as_str());
      for (v, used) in used {
        println!("\t{}: {}", (v.0).0.as_str(), used);
      }
    }
  }
}

/*
fn test_parse_richir() {
    let mut env = Env::new();
    rparse(include_str!("./demo.r.cfir"), &mut env).unwrap();
    println!("{:?}", env);
}

fn test_parse_graphir() {
    let mut env = Env::new();
    gparse(include_str!("./demo.g.cfir"), &mut env).unwrap();
    println!("{:?}", env);
}
 */
