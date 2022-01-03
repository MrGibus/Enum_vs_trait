//! https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=1b16e6fb6cd37ec027b3226d8e638655

pub fn quxit(xs: &[i32], p: i32, w: i32) -> Vec<i32> {
    let quxers: Vec<Box<dyn Qux>> = vec![Box::new(Foo { p }), Box::new(Bar { w })];

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

trait Qux {
    fn qux(&self) -> Box<dyn Fn(i32) -> i32 + '_>;
}

impl Qux for Foo {
    fn qux(&self) -> Box<dyn Fn(i32) -> i32 + '_> {
        Box::new(move |x: i32| self.p * x)
    }
}

impl Qux for Bar {
    fn qux(&self) -> Box<dyn Fn(i32) -> i32 + '_> {
        Box::new(move |x: i32| self.w * x * x)
    }
}
