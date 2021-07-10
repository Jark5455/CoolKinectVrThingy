use nite2_sys;
use nite2_sys::NiteUserTracker;
use nite2_sys::NiteUserTrackerHandle;

fn main() {
    let mut userTracker:NiteUserTracker = unsafe {std::mem::zeroed()};
    let mut userTrackerHandle:NiteUserTrackerHandle = &mut userTracker;
    let mut niteRc:nite2_sys::NiteStatus;

    unsafe {
        nite2_sys::niteInitialize();
        niteRc = nite2_sys::niteInitializeUserTracker(&mut userTrackerHandle);
    }

    if (niteRc != nite2_sys::NITE_STATUS_OK){
        println!("Couldn't create user tracker");
        std::process::exit(3);
    }
}
