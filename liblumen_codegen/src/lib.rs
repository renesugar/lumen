#![feature(extern_types)]
#![feature(associated_type_bounds)]
pub mod codegen;
pub mod ffi;
pub mod linker;
pub mod llvm;
pub mod mlir;

pub use self::ffi::passes::print_passes;
pub use self::ffi::target::{self, print_target_cpus, print_target_features};
pub use self::ffi::util::llvm_version;

use liblumen_session::Options;

pub type Result<T> = std::result::Result<T, anyhow::Error>;

/// Perform initialization of MLIR/LLVM for code generation
pub fn init(options: &Options) {
    self::ffi::init(options);
}