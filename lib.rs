#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod popo;
mod nana;

#[skyline::main(name = "buster_climbers")]
pub fn main() {
    popo::install();
    nana::install();
}