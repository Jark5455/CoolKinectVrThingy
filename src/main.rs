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
use std::time;

fn main() {
    
    thread::spawn(|| unsafe { nite4vr::startTracking() });
    
    static mut x: u32 = 130;
    static mut y: u32 = 130;
    static mut c: bool = false;
    static mut z: bool = false;
    
    unsafe {thread::spawn(|| startNunchukListener(&mut x, &mut y, &mut c, &mut z).expect("Error occured when starting Nunchuk Listener"))};

    while !wasKeyboardHit() {
        unsafe {
            if nite4vr::head.getPosition().x != 0 as f32 || nite4vr::head.getPosition().y != 0 as f32 || nite4vr::head.getPosition().z != 0 as f32 {
                println!("Head: ({}, {}, {})", nite4vr::head.getPosition().x, nite4vr::head.getPosition().y, nite4vr::head.getPosition().z);
            }
            
            if x != 130 || y != 130 {
                println!("Joystick {} {}", x, y);
            }
            
            if c {
                println!("Joystick C");
            }
            
            if z {
                println!("Joystick Z");
            }
            
            thread::sleep(time::Duration::from_millis(34));
        }
    }

    unsafe { nite4vr::niteShutdown() };
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

    tcsetattr(STDIN_FILENO, TCSANOW, unsafe { oldt.as_mut_ptr().as_mut().unwrap()}).expect("Unable to set attribute");

    if unsafe { fcntl(STDIN_FILENO, F_SETFL, oldf) } != 0 {
        return false;
    }

    if ch != EOF {
        unsafe { ungetc(ch, stdin()) };
        return true;
    }

    return false;
}