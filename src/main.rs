mod tempclean;

mod logging;
use logging::*;
use tempclean::*;

#[macro_use]
// This directive is necessary to be able to use all of the tracing macros without directly importing them individually.
extern crate tracing;

fn main() {
    logs();
    tempclean();
}
