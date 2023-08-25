//! # Windows Native Rust Library
//!
//! The **Windows-Native** Rust library provides a convenient and safe way to
//! access the native Windows undocumented APIs using the Rust programming
//! language. These APIs are mostly exported from the Process Hacker native API
//! headers (phnt), enabling you to interact with Windows internals in a
//! reliable and efficient manner.
//!
//! Please note that using undocumented APIs can be risky, as they might change
//! without notice in different Windows versions and can potentially cause
//! system instability. Use this library with caution and ensure you have a good
//! understanding of the implications of using undocumented APIs.
//!
//! ## Features
//!
//! - Access undocumented Windows APIs through Rust.
//! - Headers sourced mainly from Process Hacker's NT headers.
//! - Provides a safer interface compared to raw FFI.
//! - Detailed documentation and examples for each API.
//! - Easy-to-use functions and types for common Windows tasks.
//!
//! ## Installation
//!
//! Add this library to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! windows-native = "1.0.40"
//! ```
//! or run this command
//!
//! ```text
//! cargo add windows-native
//! ```
//!
//! and then install [windows-rs](https://github.com/microsoft/windows-rs)
//!
//! ## Usage
//!
//! ```no_run
//! # fn main() {
//! use std::{thread, time::Duration};
//!
//! use windows_native::ntpsapi::{NtResumeProcess, NtSuspendProcess};
//! use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};
//!
//! let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, 69420).unwrap() };
//! let result = unsafe { NtSuspendProcess(handle) };
//! println!("Result {:?}", result);
//! thread::sleep(Duration::from_secs(3));
//! let result = unsafe { NtResumeProcess(handle) };
//! println!("Result {:?}", result);
//! # }
//! ```
//!
//! ## Contributing
//!
//! Contributions are welcome! If you find a bug or want to add new features to
//! the library, please open an issue or submit a pull request.
//!
//! ## Disclaimer
//!
//! **Windows-Native** is provided as-is and does not guarantee compatibility
//! with future Windows versions. Using undocumented APIs can have unintended
//! consequences, including system instability and security vulnerabilities. Use
//! at your own risk.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::too_many_arguments)]
#![warn(clippy::cargo)]

pub mod bitfield;
pub mod ntbcd;
pub mod ntdbg;
pub mod ntexapi;
pub mod ntgdi;
pub mod ntimage;
pub mod ntioapi;
pub mod ntkeapi;
pub mod ntldr;
pub mod ntlpcapi;
pub mod ntmisc;
pub mod ntmmapi;
pub mod ntnls;
pub mod ntobapi;
pub mod ntpebteb;
pub mod ntpfapi;
pub mod ntpnpapi;
pub mod ntpoapi;
pub mod ntpsapi;
pub mod ntregapi;
pub mod ntrtl;
pub mod ntsam;
pub mod ntseapi;
pub mod ntsmss;
pub mod ntsxs;
pub mod nttmapi;
pub mod nttp;
pub mod ntwow64;
pub mod ntxcapi;
pub mod ntzwapi;
pub mod phnt_ntdef;
pub mod subprocesstag;
pub mod winsta;
