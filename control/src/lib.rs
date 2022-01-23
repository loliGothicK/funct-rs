#![feature(generic_associated_types)]

trait Functor {
    type Underlying;
    type Rebind<B>: Functor;

    fn fmap<F, B>(self, f: F) -> Self::Rebind<B>
    where
        F: FnOnce(Self::Underlying) -> B;
}

trait Applicative: Functor {
    fn pure(a: Self::Underlying) -> Self;
    fn apply<A, B, FA>(self, fa: FA) -> Self::Rebind<B>
        where
            FA: Functor<Underlying = A, Rebind<B> = Self::Rebind<B>>,
            Self::Underlying: FnOnce(<FA as Functor>::Underlying) -> B;
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
}

impl<T> Applicative for Option<T> {
    fn pure(a: Self::Underlying) -> Self {
        Some(a)
    }

    fn apply<A, B, FA>(self, fa: FA) -> Self::Rebind<B>
        where
            FA: Functor<Underlying = A, Rebind<B> = Self::Rebind<B>>,
            Self::Underlying: FnOnce(<FA as Functor>::Underlying) -> B
    {
        self.map_or(None, |f| fa.fmap(f))
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
}
