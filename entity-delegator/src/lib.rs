#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod lib_tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
