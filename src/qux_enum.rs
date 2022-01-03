//! https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=6b8d67ae6b92be56d64e6b978e530cb8

pub fn quxit(xs: &[i32], p: i32, w: i32) -> Vec<i32> {
    let quxers = vec![Qux::Foo(Foo { p }), Qux::Bar(Bar { w })];

    let fn_vec: Vec<Box<dyn Fn(i32) -> i32>> = quxers.iter().map(|f| f.qux()).collect();

    xs.iter()
        .map(|x| fn_vec.iter().map(|f| f(*x)).sum::<i32>())
        .collect()
}

struct Foo {
    p: i32,
}

struct Bar {
    w: i32,
}

enum Qux {
    Foo(Foo),
    Bar(Bar),
}

impl Qux {
    fn qux(&self) -> Box<dyn Fn(i32) -> i32 + '_> {
        match self {
            Qux::Foo(foo) => Box::new(move |x: i32| foo.p * x),
            Qux::Bar(bar) => Box::new(move |x: i32| bar.w * x * x),
        }
    }
}
