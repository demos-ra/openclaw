//! Host programs started from the AppImage must resolve the host's libraries.
//! AppRun puts the bundle's usr/lib (built on Ubuntu 22.04) first in
//! LD_LIBRARY_PATH for the app and its WebKit helpers, which find
//! libwebkit2gtk only through it. A newer host program that loads those
//! copies aborts, as curl did with "OPENSSL_3.2.0 not found" (#146088) and
//! an ssh ProxyCommand does with "OPENSSL_3.3.0 not found", or runs on the
//! bundle's older OpenSSL, as ssh itself does.

use std::env;
use std::process::Command;

/// Gives `command` the host's library search path: inside the AppImage, where
/// APPDIR is set, LD_LIBRARY_PATH is removed, including entries the user set
/// before launching it; outside it nothing changes.
pub(crate) fn use_host_libraries(command: &mut Command) {
    if env::var_os("APPDIR").is_some() {
        command.env_remove("LD_LIBRARY_PATH");
    }
}
