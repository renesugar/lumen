#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use anyhow::*;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::term::prelude::*;

use crate::erlang::list_to_string::list_to_string;

#[native_implemented::function(erlang:list_to_atom/1)]
pub fn result(string: Term) -> exception::Result<Term> {
    list_to_string(string).and_then(|s| match Atom::try_from_str(s) {
        Ok(atom) => Ok(atom.encode()?),
        Err(atom_error) => Err(atom_error)
            .context(format!("string ({}) cannot be converted to atom", string))
            .map_err(From::from),
    })
}
