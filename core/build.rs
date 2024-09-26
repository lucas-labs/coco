fn main() {
    println!("cargo:rerun-if-changed=src/i18n/locales");
}
