// Created by Gemini
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../protos");
    prost_build::Config::new()
        .out_dir("src/models")
        .compile_protos(&["../protos/price_update.proto", "../protos/candle.proto"], &["../protos/"])?;
    Ok(())
}