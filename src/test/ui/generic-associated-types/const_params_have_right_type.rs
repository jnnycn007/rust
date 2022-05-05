#![feature(generic_associated_types)]

trait Trait {
    type Foo<const N: u8>;
}

impl Trait for () {
    type Foo<const N: u64> = u32;
    //~^ error: associated type `Foo` has an incompatible const parameter type
}

fn main() {}
