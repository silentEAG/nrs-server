fn main() {
    tonic_build::compile_protos("proto/newsrecommend.proto").unwrap();
}
