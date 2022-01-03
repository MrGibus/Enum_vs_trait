//! https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=1b16e6fb6cd37ec027b3226d8e638655

use std::ops::Mul;
use std::iter::Sum;

pub fn quxit<T: Copy + Mul<Output = T> + Sum>(xs: &[T], p: T, w: T) -> Vec<T> {
    let quxers: Vec<Box<dyn Qux<T>>> = vec![Box::new(Foo { p }), Box::new(Bar { w })];

    let fn_vec: Vec<Box<dyn Fn(T) -> T>> = quxers.iter().map(|f| f.qux()).collect();

    xs.iter()
        .map(|x| fn_vec.iter().map(|f| f(*x)).sum::<T>())
        .collect()
}

struct Foo<T> {
    p: T,
}

struct Bar<T> {
    w: T,
}

trait Qux<T>{
    fn qux(&self) -> Box<dyn Fn(T) -> T + '_>;
}

impl<T: Mul<Output = T> + Copy> Qux<T> for Foo<T> {
    fn qux(&self) -> Box<dyn Fn(T) -> T + '_> {
        Box::new(move |x: T| self.p * x)
    }
}

impl<T: Mul<Output = T> + Copy> Qux<T> for Bar<T> {
    fn qux(&self) -> Box<dyn Fn(T) -> T + '_> {
        Box::new(move |x: T| self.w * x * x)
    }
}
