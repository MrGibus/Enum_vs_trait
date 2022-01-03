# Enum vs Trait Benchmark

## Background

When dealing with different structs that exibit similar behaviour there are two methods which can be employed:

A: Pattern Matching via Enum

B: Dynamic Trait Objects

I wrote code using each method to achieve a Vec of closures so I can see what differences there may be.

## Code

### Enum

#### Typical

```rust
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
```

#### Generic

```rust
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
```

### Traits

#### Typical

```rust
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
```

#### Generic

```rust
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
```

## Performance of closure collection

Tested using Criterion.

### Benchmark Code

```rust
use enum_vs_trait_lib::{quxit_enum, quxit_trait};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn enum_benchmark(c: &mut Criterion) {
    c.bench_function("enum", |b| {
        b.iter(|| quxit_enum(black_box(&[3, 10, -2, -3]), black_box(10), black_box(3)))
    });
}

fn trait_benchmark(c: &mut Criterion) {
    c.bench_function("trait", |b| {
        b.iter(|| quxit_trait(black_box(&[3, 10, -2, -3]), black_box(10), black_box(3)))
    });
}

criterion_group!(benches, enum_benchmark, trait_benchmark);
criterion_main!(benches);

```

### Results

```

enum                    time:   [78.075 ns 78.118 ns 78.161 ns]                 
                        change: [-1.0106% -0.9208% -0.8276%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe

trait                   time:   [91.944 ns 91.988 ns 92.032 ns]                  
                        change: [+0.7006% +0.8062% +0.9062%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe

generic trait           time:   [96.324 ns 96.352 ns 96.381 ns]                          
                        change: [+0.4529% +0.5547% +0.6557%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 18 outliers among 100 measurements (18.00%)
  6 (6.00%) low severe
  4 (4.00%) low mild
  5 (5.00%) high mild
  3 (3.00%) high severe

generic enum            time:   [77.570 ns 77.644 ns 77.722 ns]                         
                        change: [+0.9385% +1.0607% +1.2018%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild

```

## Discussion Points

- All methods required a similar amout of code.
- Neither method appears more complex than the other.
- Both methods can utilise generics equally. 
- Enums are noticeably more performant that traits.
- Generics make no noticable impact on performance for enums but a marginal impact for traits. No explanation for this.

