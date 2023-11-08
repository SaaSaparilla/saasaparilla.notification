/*
 This file exists to give a binding for integration tests.  It is not used in any other context.
*/
#![warn(clippy::all)]

pub use self::paths::notification::app;

mod paths;
mod routes;
