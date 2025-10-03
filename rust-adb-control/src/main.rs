mod services;
mod utilities;

use services::adb::*;
use utilities::io_helper::*;

const PASSWORD_LIST: &[&str] = &["123456", "789012", "654321", "654621"];

fn main() {
    let mut adb: Adb = Adb::new();
    let input: Input = Input;

    let ip = input.read_line("Enter device IP:");
    let port = input.read_line("Enter device port:");

    adb.connect_device(ip.as_str(), port.as_str(), 1);

    adb.wake_device(1);
    adb.swipe_up_default(1);
    adb.brute_force_password(PASSWORD_LIST, false, 1);
    adb.open_whatsapp(1);
    adb.tap_screen(600, 2100, 1);
    adb.brute_force_password(PASSWORD_LIST, false, 1);
    adb.tap_screen(400, 300, 1);
    adb.type_text("Mina", 1);
    adb.tap_screen(400, 600, 1);
    adb.type_text("Hello Mina From *Rust*.", 1);
    adb.tap_screen(950, 2250, 1);
    adb.open_linkedin(5);
}
