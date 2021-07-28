#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

mod nite4vr;

use termios::tcgetattr;
use termios::tcsetattr;
use termios::Termios;
use termios::ECHO;
use termios::ICANON;
use termios::TCSANOW;

use libc::fcntl;
use libc::getchar;
use libc::ungetc;
use libc::EOF;
use libc::F_GETFL;
use libc::F_SETFL;
use libc::O_NONBLOCK;
use libc::STDIN_FILENO;

use libc_stdhandle::stdin;

use pyo3::prelude::*;
use pyo3::types::PyBool;
use pyo3::types::PyInt;
use pyo3::types::PyTuple;

use std::env;
use std::mem::MaybeUninit;
use std::thread;

static mut joystick_x: u32 = 130;
static mut joystick_y: u32 = 130;
static mut joystick_c: bool = false;
static mut joystick_z: bool = false;

fn main() {    
    CoolKinectVrThingy();
}

#[no_mangle]
pub extern "C" fn CoolKinectVrThingy() {
    thread::spawn(|| unsafe { nite4vr::startTracking() });

    unsafe {thread::spawn(|| {startNunchukListener(&mut joystick_x, &mut joystick_y, &mut joystick_c, &mut joystick_z,).expect("Error occured when starting Nunchuk Listener")})};
}

fn startNunchukListener(x: &mut u32, y: &mut u32, c: &mut bool, z: &mut bool) -> PyResult<()> {
    env::set_var("BLINKA_MCP2221", "1");

    Python::with_gil(|py| -> PyResult<()> {
        let board = PyModule::import(py, "board")?;
        let adafruit_nunchuk = PyModule::import(py, "adafruit_nunchuk")?;

        let uart = board.getattr("I2C")?.call0()?;
        let nc = adafruit_nunchuk.getattr("Nunchuk")?.call1((uart,))?;

        loop {
            let joystick = nc.getattr("joystick").expect("No attribute \"joystick\" exists").downcast::<PyTuple>().expect("Error when downcasting \"joystick\"");

            *x = joystick.get_item(0 as usize).downcast::<PyInt>().expect("Error when downcasting \"x\"").extract::<u32>().expect("Error when converting \"x\" to u32");
            *y = joystick.get_item(1 as usize).downcast::<PyInt>().expect("Error when downcasting \"y\"").extract::<u32>().expect("Error when converting \"y\" to u32");

            let buttons = nc.getattr("buttons").expect("No attribute \"buttons\" exists");

            *c = buttons.getattr("C").expect("No attribute \"C\" exists").downcast::<PyBool>().expect("Error when downcasting \"C\"").extract::<bool>().expect("Error when converting \"C\" to bool");
            *z = buttons.getattr("Z").expect("No attribute \"Z\" exists").downcast::<PyBool>().expect("Error when downcasting \"Z\"").extract::<bool>().expect("Error when converting \"Z\" to bool");
        }
    })
}

fn wasKeyboardHit() -> bool {
    let mut oldt: MaybeUninit<Termios> = MaybeUninit::<Termios>::uninit();
    let mut newt: Termios;

    let ch: i32;
    let oldf: i32;

    tcgetattr(STDIN_FILENO, unsafe { oldt.as_mut_ptr().as_mut().unwrap() }).expect("Unable to set attribute");

    newt = unsafe { *oldt.as_ptr() };
    newt.c_lflag &= !(ICANON | ECHO);
    tcsetattr(STDIN_FILENO, TCSANOW, &newt).expect("Unable to set attribute");

    oldf = unsafe { fcntl(STDIN_FILENO, F_GETFL) };

    if (unsafe { fcntl(STDIN_FILENO, F_SETFL, oldf | O_NONBLOCK) } != 0) {
        return false;
    }

    ch = unsafe { getchar() };

    tcsetattr(STDIN_FILENO, TCSANOW, unsafe {oldt.as_mut_ptr().as_mut().unwrap()}).expect("Unable to set attribute");

    if unsafe { fcntl(STDIN_FILENO, F_SETFL, oldf) } != 0 {
        return false;
    }

    if ch != EOF {
        unsafe { ungetc(ch, stdin()) };
        return true;
    }

    return false;
}

#[no_mangle]
pub extern "C" fn deactivate() {
    unsafe { nite4vr::niteShutdown() };
}

#[no_mangle]
pub extern "C" fn getJoystickX() -> u32 {
    return unsafe { joystick_x };
}

#[no_mangle]
pub extern "C" fn getJoystickY() -> u32 {
    return unsafe { joystick_y };
}

#[no_mangle]
pub extern "C" fn getJoystickC() -> bool {
    return unsafe { joystick_c };
}

#[no_mangle]
pub extern "C" fn getJoystickZ() -> bool {
    return unsafe { joystick_z };
}

#[no_mangle]
pub extern "C" fn getHeadX() -> f32 {
    return unsafe { nite4vr::head.getPosition().x };
}

#[no_mangle]
pub extern "C" fn getHeadY() -> f32 {
    return unsafe { nite4vr::head.getPosition().y };
}

#[no_mangle]
pub extern "C" fn getHeadZ() -> f32 {
    return unsafe { nite4vr::head.getPosition().z };
}

#[no_mangle]
pub extern "C" fn getLeftHandX() -> f32 {
    return unsafe { nite4vr::left_hand.getPosition().x };
}

#[no_mangle]
pub extern "C" fn getLeftHandY() -> f32 {
    return unsafe { nite4vr::left_hand.getPosition().y };
}

#[no_mangle]
pub extern "C" fn getLeftHandZ() -> f32 {
    return unsafe { nite4vr::left_hand.getPosition().z };
}

#[no_mangle]
pub extern "C" fn getRightHandX() -> f32 {
    return unsafe { nite4vr::right_hand.getPosition().x };
}

#[no_mangle]
pub extern "C" fn getRightHandY() -> f32 {
    return unsafe { nite4vr::right_hand.getPosition().y };
}

#[no_mangle]
pub extern "C" fn getRightHandZ() -> f32 {
    return unsafe { nite4vr::right_hand.getPosition().z };
}

#[no_mangle]
pub extern "C" fn getLeftElbowX() -> f32 {
    return unsafe { nite4vr::left_elbow.getPosition().x };
}

#[no_mangle]
pub extern "C" fn getLeftElbowY() -> f32 {
    return unsafe { nite4vr::left_elbow.getPosition().y };
}

#[no_mangle]
pub extern "C" fn getLeftElbowZ() -> f32 {
    return unsafe { nite4vr::left_elbow.getPosition().z };
}

#[no_mangle]
pub extern "C" fn getRightElbowX() -> f32 {
    return unsafe { nite4vr::left_elbow.getPosition().x };
}

#[no_mangle]
pub extern "C" fn getRightElbowY() -> f32 {
    return unsafe { nite4vr::left_elbow.getPosition().y };
}

#[no_mangle]
pub extern "C" fn getRightElbowZ() -> f32 {
    return unsafe { nite4vr::left_elbow.getPosition().z };
}