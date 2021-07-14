mod NiTE;

use NiTE::NiteUserData;
use NiTE::NiteUserTracker;
use NiTE::NiteUserTrackerHandle;
use NiTE::NiteStatus;
use NiTE::niteInitialize;
use NiTE::niteInitializeUserTracker;
use NiTE::NiteStatus_NITE_STATUS_OK;
use NiTE::NiteUserTrackerFrame;
use NiTE::niteReadUserTrackerFrame;
use NiTE::niteStartSkeletonTracking;

use termios::Termios;
use termios::tcgetattr;
use termios::tcsetattr;
use termios::ICANON;
use termios::ECHO;
use termios::TCSANOW;

use libc::STDIN_FILENO;
use libc::fcntl;
use libc::getchar;
use libc::ungetc;
use libc::F_GETFL;
use libc::F_SETFL;
use libc::O_NONBLOCK;
use libc::EOF;

use libc_stdhandle::stdin;

fn main() {
    let mut userTracker:NiteUserTracker = unsafe {std::mem::zeroed()};
    let mut userTrackerHandle:NiteUserTrackerHandle = &mut userTracker;
    let mut niteRc:NiteStatus;

    unsafe {
        niteInitialize();
        niteRc = niteInitializeUserTracker(&mut userTrackerHandle);
    }

    if niteRc != NiteStatus_NITE_STATUS_OK {
        println!("Couldn't create user tracker");
        dbg!(niteRc);
        std::process::exit(3);
    }

    println!("Start moving around to get detected...");
    println!("(PSI pose may be required for skeleton calibration, depending on the configuration)");

    let mut userTrackerFrame:NiteUserTrackerFrame = unsafe {std::mem::zeroed()};
    while(!wasKeyboardHit()){
        niteRc = unsafe {niteReadUserTrackerFrame(userTrackerHandle, &mut userTrackerFrame as *mut NiteUserTrackerFrame as *mut *mut NiteUserTrackerFrame)};

        if niteRc != NiteStatus_NITE_STATUS_OK {
            println!("Get next frame failed");
            continue;
        }

        let users = unsafe {std::slice::from_raw_parts(userTrackerFrame.pUser as *mut NiteUserData, userTrackerFrame.userCount as usize)};

        for user in users {
            updateUserState(*user, userTrackerFrame.timestamp);
            if user.state & 2 != 0 {
                println!("Attempting to start Skeleton Tracking...");
                unsafe {niteStartSkeletonTracking(userTrackerHandle, user.id)};
            } else if user.skeleton.state == NiTE::NiteSkeletonState_NITE_SKELETON_TRACKED {
                let head:NiTE::NiteSkeletonJoint = user.skeleton.joints[0];
                if head.positionConfidence > 0.5 {
                    println!("{}. ({}, {}, {})", user.id, head.position.x, head.position.y, head.position.z);
                }
            }
        }
    }

    println!("Keyboard Interupt");
    unsafe {NiTE::niteShutdown()};
}

fn wasKeyboardHit() -> bool {
    let mut oldt:Termios = unsafe {std::mem::zeroed()};
    let mut newt:Termios;

    let ch:i32;
    let oldf:i32;

    tcgetattr(STDIN_FILENO, &mut oldt);

    newt = oldt;
    newt.c_lflag &= !(ICANON | ECHO);
    tcsetattr(STDIN_FILENO, TCSANOW, &newt);

    oldf = unsafe {fcntl(STDIN_FILENO, F_GETFL)};

    if (unsafe {fcntl(STDIN_FILENO, F_SETFL, oldf | O_NONBLOCK)} != 0){
        return false;
    }

    ch = unsafe {getchar()};

    tcsetattr(STDIN_FILENO, TCSANOW, &oldt);
    if unsafe {fcntl(STDIN_FILENO, F_SETFL, oldf)} != 0 {
        return false;
    }

    if ch != EOF {
        unsafe {ungetc(ch, stdin())};
        return true;
    }

    return false;
}

fn updateUserState(user:NiTE::NiteUserData, ts:u64){
    if user.state & 2 != 0 {
        println!("User #{}: New", user.id);
    } else if user.state & 1 != 0 {
        println!("User #{}: Visible", user.id);
    } else if user.state & 1 == 0 {
        println!("User #{}: Out of scene", user.id);
    } else if user.state & 4 != 0 {
        println!("User #{}: Lost", user.id);
    }
}
