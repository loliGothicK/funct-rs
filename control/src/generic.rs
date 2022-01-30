// Type Hall
pub trait Generic1<'r, T: 'r> {
    type Type = T;
    type Ref = &'r T;
    type Rebind<U: 'r>;
}
