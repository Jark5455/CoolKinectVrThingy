mod NiTE;

use NiTE::nite_UserTracker;
use NiTE::nite_UserTrackerFrameRef;
use NiTE::NiteStatus;
use NiTE::NiteUserTracker;
use NiTE::niteInitialize;
use NiTE::NiteStatus_NITE_STATUS_OK;

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

use std::mem::MaybeUninit;

fn main() {

    let mut niteUserTracker:MaybeUninit<NiteUserTracker> = MaybeUninit::<NiteUserTracker>::uninit();

    let mut userTracker:nite_UserTracker = nite_UserTracker {m_userTrackerHandle: &mut niteUserTracker};
    let mut niteRc:NiteStatus;

    unsafe {
        niteInitialize();
        niteRc = userTracker.create(None);
    }

    if niteRc != NiteStatus_NITE_STATUS_OK {
        println!("Couldn't create user tracker");
        dbg!(niteRc);
        std::process::exit(3);
    }

    println!("Start moving around to get detected...");
    println!("(PSI pose may be required for skeleton calibration, depending on the configuration)");

    let mut userTrackerFrame:MaybeUninit<nite_UserTrackerFrameRef> = MaybeUninit::<nite_UserTrackerFrameRef>::uninit();
    let mut userTrackerFrameRef = userTrackerFrame.as_mut_ptr();

    while(!wasKeyboardHit()){

        niteRc = unsafe {userTracker.readFrame(userTrackerFrameRef)};

        if niteRc != NiteStatus_NITE_STATUS_OK {
            println!("Get next frame failed");
            continue;
        }

        let users = unsafe {std::slice::from_raw_parts(userTrackerFrameRef.as_ref().unwrap().m_users.m_data, userTrackerFrameRef.as_ref().unwrap().m_users.m_size as usize)};
        for i in 0..users.len() {

            let user = users[i];

            updateUserState(user._base, unsafe {(*(userTrackerFrameRef)).m_pFrame.as_mut().unwrap().timestamp});
            if user._base.state & 2 != 0 {
                println!("Attempting to start Skeleton Tracking...");
                userTracker.startSkeletonTracking(user._base.id);
            } else if user._base.skeleton.state == NiTE::NiteSkeletonState_NITE_SKELETON_TRACKED {
                let head:NiTE::NiteSkeletonJoint = user._base.skeleton.joints[0];
                if head.positionConfidence > 0.5 {
                    println!("{}. ({}, {}, {})", user._base.id, head.position.x, head.position.y, head.position.z);
                } else {
                    println!("{}", head.positionConfidence);
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
