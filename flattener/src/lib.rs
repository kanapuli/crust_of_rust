pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            inner: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                self.inner = None;
            }
            let next_inner_iter = self.outer.next()?.into_iter();
            self.inner = Some(next_inner_iter);
        }
    }
}

//DoubleEndedIterator can be implemented only to objects that have implemented Iterator trait
impl <O> DoubleEndedIterator for Flatten<O>
where
    O: Iterator + DoubleEndedIterator,  // this can be just DoubleEndedIterator because DoubleEndedIterator implements Iterator already
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
  fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(i) = inner_iter.next_back() {
                    return Some(i);
                }
                self.inner = None;
            }
            let next_inner_iter = self.outer.next_back()?.into_iter();
            self.inner = Some(next_inner_iter);
        }
  }

}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0)
    }
    #[test]
    fn empty_wide() {
        assert_eq!(
            flatten(vec![Vec::<()>::new(), vec![], vec![], vec![]]).count(),
            0
        )
    }
    #[test]
    fn one_item() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1)
    }
    #[test]
    fn two_item() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2)
    }
    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2)
    }

    //Tests for DoubleEndedIterator 
    #[test]
    fn reverse() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"]))
          .rev()
          .collect::<Vec<_>>(), vec!["b", "a"])
    }
    #[test]
    fn reverse_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]])
          .rev()
          .collect::<Vec<_>>(), vec!["b", "a"])
    }
}
