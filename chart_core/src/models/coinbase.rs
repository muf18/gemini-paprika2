// Created by Gemini
// This file will be populated by `build.rs` with `include!` macros.
// For it to compile before the first build, we can add a placeholder.
// The build script will generate a file named `models.rs` in this directory.

// This line tells Rust to include the contents of the generated file.
// The file is generated from your .proto schemas.
include!(concat!(env!("OUT_DIR"), "/models.rs"));