// Created by Gemini - CORRECTED VERSION
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../protos");
    prost_build::Config::new()
        // By removing the `.out_dir("src/models")` line, prost-build will
        // correctly place the generated file in the `OUT_DIR` that Cargo
        // provides, which is what our `src/models/mod.rs` expects.
        .compile_protos(
            &["../protos/price_update.proto", "../protos/candle.proto"],
            &["../protos/"],
        )?;
    Ok(())
}