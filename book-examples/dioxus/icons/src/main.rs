mod app;

#[cfg(feature = "icons")]
mod icons;

use crate::app::App;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    dioxus::launch(App);
}
