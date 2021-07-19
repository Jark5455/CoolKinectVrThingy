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

use std::mem::MaybeUninit;
use std::thread;
use std::time;

fn main() {
    thread::spawn(|| {unsafe { nite4vr::startTracking() }});
        
    while (!wasKeyboardHit()) {
        unsafe {
            if (nite4vr::head.getPosition().x != 0 as f32 || nite4vr::head.getPosition().y != 0 as f32 || nite4vr::head.getPosition().z != 0 as f32)
            {                
                println!("({}, {}, {})", nite4vr::head.getPosition().x, nite4vr::head.getPosition().y, nite4vr::head.getPosition().z);                
            }
            
            thread::sleep(time::Duration::from_millis(34));
        }
    }
    
    unsafe {nite4vr::niteShutdown()};
}

fn wasKeyboardHit() -> bool {
    let mut oldt: MaybeUninit<Termios> = MaybeUninit::<Termios>::uninit();
    let mut newt: Termios;

    let ch: i32;
    let oldf: i32;

    tcgetattr(STDIN_FILENO, unsafe { oldt.as_mut_ptr().as_mut().unwrap() });

    newt = unsafe { *oldt.as_ptr() };
    newt.c_lflag &= !(ICANON | ECHO);
    tcsetattr(STDIN_FILENO, TCSANOW, &newt);

    oldf = unsafe { fcntl(STDIN_FILENO, F_GETFL) };

    if (unsafe { fcntl(STDIN_FILENO, F_SETFL, oldf | O_NONBLOCK) } != 0) {
        return false;
    }

    ch = unsafe { getchar() };

    tcsetattr(STDIN_FILENO, TCSANOW, unsafe {
        oldt.as_mut_ptr().as_mut().unwrap()
    });

    if unsafe { fcntl(STDIN_FILENO, F_SETFL, oldf) } != 0 {
        return false;
    }

    if ch != EOF {
        unsafe { ungetc(ch, stdin()) };
        return true;
    }

    return false;
}
