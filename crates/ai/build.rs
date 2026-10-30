use anyhow::Result;

fn main() -> Result<()> {
    let target_family = std::env::var("CARGO_CFG_TARGET_FAMILY")?;

    if std::env::var_os("CARGO_FEATURE_RUNTIME").is_some() && target_family != "wasm" {
        println!("cargo:rustc-cfg=feature=\"local_fs\"");
    }

    Ok(())
}
