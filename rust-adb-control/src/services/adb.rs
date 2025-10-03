use crate::utilities::printing_helper::*;
use std::process::Command;
use std::{thread, time::Duration};

// Device & ADB
const ADB_CMD: &str = "adb";

// Shell commands
const SHELL: &str = "shell";
const MONKEY: &str = "monkey";
const INPUT: &str = "input";
const KEYEVENT: &str = "keyevent";
const SWIPE: &str = "swipe";
const AM: &str = "am";
const TAP: &str = "tap";
const TEXT: &str = "text";
// const DUMPSYS: &str = "dumpsys";
// const INPUT_METHOD: &str = "input_method";

// App-specific
const LINKEDIN_URL: &str = "https://www.linkedin.com/in/minanoshywahba";
const VIEW_INTENT: &str = "android.intent.action.VIEW";
const LAUNCHER_INTENT: &str = "android.intent.category.LAUNCHER";
const WHATSAPP_PACKAGE: &str = "com.whatsapp";

pub struct Adb {
    device_ip: String,
    device_port: String,
}

impl Adb {
    pub fn new() -> Self {
        Self {
            device_ip: String::new(),
            device_port: String::new(),
        }
    }

    pub fn connect_device(&mut self, ip: &str, port: &str, wait_sec: u64) {
        PRINTER.print_success(&format!("Connecting to device {}:{}", ip, port));

        self.device_ip = ip.to_string();
        self.device_port = port.to_string();

        if wait_sec > 0 {
            std::thread::sleep(std::time::Duration::from_secs(wait_sec));
        }

        PRINTER.print_success("Device connected!");
        PRINTER.print_success(&format!("{}:{}", self.device_ip, self.device_port));
    }

    pub fn run_adb(&self, args: &[&str], wait_sec: u64) {
        if wait_sec > 0 {
            thread::sleep(Duration::from_secs(wait_sec));
        }

        Command::new(ADB_CMD)
            .args(&["-s", &format!("{}:{}", self.device_ip, self.device_port)])
            .args(args)
            .output()
            .expect("Failed to execute ADB command");

        // let stdout = String::from_utf8_lossy(&output.stdout);
        // let stderr = String::from_utf8_lossy(&output.stderr);

        // if !stdout.is_empty() {
        //     PRINTER.print_success(&stdout);
        // }
        // if !stderr.is_empty() {
        //     PRINTER.print_error(&stderr);
        // }
    }

    pub fn wake_device(&self, wait_sec: u64) {
        PRINTER.print_success("Waking up device...");
        self.run_adb(&[SHELL, INPUT, KEYEVENT, "224"], wait_sec);
    }

    pub fn swipe_up(
        &self,
        start_x: u32,
        start_y: u32,
        end_x: u32,
        end_y: u32,
        duration_ms: u32,
        wait_sec: u64,
    ) {
        PRINTER.print_success(&format!(
            "Swiping up from (x: {}, y: {}) to (x: {}, y: {}) over {} ms...",
            start_x, start_y, end_x, end_y, duration_ms
        ));
        self.run_adb(
            &[
                SHELL,
                INPUT,
                SWIPE,
                &start_x.to_string(),
                &start_y.to_string(),
                &end_x.to_string(),
                &end_y.to_string(),
                &duration_ms.to_string(),
            ],
            wait_sec,
        );
    }

    pub fn swipe_up_default(&self, wait_sec: u64) {
        self.swipe_up(540, 2200, 540, 600, 700, wait_sec);
    }

    pub fn type_text(&self, text: &str, wait_sec: u64) {
        PRINTER.print_success(&format!("Typing '{}'...", text));
        let safe_text = &text.replace(" ", "%s");
        self.run_adb(&[SHELL, INPUT, TEXT, &safe_text], wait_sec);
    }

    pub fn open_linkedin(&self, wait_sec: u64) {
        PRINTER.print_success("Opening LinkedIn profile...");
        self.run_adb(
            &[SHELL, AM, "start", "-a", VIEW_INTENT, "-d", LINKEDIN_URL],
            wait_sec,
        );
    }

    pub fn open_whatsapp(&self, wait_sec: u64) {
        PRINTER.print_success("Opening WhatsApp...");
        self.run_adb(
            &[
                SHELL,
                MONKEY,
                "-p",
                WHATSAPP_PACKAGE,
                "-c",
                LAUNCHER_INTENT,
                "1",
            ],
            wait_sec,
        );
    }

    pub fn tap_screen(&self, x: u32, y: u32, wait_sec: u64) {
        PRINTER.print_success(&format!("Tapping screen at (x: {}, y: {})...", x, y));
        self.run_adb(
            &[SHELL, INPUT, TAP, &x.to_string(), &y.to_string()],
            wait_sec,
        );
    }

    pub fn brute_force_password(
        &self,
        passwords: &[&str],
        press_enter_after_input: bool,
        wait_sec: u64,
    ) {
        PRINTER.print_success("Starting brute-force password attempt...");
        for &pwd in passwords {
            PRINTER.print_success(&format!("Trying password: {}", pwd));
            // Input the password
            self.type_text(pwd, wait_sec);

            // Press "Enter" to submit the password (keycode 66 = Enter)
            if press_enter_after_input {
                self.press_enter(0);
            }

            // if !is_keyboard_visible() {
            //     PRINTER.print_success(&format!("Password found: {}", pwd));
            //     break;
            // }
        }
    }

    pub fn press_enter(&self, wait_sec: u64) {
        PRINTER.print_success("Pressing Enter key...");
        self.run_adb(&[SHELL, INPUT, KEYEVENT, "66"], wait_sec);
    }

    // pub fn is_keyboard_visible(&self) -> bool {
    //     PRINTER.print_success("Checking if keyboard is visible...");
    //     let output = Command::new(ADB_CMD)
    //         .args(&[
    //             "-s",
    //             &format!("{}:{}", self.device_ip, self.device_port),
    //             SHELL,
    //             DUMPSYS,
    //             INPUT_METHOD,
    //         ])
    //         .output();

    //     let output = match output {
    //         Ok(o) => o,
    //         Err(_) => return false,
    //     };

    //     let stdout = String::from_utf8_lossy(&output.stdout);
    //     let stderr = String::from_utf8_lossy(&output.stderr);
    //     let text = format!("{}{}", stdout, stderr);

    //     // Scan each line for the exact keyword
    //     for line in text.lines() {
    //         if line.trim().eq_ignore_ascii_case("mInputShown=true") {
    //             return true;
    //         }
    //     }

    //     false
    // }
}
