pub fn flatten<I>(iter: I) -> Flatten<I> {
  Flatten::new(iter)
}

pub struct Flatten<I>{
  outer: I,
}

impl <I> Flatten<I> {
  fn new(iter: I) -> Self {
    Flatten {
      outer: iter
    }
  }
}
