fn main() {
    println!("cargo:rerun-if-env-changed=NEOHTOP_ANALYTICS_ENDPOINT");
    tauri_build::build()
}
