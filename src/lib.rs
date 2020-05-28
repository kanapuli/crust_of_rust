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
