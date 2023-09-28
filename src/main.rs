fn main() {}

#[test]
fn matching_test() {
  use cfir_frontend::{cfir, pat};
  use egraph::egraph::EGraph;

  let op = cfir!("add(add(a, 1): (int, int) -> int, 1): (int, int) -> int");
  let op_pat = pat!("add(add(?a, ?b), ?b)");

  let mut egg: EGraph<()> = EGraph::new();
  egg.add_op(&op);

  let r = egg.matching_op(op_pat);
  println!("op_pat: {:?}", r);
}
