use x86_64::instructions::hlt;

pub extern "C" fn proot() -> ! {
    loop {
        log::info!("looping");
        hlt();
    }
}
