//@ run-rustfix
//@ edition:2018
//@ check-pass
#![feature(ergonomic_clones)]
#![warn(rust_2021_compatibility)]
#![allow(incomplete_features)]

#[derive(Debug)]
struct Foo(i32);
impl Drop for Foo {
    fn drop(&mut self) {
        println!("{:?} dropped", self.0);
    }
}

fn main() {
    let a = (Foo(0), Foo(1));
    let f = use || {
        //~^ HELP: add a dummy
        //~| WARNING: drop order
        let x = a.0;
        println!("{:?}", x);
    };
    f();
}
