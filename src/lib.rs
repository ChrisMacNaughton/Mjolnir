extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

extern crate mjolnir_api;

mod pipeline;

pub use pipeline::Pipeline;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
