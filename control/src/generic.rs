// Type Hall
pub trait Generic1<T> {
    type Type = T;
    type Rebind<U>;
}
