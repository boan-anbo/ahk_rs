//! Send strings as keys to Windows

use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::GetMessageExtraInfo;
use crate::CBSIZE;


fn send_keys(virutal_keys: &[VIRTUAL_KEY]) -> i32 {

    // send 0 if failed and 1 if success, try and catch error
    unsafe {
        let extra_info = GetMessageExtraInfo();
        let extra_info = extra_info.0.unsigned_abs();

        let mut pinputs = Vec::new();
        for virutal_key in virutal_keys {
            pinputs.push(INPUT {
                r#type: INPUT_TYPE(1),
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: *virutal_key,
                        wScan: 0,
                        dwFlags: KEYBD_EVENT_FLAGS(0), // the kind of keyboard event
                        time: 0, // supposedly time interval?
                        dwExtraInfo: extra_info,
                    }
                },
            });
        }
        SendInput(&pinputs, CBSIZE); // CBSIZE is a static variable, indicating the memory size.
    }
    1
}
/// send_string as virutal keys and send them
pub fn send_string(string: &str) -> i32 {
    // create new slice for the chars
    let mut virutal_keys = Vec::new();
    unsafe {
        for c in string.chars() {
            let virutal_key = VIRTUAL_KEY(VkKeyScanW(c as u16) as u16);
            virutal_keys.push(virutal_key);
        }
        println!("string: {}", string);
        send_keys(virutal_keys.as_slice())
    }
    // print the strings
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        send_keys(&[VK_B, VK_O, VK_A, VK_N]);

    }

    #[test]
    fn test_string() {
        send_string("Hello, Little Baby");
    }
}
