pub fn main() {
    println!("cargo::rerun-if-changed=../assets/**");
    println!("cargo::rerun-if-env-changed=BASE_TEST_DIR");
}