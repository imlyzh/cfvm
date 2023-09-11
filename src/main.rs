fn main() {}

#[test]
fn matching_test() {
  use egraph::egraph::EGraph;
  use fcir::rewriter::form::GetForm;
  use fcir_frontend::{fcir, pat};

  let op = fcir!("add(add(a, 1): (int, int) -> int, 1): (int, int) -> int");
  let op_pat = pat!("add(add(?a, ?b), ?b)");
  // println!("ir: {:?}", op);
  // println!("pat: {:?}", op_pat);
  println!("pat form: {:?}", op_pat.get_form());

  let mut egg: EGraph<()> = EGraph::new();
  egg.add_op(&op);
  // use egraph::gen_fcir::GenFcir;
  for (f, likes) in egg.likes.0.iter() {
    println!("form: {:?}", f);
    println!("len: {}", likes.len());
    println!("------------------------");
    for i in likes {
      println!("likes: {:?}", i);
    }
    println!("------------------------");
  }
  // println!("egg: {:?}", egg);
  let r = egg.matching_op(op_pat);
  println!("");
  println!("op_pat: {:?}", r);
}
