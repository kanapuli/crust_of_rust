pub fn flatten<I>(iter: I) -> Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter)
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
    O::Item: IntoIterator
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
        if let Some(ref mut inner_iter) = self.inner {
            if let Some(i) = inner_iter.next() {
                return Some(i);
            }
            self.inner = None;
        }
        let inner_item = self.outer.next()?;
        let mut inner_iter = inner_item.into_iter();
        inner_iter.next()
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
            flatten(vec![Vec::<()>::new(), vec![], vec![], vec![]].into_iter()).count(),
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
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]].into_iter()).count(), 2)
    }
}