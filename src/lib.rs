// write package level documentation here
//! # Utility program to control Windows with Rust for Windows
//!
//! [Windows Rs: github.com/microsoft/windows-rs](github.com/microsoft/windows-rs)
//!
//! # Current features
//!
//! - Send keyboard input
//!
//! # Future features
//!
//! - To be decided.
//!
pub mod keyboard;

use std::mem::size_of;
use windows::Win32::UI::Input::KeyboardAndMouse::INPUT;

const CBSIZE: i32 = size_of::<INPUT>() as i32;






#[cfg(test)]
mod tests {
    use super::*;


}

