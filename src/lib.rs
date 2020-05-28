pub fn flatten<I>(iter: I) -> Flatten<I> {
  Flatten::new(iter)
}

pub struct Flatten<O>{
  outer: O,
}

impl <O> Flatten<O> {
  fn new(iter: O) -> Self {
    Flatten {
      outer: iter
    }
  }
}

impl <O> Iterator for Flatten<O> 
where 
    O: Iterator,
    O::Item: IntoIterator,
{
  type Item = <O::Item as IntoIterator>::Item;
  fn next(&mut self) -> Option<Self::Item> {
    self.outer.next().and_then(|inner| { inner.into_iter().next()})
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn empty() {
    assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count() , 0)
  }
  #[test]
  fn one_item() {
    assert_eq!(flatten(std::iter::once(vec!["a"])).count() , 1)
  }
  #[test]
  fn two_item() {
    assert_eq!(flatten(std::iter::once(vec!["a","b"])).count() , 2)
  }
  #[test]
  fn two_wide() {
    assert_eq!(flatten( vec![vec!["a"],vec!["b"]].into_iter()).count(), 2)
  }
}
