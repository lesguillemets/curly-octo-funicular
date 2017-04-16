pub const SIZE: usize = 8;

// -- TODO
pub const DIRS: &[(i8, i8)] = &[(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1),
                                (1, -1)];


pub trait Monoid {
    fn mempty() -> Self;
    fn madd(&self, Self) -> Self;
}

impl Monoid for (i8, i8) {
    fn mempty() -> (i8, i8) {
        (0, 0)
    }
    fn madd(&self, v1: (i8, i8)) -> (i8, i8) {
        addv(*self, v1)
    }
}

fn addv(v0: (i8, i8), v1: (i8, i8)) -> (i8, i8) {
    (v0.0 + v1.0, v0.1 + v1.1)
}
