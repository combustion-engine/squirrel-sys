extern crate cmake;
use cmake::Config;

fn main() {
    let mut config = Config::new(".");

    let dst = config.build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=squirrel_static");
}
