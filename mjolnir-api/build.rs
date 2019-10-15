use protoc_rust;
use protoc_rust::Customize;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/proto",
        input: &[
            "protos/agent.proto",
            "protos/plugin.proto",
            "protos/mjolnir.proto",
        ],
        includes: &[],
        customize: Customize {
          ..Default::default()
        },
    }).expect("protoc");
}
