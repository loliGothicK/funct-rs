use crate::functor::Functor;
use crate::generic::Generic1;

/// Applicative Requirements
pub trait Applicative<'r, T: 'r>: Functor<'r, T> {
    fn apply<U: 'r, F>(self, f: F) -> Self::Rebind<U>
    where
        F: IntoIterator,
        <F as IntoIterator>::Item: Fn(Self::Type) -> U;

    // f a -> (a -> b -> c) -> f b -> f c
    fn lift_a2<F, B: 'r, FB, R: 'r>(self, f: F, fb: FB) -> Self::Rebind<R>
    where
        FB: Clone + Applicative<'r, B> + Generic1<'r, B, Rebind<R> = Self::Rebind<R>>,
        F: Fn(Self::Type, FB::Type) -> R;
}

// Applicative instances

impl<'r, T: 'r> Applicative<'r, T> for &'r [T] {
    fn apply<U: 'r, F: IntoIterator>(self, f: F) -> Self::Rebind<U>
    where
        F: IntoIterator,
        <F as IntoIterator>::Item: Fn(Self::Type) -> U,
    {
        f.into_iter()
            .flat_map(|func| self.iter().map(func))
            .collect::<Vec<_>>()
    }

    fn lift_a2<F, B: 'r, FB, R: 'r>(self, f: F, fb: FB) -> Self::Rebind<R>
    where
        FB: Clone + Applicative<'r, B> + Generic1<'r, B, Rebind<R> = Self::Rebind<R>>,
        F: Fn(Self::Type, FB::Type) -> R,
    {
        self.iter()
            .flat_map(|a| fb.clone().apply([|b| f(a, b)]))
            .collect()
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

    #[test]
    fn lift_a2_vec() {
        assert_eq!(
            [1, 2, 3].lift_a2(|x, y| x + y, &*vec![1, 2, 3]),
            [2, 3, 4, 3, 4, 5, 4, 5, 6]
        );
    }
}
