fn main() {}

#[test]
fn matching_test() {
  use egraph::egraph::EGraph;
  use fcir_frontend::{fcir, pat};

  let op = fcir!("add(add(a, 1): (int, int) -> int, 1): (int, int) -> int");
  let op_pat = pat!("add(add(?a, ?b), ?b)");

  let mut egg: EGraph<()> = EGraph::new();
  egg.add_op(&op);

  let r = egg.matching_op(op_pat);
  println!("op_pat: {:?}", r);
}
