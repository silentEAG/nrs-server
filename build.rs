fn main() {
    std::env::set_var(
        "PROTOC",
        "E:\\\\Program\\\\protoc-23.3-win64\\\\bin\\\\protoc.exe",
    );
    tonic_build::compile_protos("proto/newsrecommend.proto").unwrap();
    let db_link = "postgres://news_recommender:nekopara@127.0.0.1:5432/news_recommend";
    println!("cargo:rustc-env=DATABASE_URL={}", db_link);
    println!("cargo:rustc-env=RUST_LOG=DEBUG");
}
