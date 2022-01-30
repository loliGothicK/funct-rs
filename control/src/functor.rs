use crate::generic::Generic1;

// Functor requirements
pub trait Functor<T>: Generic1<T> {
    /// fmap :: f a -> (a -> b) -> f b
    ///
    /// Move `self` and maps underlying value by applying `f`.
    /// Because of Rust style (impl Trait), we should flip the arities.
    fn fmap<U>(self, f: impl Fn(&Self::Type) -> U) -> Self::Rebind<U>;
}

// Functor instances

// Type Hall implementation for &[T]
impl<T> Generic1<T> for &[T] {
    type Rebind<U> = Vec<U>;
}

// Functor Hall implementation for &[T]
impl<T> Functor<T> for &[T] {
    fn fmap<U>(self, f: impl Fn(&Self::Type) -> U) -> Self::Rebind<U> {
        self.iter().map(f).collect()
    }
}

// Type Hall implementation for Option<T>
impl<T> Generic1<T> for Option<T> {
    type Rebind<U> = Option<U>;
}

// Functor Hall implementation for Option<T>
impl<T> Functor<T> for Option<T> {
    fn fmap<U>(self, f: impl Fn(&Self::Type) -> U) -> Self::Rebind<U> {
        self.map(|x| f(&x))
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
