use crate::{matcher::Matching, rewriter::Rewrite, Records, Unify};

pub trait Scheduler<
  Pat,
  Tem,
  I: Clone + Matching<Pat, T1>,
  T1: Unify + Into<T2>,
  T2: Rewrite<Tem, I>,
>
{
  fn add_rule(&mut self, matcher: Pat, rewriter: Tem);
  fn add_rules(&mut self, rules: Vec<(Pat, Tem)>) {
    for (matcher, rewriter) in rules {
      self.add_rule(matcher, rewriter);
    }
  }

  // todo: trigger, option

  fn runner(&self, input: Records<I>) -> Vec<I>;
}
