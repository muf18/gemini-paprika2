// Created by Gemini - CORRECTED VERSION
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tell cargo to tell rustc to expect the `frb_expand` cfg flag.
    println!("cargo:rustc-check-cfg=cfg(frb_expand)");
    
    println!("cargo:rerun-if-changed=../protos");
    prost_build::Config::new()
        .compile_protos(
            &["../protos/price_update.proto", "../protos/candle.proto"],
            &["../protos/"],
        )?;
    Ok(())
}