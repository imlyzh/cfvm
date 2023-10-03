

fn main() {}

#[test]
fn matching_test() {
  use cfir_frontend::{cfir_expr, pat};
  use egraph::egraph::EGraph;

  let op = cfir_expr!("add(add(a, 1): (int, int) -> int, 1): (int, int) -> int");
  let op_pat = pat!("add(add(?a, ?b), ?b)");

  let mut egg: EGraph<()> = EGraph::new();
  egg.add_op(&op);

  let r = egg.matching_op(op_pat);
  println!("op_pat: {:?}", r);
}


#[test]
fn relinking_test() {
  use cfir::tools::relinking;
  use cfir_frontend::cfir_block;
  use cfir::value::Value;

  let ops = cfir_block!("
  r = arthi.add (a, 1): (int, int) -> int
  fn.ret (r): (int) -> never
  ").2;

  let relinked_ops = relinking(&ops);
  assert_eq!(relinked_ops[1].as_ref().borrow().uses[0], Value::Use(relinked_ops[0].clone(), 0));
}
