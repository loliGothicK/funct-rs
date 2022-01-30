// Type Hall
pub trait Generic1<'r, T> {
    type Type = T;
    type Rebind<U: 'r>;
}
