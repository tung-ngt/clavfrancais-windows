use clavfrancais_engine::keys::Key;

pub trait KeyConverter {
    fn to_virtual_key_code(&self) -> Option<u32>;
    fn from_virtual_key_code(virtual_key_code: u32) -> Self;
}

macro_rules! decl_keycodes {
    ($($key:ident, $code:literal),*) => {
        impl KeyConverter for Key {
            fn to_virtual_key_code(&self) -> Option<u32> {
                match self {
                    $(
                        Key::$key => Some($code),
                    )*
                    Key::Unknown(code) => Some(*code),
                    _ => None,
                }
            }

            fn from_virtual_key_code(virtual_key_code: u32) -> Self {
                match virtual_key_code {
                    $(
                        $code => Key::$key,
                    )*
                    _ => Key::Unknown(virtual_key_code)
                }
            }
        }
    };
}

// https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
decl_keycodes! {
    Alt, 0xA4, // 164,
    AltGr, 0xA5, //165,
    Backspace, 0x08, // 8,
    CapsLock, 0x14, // 20,
    ControlLeft, 0xA2, // 162,
    ControlRight, 0xA3, // 163,
    Delete, 0x2E, // 46,
    DownArrow, 0x28, // 40,
    End, 0x23, // 35,
    Escape, 0x1B, // 27,
    F1, 0x70, // 112,
    F10, 0x79, // 121,
    F11, 0x7A, // 122,
    F12, 0x7B, // 123,
    F2, 0x71, // 113,
    F3, 0x72, // 114,
    F4, 0x73, // 115,
    F5, 0x74, // 116,
    F6, 0x75, // 117,
    F7, 0x76, // 118,
    F8, 0x77, // 119,
    F9, 0x78, // 120,
    Home, 0x24, // 36,
    LeftArrow, 0x25, // 37,
    MetaLeft, 0x5B, // 91,
    PageDown, 0x22, // 34,
    PageUp, 0x21, // 33,
    Return, 0x0D, // 0x0D,
    RightArrow, 0x27, // 39,
    ShiftLeft, 0xA0, // 160,
    ShiftRight, 0xA1, // 161,
    Space, 0x20, // 32,
    Tab, 0x09, // 0x09,
    UpArrow, 0x26, // 38,
    PrintScreen, 0x2C, // 44,
    ScrollLock, 0x91, // 145,
    Pause, 0x13, // 19,
    NumLock, 0x90, // 144,
    BackQuote, 0xC0, // 192,
    Key1, 0x31, // 49,
    Key2, 0x32, // 50,
    Key3, 0x33, // 51,
    Key4, 0x34, // 52,
    Key5, 0x35, // 53,
    Key6, 0x36, // 54,
    Key7, 0x37, // 55,
    Key8, 0x38, // 56,
    Key9, 0x39, // 57,
    Key0, 0x30, // 48,
    Minus, 0xBD, // 189,
    Equal, 0xBB, // 187,
    Q, 0x51, // 81,
    W, 0x57, // 87,
    E, 0x45, // 69,
    R, 0x52, // 82,
    T, 0x54, // 84,
    Y, 0x59, // 89,
    U, 0x55, // 85,
    I, 0x49, // 73,
    O, 0x4F, // 79,
    P, 0x50, // 80,
    LeftBracket, 0xDB, // 219,
    RightBracket, 0xDD, // 221,
    A, 0x41, // 65,
    S, 0x53, // 83,
    D, 0x44, // 68,
    F, 0x46, // 70,
    G, 0x47, // 71,
    H, 0x48, // 72,
    J, 0x4A, // 74,
    K, 0x4B, // 75,
    L, 0x4C, // 76,
    SemiColon, 0xBA, // 186,
    Quote, 0xDE, // 222,
    BackSlash, 0xDC, // 220,
    IntlBackslash, 0xE2, // 226,
    Z, 0x5A, // 90,
    X, 0x58, // 88,
    C, 0x43, // 67,
    V, 0x56, // 86,
    B, 0x42, // 66,
    N, 0x4E, // 78,
    M, 0x4D, // 77,
    Comma, 0xBC, // 188,
    Dot, 0xBE, // 190,
    Slash, 0xBF, // 191,
    Insert, 0x2D, // 45,
    NumpadMinus, 0x6D, // 109,
    NumpadPlus, 0x6B, // 107,
    NumpadMultiply, 0x6A, // 106,
    NumpadDivide, 0x6F, // 111,
    Numpad0, 0x60, // 96,
    Numpad1, 0x61, // 97,
    Numpad2, 0x62, // 98,
    Numpad3, 0x63, // 99,
    Numpad4, 0x64, // 100,
    Numpad5, 0x65, // 101,
    Numpad6, 0x66, // 102,
    Numpad7, 0x67, // 103,
    Numpad8, 0x68, // 104,
    Numpad9, 0x69, // 105,
    NumpadDecimal, 0x6E // 110
}

#[cfg(test)]
mod test {
    use super::KeyConverter;
    use crate::keys::Key;

    // use super::{code_from_key, key_from_code};
    #[test]
    fn test_reversible() {
        for code in 0..65535 {
            let key = Key::from_virtual_key_code(code);
            if let Some(code2) = key.to_virtual_key_code() {
                assert_eq!(code, code2)
            } else {
                panic!("We could not convert back code: {:?}", code);
            }
        }
    }
}
