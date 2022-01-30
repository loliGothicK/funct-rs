use crate::functor::Functor;

/// Applicative Requirements
pub trait Applicative<'r, T: 'r>: Functor<'r, T> {
    fn apply<U: 'r, F>(self, f: F) -> Self::Rebind<U>
    where
        F: IntoIterator,
        <F as IntoIterator>::Item: Fn(Self::Type) -> U;
}

// Applicative instances

impl<'r, T: 'r> Applicative<'r, T> for &'r [T]
{
    fn apply<U: 'r, F: IntoIterator>(self, f: F) -> Self::Rebind<U>
    where
        F: IntoIterator,
        <F as IntoIterator>::Item: Fn(Self::Type) -> U
    {
        f.into_iter()
            .flat_map(|func| self.iter().map(func))
            .collect::<Vec<_>>()
    }
}

mod test {
    use super::Applicative;
    #[test]
    fn apply_vec() {
        assert_eq!([1, 2, 3].apply([|x| x + x]), [2, 4, 6]);
        assert_eq!(
            [1, 2, 3].apply([|x| x + x, |x| x + x + x]),
            [2, 4, 6, 3, 6, 9]
        );
    }
}
