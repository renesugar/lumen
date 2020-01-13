mod with_atom_destination;
mod with_local_pid_destination;
mod with_tuple_destination;

use proptest::strategy::Just;
use proptest::test_runner::{Config, TestRunner};
use proptest::{prop_assert, prop_assert_eq};

use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

use crate::otp::erlang;
use crate::otp::erlang::send_2::native;
use crate::process;
use crate::scheduler::{with_process, with_process_arc};
use crate::test::{
    external_arc_node, has_heap_message, has_process_message, registered_name, strategy,
};

#[test]
fn without_atom_pid_or_tuple_destination_errors_badarg() {
    run!(
        |arc_process| {
            (
                Just(arc_process.clone()),
                strategy::term::is_not_destination(arc_process.clone()),
                strategy::term(arc_process),
            )
        },
        |(arc_process, destination, message)| {
            prop_assert_badarg!(
                native(&arc_process, destination, message),
                format!(
                "destination ({}) is not registered_name (atom), {{registered_name, node}}, or pid",
                destination
            )
            );

            Ok(())
        },
    );
}
