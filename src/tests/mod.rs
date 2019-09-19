use std::sync::Once;

pub mod ram;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        env_logger::init();
    });
}
