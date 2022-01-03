use std::ops::Mul;
use std::iter::Sum;

pub fn quxit<T: Copy + Mul<Output = T> + Sum>(xs: &[T], p: T, w: T) -> Vec<T>{
    let quxers: Vec<Qux<T>> = vec![Qux::Foo(Foo { p }), Qux::Bar(Bar { w })];

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

enum Qux<T> {
    Foo(Foo<T>),
    Bar(Bar<T>),
}

impl<T: Mul<Output = T> + Copy> Qux<T> {
    fn qux(&self) -> Box<dyn Fn(T) -> T + '_> {
        match self {
            Qux::Foo(foo) => Box::new(move |x: T| foo.p * x),
            Qux::Bar(bar) => Box::new(move |x: T| bar.w * x * x),
        }
    }
}
