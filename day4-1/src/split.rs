pub trait SplitBy<T>: Iterator {
    fn split_by<F>(self, f: F) -> SplitIterator<Self, F>
    where
        Self: Sized,
        F: FnMut(T) -> bool;
    // where
    //     Self: Sized,
    //     F: FnMut(T) -> bool;
}

impl<T> SplitBy<T> for dyn Iterator<Item = T> {
    fn split_by<F>(self, f: F) -> SplitIterator<Self, F>
    where
        Self: Sized,
        F: FnMut(T) -> bool,
    {
        SplitIterator::new(self, f)
    }
}

pub struct SplitIterator<I, F> {
    iter: I,
    f: F,
}

impl<I, F> SplitIterator<I, F> {
    pub fn new(iter: I, f: F) -> SplitIterator<I, F> {
        Self { iter, f }
    }
}

impl<'a, I: Iterator, F> Iterator for SplitIterator<I, F>
where
    F: FnMut(&I::Item) -> bool,
{
    type Item = Vec<&'a I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = vec![];

        while let Some(item) = self.iter.next() {
            if !(self.f)(&item) {
                result.push(&item);
            } else if !result.is_empty() {
                break;
            }
        }

        if !result.is_empty() {
            Some(result)
        } else {
            None
        }
    }
}
