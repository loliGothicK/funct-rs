use crate::generic::Generic1;

// Functor requirements
pub trait Functor<'r, T: 'r>: Generic1<'r, T> {
    /// fmap :: f a -> (a -> b) -> f b
    ///
    /// Move `self` and maps underlying value by applying `f`.
    /// Because of Rust style (impl Trait), we should flip the arities.
    fn fmap<U: 'r>(self, f: impl Fn(T) -> U) -> Self::Rebind<U>;

    fn replace<U: 'r>(self, fb: Self::Rebind<U>) -> Self::Rebind<U>;
}

// Functor instances

// Type Hall implementation for &[T]
impl<'r, T> Generic1<'r, T> for &'r [T] {
    type Rebind<U: 'r> = Vec<U>;
}
impl<'r, T: 'r> Generic1<'r, T> for Vec<T> {
    type Rebind<U: 'r> = Vec<U>;
}
impl<'r, T: 'r, const N: usize> Generic1<'r, T> for [T; N] {
    type Rebind<U: 'r> = Vec<U>;
}

impl<'r, T: 'r, const N: usize> Functor<'r, T> for [T; N] {
    fn fmap<U: 'r>(self, f: impl Fn(T) -> U) -> Self::Rebind<U> {
        self.into_iter().map(f).collect::<Vec<_>>()
    }
    fn replace<U: 'r>(self, fb: Self::Rebind<U>) -> Self::Rebind<U> {
        fb
    }
}

// Functor Hall implementation for &[T]
impl<'r, T: 'r + Clone> Functor<'r, T> for &'r [T] {
    fn fmap<U: 'r>(self, f: impl Fn(T) -> U) -> Self::Rebind<U> {
        self.to_vec().into_iter().map(f).collect::<Vec<_>>()
    }
    fn replace<U: 'r>(self, fb: Self::Rebind<U>) -> Self::Rebind<U> {
        fb
    }
}

impl<'r, T: 'r + Clone> Functor<'r, T> for Vec<T> {
    fn fmap<U: 'r>(self, f: impl Fn(T) -> U) -> Self::Rebind<U> {
        self.into_iter().map(f).collect::<Vec<_>>()
    }
    fn replace<U: 'r>(self, fb: Self::Rebind<U>) -> Self::Rebind<U> {
        fb
    }
}

// Type Hall implementation for Option<T>
impl<'r, T: 'r> Generic1<'r, T> for Option<T> {
    type Type = T;
    type Rebind<U: 'r> = Option<U>;
}

// Functor Hall implementation for Option<T>
impl<'r, T: 'r> Functor<'r, T> for Option<T> {
    fn fmap<U: 'r>(self, f: impl Fn(Self::Type) -> U) -> Self::Rebind<U> {
        self.map(|x| f(x))
    }

    fn replace<U: 'r>(self, fb: Self::Rebind<U>) -> Self::Rebind<U> {
        fb
    }
}

mod tests {
    use super::Functor;

    #[test]
    fn fmap_vec() {
        assert_eq!(vec![1, 2, 3].fmap(|x| x + x), vec![2, 4, 6]);
    }

    #[test]
    fn fmap_option() {
        assert_eq!(Some(1).fmap(|x| x + x), Some(2));
    }
}
