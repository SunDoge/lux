extern crate hyper;
#[macro_use]
extern crate log;
extern crate futures;
extern crate radix_router;

pub mod application;
pub mod context;
pub mod middleware;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
