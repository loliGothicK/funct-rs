#![feature(generic_associated_types)]

use std::convert::identity;

trait Functor {
    type Underlying;
    type Rebind<T>;

    fn fmap<F, B>(self, f: F) -> Self::Rebind<B>
    where
        F: FnOnce(Self::Underlying) -> B;

    fn replace<B>(self, fb: Self::Rebind<B>) -> Self::Rebind<B>;
}

trait Applicative: Functor {
    fn pure(a: Self::Underlying) -> Self;
    fn apply<A, B, FA>(self, fa: FA) -> Self::Rebind<B>
    where
        FA: Functor<Underlying = A, Rebind<B> = Self::Rebind<B>>,
        Self::Underlying: FnOnce(<FA as Functor>::Underlying) -> B;
    fn lift_a2<F, A, B, C, FA, FB>(f: F, fa: FA, fb: FB) -> Self::Rebind<C>
    where
        FA: Functor<Underlying = A, Rebind<A> = Option<A>>,
        FB: Functor<Underlying = B, Rebind<C> = Option<C>>,
        F: FnOnce(<FA as Functor>::Underlying, <FB as Functor>::Underlying) -> C;
}

impl<T> Functor for Option<T> {
    type Underlying = T;
    type Rebind<B> = Option<B>;

    fn fmap<F, B>(self, f: F) -> Self::Rebind<B>
    where
        F: FnOnce(T) -> B,
    {
        self.map(f)
    }

    fn replace<B>(self, fb: Self::Rebind<B>) -> Self::Rebind<B> {
        fb
    }
}

impl<T> Applicative for Option<T> {
    fn pure(a: Self::Underlying) -> Self {
        Some(a)
    }

    fn apply<A, B, FA>(self, fa: FA) -> Self::Rebind<B>
    where
        FA: Functor<Underlying = A, Rebind<B> = Self::Rebind<B>>,
        Self::Underlying: FnOnce(<FA as Functor>::Underlying) -> B,
    {
        self.map_or(None, |f| fa.fmap(f))
    }

    fn lift_a2<F, A, B, C, FA, FB>(f: F, fa: FA, fb: FB) -> Self::Rebind<C>
    where
        FA: Functor<Underlying = A, Rebind<A> = Option<A>>,
        FB: Functor<Underlying = B, Rebind<C> = Option<C>>,
        F: FnOnce(<FA as Functor>::Underlying, <FB as Functor>::Underlying) -> C,
    {
        fa.fmap(identity).and_then(|a| fb.fmap(|b| f(a, b)))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Applicative, Functor};

    #[test]
    fn fmap() {
        let opt = Some(2);
        assert_eq!(opt.fmap(|x| x + 2), Some(4));
    }

    #[test]
    fn apply() {
        let add = Some(|x| x + 1);
        assert_eq!(add.apply(Some(1)), Some(2));
    }

    #[test]
    fn lift_a2() {
        assert_eq!(Option::<i32>::lift_a2(|x, y| x + y, Some(1), Some(1)), Some(2));
    }
}
