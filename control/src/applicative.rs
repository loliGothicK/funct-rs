use crate::functor::Functor;

/// Applicative Requirements
pub trait Applicative<T>: Functor<T> {
    // apply :: f a -> (a -> b) -> f b
    fn apply<U>(self, f: Self::Rebind<Box<dyn Fn(&Self::Type) -> U>>) -> Self::Rebind<U>;
}

// Applicative instances

impl<T> Applicative<T> for &[T] {
    fn apply<U>(self, f: Self::Rebind<Box<dyn Fn(&Self::Type) -> U>>) -> Self::Rebind<U> {
        f.iter()
            .flat_map(|func| self.iter().map(|item| func(item)))
            .collect()
    }
}

#[allow(dead_code)]
struct Pure<T>(T);

mod test {
    use super::Applicative;
    #[test]
    fn apply_vec() {
        assert_eq!([1, 2, 3].apply(vec![Box::new(|x| x + x)]), [2, 4, 6]);
        assert_eq!(
            [1, 2, 3].apply(vec![Box::new(|x| x + x), Box::new(|x| x + x + x)]),
            [2, 4, 6, 3, 6, 9]
        );
    }
}
