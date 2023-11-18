//! Constants and utilities for conversion between Rust string-likes and Elixir atoms.

use crate::error::Error;
use rustler::{types::atom::Atom, Encoder, Env, Term};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref OK: String = String::from("Ok");
    pub static ref ERROR: String = String::from("Err");
}

rustler::atoms! {
    nil,
    ok,
    error,

    eof,

    // Posix
    enoent, // File does not exist
    eacces, // Permission denied
    epipe, // Broken pipe
    eexist, // File exists

    unknown, // Other error
    true_ = "true",
    false_ = "false",
    __struct__
}

/**
 * Attempts to create an atom term from the provided string (if the atom already exists in the atom table). If not, returns a string term.
 */
pub fn str_to_term<'a>(env: &Env<'a>, string: &str) -> Result<Term<'a>, Error> {
    if string == "Ok" {
        Ok(ok().encode(*env))
    } else if string == "Err" {
        Ok(error().encode(*env))
    } else {
        match Atom::try_from_bytes(*env, string.as_bytes()) {
            Ok(Some(term)) => Ok(term.encode(*env)),
            Ok(None) => Err(Error::InvalidStringable),
            _ => Err(Error::InvalidStringable),
        }
    }
}
