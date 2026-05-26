// Exposes the `src/protos/anytype.rs` file as the `anytype` module
pub mod anytype;

// Exposes the `src/protos/anytype.model.rs` file.
// We map the dotted filename to a safe Rust identifier: `anytype_model`
#[path = "anytype.model.rs"]
pub mod anytype_model;
