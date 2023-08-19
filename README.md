# Windows Native Rust Library

![License](https://img.shields.io/badge/license-MIT-blue)

The **Windows-Native** Rust library provides a convenient and safe way to access the native Windows undocumented APIs using the Rust programming language. These APIs are mostly exported from the Process Hacker (PH) NT headers, enabling you to interact with Windows internals in a reliable and efficient manner.

Please note that using undocumented APIs can be risky, as they might change without notice in different Windows versions and can potentially cause system instability. Use this library with caution and ensure you have a good understanding of the implications of using undocumented APIs.

## Features

- Access undocumented Windows APIs through Rust.
- Headers sourced mainly from Process Hacker's NT headers.
- Provides a safer interface compared to raw FFI.
- Detailed documentation and examples for each API.
- Easy-to-use functions and types for common Windows tasks.

## Installation

Add this library to your `Cargo.toml`:

```toml
[dependencies]
windows-native = "0.51.1"
```
or run this command

```
cargo add windows-native
```

## Usage

```rust
use windows_native::ntapi::ntdll::NtQuerySystemInformation;
use windows_native::ntapi::ntdef::SYSTEM_INFORMATION_CLASS;
use windows_native::ntapi::ntdef::SYSTEM_PROCESS_INFORMATION;
use windows_native::ntapi::ntstatus::NTSTATUS;

fn main() {
    // Call an undocumented API
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut return_length: u32 = 0;
    let status = unsafe {
        NtQuerySystemInformation(
            SYSTEM_INFORMATION_CLASS::SystemProcessInformation,
            buffer.as_mut_ptr() as *mut _,
            buffer.len() as u32,
            &mut return_length,
        )
    };

    if status == NTSTATUS(0) {
        let process_info = buffer.as_ptr() as *const SYSTEM_PROCESS_INFORMATION;
        // Process the information...
    } else {
        println!("NtQuerySystemInformation failed with status: {:?}", status);
    }
}
```

## Documentation

Detailed documentation for each API and type can be found [here](https://docs.rs/windows-native/).

## Contributing

Contributions are welcome! If you find a bug or want to add new features to the library, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

## Disclaimer

**Windows-Native** is provided as-is and does not guarantee compatibility with future Windows versions. Using undocumented APIs can have unintended consequences, including system instability and security vulnerabilities. Use at your own risk.
