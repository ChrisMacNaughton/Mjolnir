extern crate protoc_rust;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src",
        input: &["protos/agent.proto"],
        includes: &[],
    }).expect("protoc");

    protoc_rust::run(protoc_rust::Args {
        out_dir: "src",
        input: &["protos/plugin.proto"],
        includes: &[],
    }).expect("protoc");
}
