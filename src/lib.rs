#![no_std]

use gstd::{msg, prelude::*};
use hikikomori_io::*;

static mut STATE: Option<Hikikomori> = None;

// The `init()` entry point.
#[no_mangle]
extern fn init() {
    unsafe {
        STATE = Some(Hikikomori {
            device: msg::source(),
            ..Default::default()
        })
    }
}

// The `handle()` entry point.
#[no_mangle]
extern fn handle() {
    let state = unsafe { STATE.as_mut().expect("State isn't initialized") };

    if state.device == msg::source() {
        let payload = msg::load().expect("Failed to load payload");

        match payload {
            HikikomoriAction::AddEnergy => {
                state.energy = state.energy.saturating_add(10);

                msg::reply(state.energy, 0).expect("Failed to reply from `handle()`");
            }
        }
    }
}

// The `state()` entry point.
#[no_mangle]
extern fn state() {
    let state = unsafe { STATE.take().expect("State isn't initialized") };
    msg::reply(state, 0).expect("Failed to reply from `state()`");
}
