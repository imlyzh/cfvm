pub mod constant;
// pub mod types_old;

pub fn unbalanced_product<T: Clone>(a: &[Vec<T>], b: &[T]) -> Vec<Vec<T>> {
  if !a.is_empty() {
    a.iter()
      .flat_map(|item_a| {
        b.iter().map(move |item_b| {
          let mut r = item_a.clone();
          r.push(item_b.clone());
          r
        })
      })
      .collect::<Vec<Vec<_>>>()
  } else {
    b.iter().map(|x| vec![x.clone()]).collect()
  }
}

mod test {

  #[test]
  fn test_product() {
    use crate::unbalanced_product;
    let r = vec![vec![1, 2], vec![1, 2, 3], vec![1, 2, 3]]
      .into_iter()
      .fold(vec![], |a, b| unbalanced_product(&a, &b));
    println!("out: {:?}", r);
  }
}
