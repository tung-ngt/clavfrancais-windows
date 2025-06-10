use core::mem::size_of;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
    KEYEVENTF_UNICODE, VIRTUAL_KEY, VK_BACK,
};

pub struct WindowsInputSimulator;

pub trait InputSimulator {
    fn character(c: char);
    fn backspace();
}

impl InputSimulator for WindowsInputSimulator {
    fn character(c: char) {
        unsafe {
            let input_array = [
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            time: 0,
                            wVk: VIRTUAL_KEY::default(),
                            wScan: c as u16,
                            dwFlags: KEYEVENTF_UNICODE,
                            dwExtraInfo: 1000_usize,
                        },
                    },
                },
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            time: 0,
                            wVk: VIRTUAL_KEY::default(),
                            wScan: c as u16,
                            dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                            dwExtraInfo: 1000_usize,
                        },
                    },
                },
            ];
            let input_size = size_of::<INPUT>().try_into().unwrap();

            SendInput(&input_array, input_size);
        }
    }
    fn backspace() {
        unsafe {
            let input_array = [
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            time: 0,
                            wVk: VK_BACK,
                            wScan: 0_u16,
                            dwFlags: KEYBD_EVENT_FLAGS::default(),
                            dwExtraInfo: 1000_usize,
                        },
                    },
                },
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            time: 0,
                            wVk: VK_BACK,
                            wScan: 0_u16,
                            dwFlags: KEYEVENTF_KEYUP,
                            dwExtraInfo: 1000_usize,
                        },
                    },
                },
            ];
            let input_size = size_of::<INPUT>().try_into().unwrap();

            SendInput(&input_array, input_size);
        }
    }
}
