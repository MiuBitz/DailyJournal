#[link(name = "user32")]
extern "system" {
    fn MessageBeep(uType: u32) -> i32;
}

pub const MB_ICONEXCLAMATION: u32 = 0x00000030;

pub fn play_reminder_sound() {
    unsafe {
        MessageBeep(MB_ICONEXCLAMATION);
        std::thread::sleep(std::time::Duration::from_millis(150));
        MessageBeep(MB_ICONEXCLAMATION);
    }
}
