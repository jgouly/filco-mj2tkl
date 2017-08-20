#![no_std]
#![no_main]

#[macro_use(reset_fn)]
extern crate mkl27z;

fn main() -> ! {
    loop {}
}
reset_fn!(main);
