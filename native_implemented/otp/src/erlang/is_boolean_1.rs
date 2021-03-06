use liblumen_alloc::erts::term::prelude::*;

#[native_implemented::function(erlang:is_boolean/1)]
pub fn result(term: Term) -> Term {
    term.is_boolean().into()
}
