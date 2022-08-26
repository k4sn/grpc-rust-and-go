fn main() {
    tonic_build::configure()
        .out_dir("src")
        .compile(&["../ProductInfo.proto"], &["../"])
        .unwrap();
}
