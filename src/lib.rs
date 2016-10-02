extern crate num;

#[allow(dead_code)]
pub trait Numeric: num::Float + Default {
    fn pi(Self) -> Self;
}

#[allow(dead_code)]
impl Numeric for f32 {
    fn pi(x: f32) -> f32 {
        x * 3.14159265359
    }
}

#[allow(dead_code)]
pub fn rn<TReal: Numeric, TSubj: Numeric>(x: TSubj) -> TReal {
    TReal::from(x).unwrap()
}

#[allow(dead_code)]
pub fn cast<T: num::NumCast, U: num::NumCast>(n: T) -> Option<U> {
    num::NumCast::from(n)
}

#[allow(dead_code)]
pub fn fcast<U: num::NumCast>(n: f32) -> U {
    num::NumCast::from(n).unwrap()
}

#[allow(dead_code)]
impl Numeric for f64 {
    fn pi(x: f64) -> f64 {
        x * 3.14159265359
    }
}