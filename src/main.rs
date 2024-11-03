#[cfg(target_os = "windows")]
mod winsys;

mod logging;
#[cfg(target_os = "macos")]
// You can conditionally include modules based on a cfg directive. That way you don't need to use cfg directives in that code.1mod logging;
mod macsys;
use logging::*;

#[cfg(target_os = "windows")]
use winsys::*;

#[cfg(target_os = "macos")]
use macsys::*;

#[macro_use]
// This directive is necessary to be able to use all of the tracing macros without directly importing them individually.
extern crate tracing;

fn main() {
    logs();
    tempclean();
}
