pub mod body_1;
/// The Document interface represents any web page loaded in the browser
pub mod create_element_2;
pub mod create_text_node_2;
pub mod get_element_by_id_2;
pub mod new_0;

use std::convert::TryInto;
use std::mem;

use web_sys::Document;

use liblumen_alloc::badarg;
use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::term::prelude::*;

// Private

fn module() -> Atom {
    Atom::try_from_str("Elixir.Lumen.Web.Document").unwrap()
}

fn document_from_term(term: Term) -> Result<&'static Document, exception::Exception> {
    let boxed: Boxed<Resource> = term.try_into()?;
    let document_reference: Resource = term.into();

    match document_reference.downcast_ref() {
        Some(document) => {
            let static_document: &'static Document =
                unsafe { mem::transmute::<&Document, _>(document) };

            Ok(static_document)
        }
        None => Err(badarg!().into()),
    }
}
