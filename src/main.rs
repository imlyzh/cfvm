
fn main() {}

#[test]
fn matching_test() {
  use fcir_frontend::{fcir, pat};
  use egraph::egraph::EGraph;


  let op = fcir!("add(add(a, 1): (int, int) -> int, 1): (int, int) -> int");
  let op_pat = pat!("add(add(?a, ?b), ?b)");
  // println!("ir: {:?}", op);
  // println!("pat: {:?}", op_pat);

  let mut egg: EGraph<()> = EGraph::new();
  egg.add_op(&op);
  let r = egg.matching_op(op_pat);
  println!("op_pat: {:?}", r);
}
