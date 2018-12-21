
#[macro_use]
extern crate serde_derive;





mod pipeline;

pub use crate::pipeline::Pipeline;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
