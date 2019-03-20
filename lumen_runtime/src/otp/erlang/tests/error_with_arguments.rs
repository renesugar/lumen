use super::*;

use std::sync::{Arc, RwLock};

use num_traits::Num;

use crate::atom::Existence::DoNotCare;
use crate::environment::{self, Environment};
use crate::process::IntoProcess;

#[test]
fn with_atom_returns_error_with_atom_reason() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let reason = Term::str_to_atom("reason", DoNotCare, &mut process).unwrap();
    let arguments = Term::cons(
        Term::str_to_atom("first", DoNotCare, &mut process).unwrap(),
        Term::EMPTY_LIST,
        &mut process,
    );

    assert_error!(
        erlang::error_with_arguments(reason, arguments),
        reason,
        arguments,
        &mut process
    );
}

#[test]
fn with_empty_list_returns_error_with_empty_list_reason() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let reason = Term::EMPTY_LIST;
    let arguments = Term::EMPTY_LIST;

    assert_error!(
        erlang::error_with_arguments(reason, arguments),
        reason,
        arguments,
        &mut process
    );
}

#[test]
fn with_list_returns_error_with_list_reason() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let reason = list_term(&mut process);
    let arguments = Term::cons(list_term(&mut process), Term::EMPTY_LIST, &mut process);

    assert_error!(
        erlang::error_with_arguments(reason, arguments),
        reason,
        arguments,
        &mut process
    );
}

#[test]
fn with_small_integer_returns_error_with_small_integer_reason() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let reason = 0usize.into_process(&mut process);
    let arguments = Term::cons(1.into_process(&mut process), Term::EMPTY_LIST, &mut process);

    assert_error!(
        erlang::error_with_arguments(reason, arguments),
        reason,
        arguments,
        &mut process
    );
}

#[test]
fn with_big_integer_returns_error_with_big_integer_reason() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let reason = <BigInt as Num>::from_str_radix("576460752303423489", 10)
        .unwrap()
        .into_process(&mut process);
    let arguments = Term::cons(
        <BigInt as Num>::from_str_radix("576460752303423490", 10)
            .unwrap()
            .into_process(&mut process),
        Term::EMPTY_LIST,
        &mut process,
    );

    assert_error!(
        erlang::error_with_arguments(reason, arguments),
        reason,
        arguments,
        &mut process
    );
}

#[test]
fn with_float_returns_error_with_float_reason() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let reason = 1.0.into_process(&mut process);
    let arguments = Term::cons(
        2.0.into_process(&mut process),
        Term::EMPTY_LIST,
        &mut process,
    );

    assert_error!(
        erlang::error_with_arguments(reason, arguments),
        reason,
        arguments,
        &mut process
    );
}

#[test]
fn with_local_pid_returns_error_with_local_pid_reason() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let reason = Term::local_pid(0, 0, &mut process).unwrap();
    let arguments = Term::cons(
        Term::local_pid(1, 2, &mut process).unwrap(),
        Term::EMPTY_LIST,
        &mut process,
    );

    assert_error!(
        erlang::error_with_arguments(reason, arguments),
        reason,
        arguments,
        &mut process
    );
}

#[test]
fn with_external_pid_returns_error_with_external_pid_reason() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let reason = Term::external_pid(1, 0, 0, &mut process).unwrap();
    let arguments = Term::cons(
        Term::external_pid(2, 3, 4, &mut process).unwrap(),
        Term::EMPTY_LIST,
        &mut process,
    );

    assert_error!(
        erlang::error_with_arguments(reason, arguments),
        reason,
        arguments,
        &mut process
    );
}

#[test]
fn with_tuple_returns_error_with_tuple_reason() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let reason = Term::slice_to_tuple(&[], &mut process);
    let arguments = Term::cons(
        Term::slice_to_tuple(&[1.into_process(&mut process)], &mut process),
        Term::EMPTY_LIST,
        &mut process,
    );

    assert_error!(
        erlang::error_with_arguments(reason, arguments),
        reason,
        arguments,
        &mut process
    );
}

#[test]
fn with_map_returns_error_with_map_reason() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let reason = Term::slice_to_map(
        &[(
            Term::str_to_atom("a", DoNotCare, &mut process).unwrap(),
            1.into_process(&mut process),
        )],
        &mut process,
    );
    let arguments = Term::cons(
        Term::slice_to_map(
            &[(
                Term::str_to_atom("b", DoNotCare, &mut process).unwrap(),
                2.into_process(&mut process),
            )],
            &mut process,
        ),
        Term::EMPTY_LIST,
        &mut process,
    );

    assert_error!(
        erlang::error_with_arguments(reason, arguments),
        reason,
        arguments,
        &mut process
    );
}

#[test]
fn with_heap_binary_returns_error_with_heap_binary_reason() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();
    let reason = Term::slice_to_binary(&[0], &mut process);
    let arguments = Term::cons(
        Term::slice_to_binary(&[1], &mut process),
        Term::EMPTY_LIST,
        &mut process,
    );

    assert_error!(
        erlang::error_with_arguments(reason, arguments),
        reason,
        arguments,
        &mut process
    );
}

#[test]
fn with_subbinary_returns_error_with_subbinary_reason() {
    let environment_rw_lock: Arc<RwLock<Environment>> = Default::default();
    let process_rw_lock = environment::process(Arc::clone(&environment_rw_lock));
    let mut process = process_rw_lock.write().unwrap();

    // <<1::1, 2>>
    let reason_original = Term::slice_to_binary(&[129, 0b0000_0000], &mut process);
    let reason = Term::subbinary(reason_original, 0, 1, 1, 0, &mut process);

    // <<3::3, 4>>
    let argument_original = Term::slice_to_binary(&[96, 0b0100_0000], &mut process);
    let argument = Term::subbinary(argument_original, 0, 2, 1, 0, &mut process);
    let arguments = Term::cons(argument, Term::EMPTY_LIST, &mut process);

    assert_error!(
        erlang::error_with_arguments(reason, arguments),
        reason,
        arguments,
        &mut process
    );
}
