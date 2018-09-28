use std::iter::{ExactSizeIterator, FusedIterator};

pub struct ChainOne<I>
where
    I: Iterator,
{
    iter: I,
    elem: Option<I::Item>,
}

pub trait WithChainOne: Iterator + Sized {
    fn chain_one(self, elem: Self::Item) -> ChainOne<Self>;
}

impl<I> WithChainOne for I
where
    I: Iterator,
{
    fn chain_one(self, elem: Self::Item) -> ChainOne<Self> {
        ChainOne {
            iter: self,
            elem: Some(elem),
        }
    }
}

impl<I> Iterator for ChainOne<I>
where
    I: Iterator,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().or_else(|| self.elem.take())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (low, hi) = self.iter.size_hint();
        let second_len = if self.elem.is_some() { 1 } else { 0 };

        (low + second_len, hi.map(|hi| hi + second_len))
    }
}

impl<I> ExactSizeIterator for ChainOne<I> where I: ExactSizeIterator {}

impl<I> FusedIterator for ChainOne<I> where I: FusedIterator {}

impl<I> DoubleEndedIterator for ChainOne<I>
where
    I: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.elem.take().or_else(|| self.iter.next_back())
    }
}

#[macro_export]
macro_rules! iter {
    () => {
        ::std::iter::none()
    };
    ($first:expr $(, $rest:expr)*$(,)*) => {{
        let i = ::std::iter::once($first);
        $(let i = $crate::WithChainOne::chain_one(i, $rest);)*
        i
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn iter() {
        assert_eq!(
            iter![1, 2, 3, 4, 5].collect::<Vec<_>>(),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn reversed() {
        assert_eq!(
            iter![1, 2, 3, 4, 5].rev().collect::<Vec<_>>(),
            vec![5, 4, 3, 2, 1]
        );
    }
}
