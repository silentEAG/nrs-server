fn main() {
    let db_link = "postgres://news_recommender:nekopara@127.0.0.1:5432/news_recommend";
    println!("cargo:rustc-env=DATABASE_URL={}", db_link);
    println!("cargo:rustc-env=RUST_LOG=DEBUG");
}
