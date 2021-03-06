use anyhow::*;

use liblumen_alloc::erts::exception::{self, error};
use liblumen_alloc::erts::process::trace::Trace;
use liblumen_alloc::erts::term::prelude::Term;

#[native_implemented::function(erlang:nif_error/1)]
pub fn result(reason: Term) -> exception::Result<Term> {
    Err(error(
        reason,
        None,
        Trace::capture(),
        Some(anyhow!("explicit nif_error from Erlang").into()),
    )
    .into())
}
