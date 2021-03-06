#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

use crate::erlang::iolist_or_binary;

/// Returns a binary that is made from the integers and binaries given in iolist
#[native_implemented::function(erlang:iolist_to_iovec/1)]
pub fn result(process: &Process, iolist_or_binary: Term) -> exception::Result<Term> {
    iolist_or_binary::result(process, iolist_or_binary, iolist_or_binary_to_iovec)
}

pub fn iolist_or_binary_to_iovec(
    process: &Process,
    iolist_or_binary: Term,
) -> exception::Result<Term> {
    let binary = iolist_or_binary::to_binary(process, "iolist_or_binary", iolist_or_binary)?;

    Ok(process.list_from_slice(&[binary]))
}
