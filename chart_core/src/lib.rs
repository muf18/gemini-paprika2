// Created by Gemini - FINAL CORRECTED VERSION

pub mod adapters;
pub mod aggregator;
pub mod api;
pub mod models;

// ====================================================================
// THE FIX IS HERE: Added an attribute to allow the specific lint
// that is triggered by the auto-generated FFI code.
// ====================================================================
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub mod frb_generated;