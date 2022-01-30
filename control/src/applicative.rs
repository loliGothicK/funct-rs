use crate::functor::Functor;
use crate::generic::Generic1;

/// Applicative Requirements
pub trait Applicative<'r, T: 'r>: Functor<'r, T> {
    // apply :: f (a -> b) -> f a -> f b
    //          ~~~~~~~~~~    ~~~
    //          FAB           self
    // AB: (a -> b)
    fn apply<B: 'r, FAB: Clone, AB: 'r>(self, fs: FAB) -> Self::Rebind<B>
    where
        <Self as Generic1<'r, T>>::Rebind<B>: 'r,
        FAB: Functor<'r, AB, Rebind<AB> = Self::Rebind<AB>> + Generic1<'r, AB, Type = AB, Rebind<AB> = Self::Rebind<AB>>,
        AB: Fn(T) -> B;

    fn lift_a2<ABC: 'r, F:'r, BC: 'r, B: 'r, FB, C: 'r>(self, f: ABC, fb: FB) -> Self::Rebind<C>
    where
        FB: Clone + Applicative<'r, B> + Generic1<'r, B>,
        <FB as Generic1<'r, B>>::Rebind<F>: Applicative<'r, BC>,
        BC: Fn(B) -> C,
        ABC: Fn(Self::Type, FB::Type) -> C + Copy,
        C: Clone;
}

// Applicative instances

impl<'r, T: 'r + Clone, const N: usize> Applicative<'r, T> for [T; N] {
    // (<*>) :: f a -> f (a -> b) -> f b
    fn apply<B: 'r, FAB: Clone, AB: 'r>(self, fs: FAB) -> Self::Rebind<B>
    where
        <Self as Generic1<'r, T>>::Rebind<B>: 'r,
        FAB: Functor<'r, AB, Rebind<AB> = Self::Rebind<AB>> + Generic1<'r, AB, Type = AB, Rebind<AB> = Self::Rebind<AB>>,
        AB: Fn(T) -> B,
    {
        fs.fmap(std::convert::identity)
            .into_iter()
            .flat_map(|f| self.into_iter().map(f))
            .collect()
    }

    fn lift_a2<ABC: 'r, F:'r, BC: 'r, B: 'r, FB, C: 'r>(self, f: ABC, fb: FB) -> Self::Rebind<C>
    where
        FB: Clone + Applicative<'r, B> + Generic1<'r, B>,
        <FB as Generic1<'r, B>>::Rebind<F>: Applicative<'r, BC>,
        BC: Fn(B) -> C,
        ABC: Fn(Self::Type, FB::Type) -> C + Copy,
        C: Clone,
    {
        self.into_iter()
            .flat_map(|a| fb.apply([move |b| f(a.clone(), b)]))
            .collect()
    }
}

impl<'r, T: 'r + Clone> Applicative<'r, T> for Vec<T> {
    // (<*>) :: f a -> f (a -> b) -> f b
    fn apply<B: 'r, FAB: Clone, AB: 'r>(self, fs: FAB) -> Self::Rebind<B>
    where
        <Self as Generic1<'r, T>>::Rebind<B>: 'r,
        FAB: Functor<'r, AB, Rebind<AB> = Self::Rebind<AB>> + Generic1<'r, AB, Type = AB, Rebind<AB> = Self::Rebind<AB>>,
        AB: Fn(T) -> B,
    {
        fs.fmap(std::convert::identity)
            .into_iter()
            .flat_map(|f| self.into_iter().map(f))
            .collect()
    }

    fn lift_a2<ABC: 'r, F:'r, BC: 'r, B: 'r, FB, C: 'r>(self, f: ABC, fb: FB) -> Self::Rebind<C>
    where
        FB: Clone + Applicative<'r, B> + Generic1<'r, B>,
        <FB as Generic1<'r, B>>::Rebind<F>: Applicative<'r, BC>,
        BC: Fn(B) -> C,
        ABC: Fn(Self::Type, FB::Type) -> C + Copy,
        C: Clone,
    {
        self.into_iter()
            .flat_map(|a| fb.apply([move |b| f(a.clone(), b)]))
            .collect()
    }
}

mod test {
    use super::Applicative;
    #[test]
    fn apply_vec() {
        assert_eq!([1, 2, 3].apply([|x| x + x]), [2, 4, 6]);
        assert_eq!([1, 2, 3].apply([|x| x + x, |x| x + x]), [2, 4, 6, 2, 4, 6]);
    }

    #[test]
    fn lift_a2_vec() {
        assert_eq!(
            [1, 2, 3].lift_a2(|x, y| x + y, [1, 2, 3]),
            [2, 3, 4, 3, 4, 5, 4, 5, 6]
        );
    }
}
