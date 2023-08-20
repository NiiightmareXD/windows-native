# Windows Native Rust Library

![License](https://img.shields.io/badge/license-MIT-blue)

The **Windows-Native** Rust library provides a convenient and safe way to access the native Windows undocumented APIs using the Rust programming language. These APIs are mostly exported from the Process Hacker native API headers (phnt), enabling you to interact with Windows internals in a reliable and efficient manner.

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

and then install [windows-rs](https://github.com/microsoft/windows-rs)

## Usage

```rust
use std::{thread, time::Duration};

use windows_native::ntpsapi::{NtResumeProcess, NtSuspendProcess};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};

let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, 69420).unwrap() };
let result = unsafe { NtSuspendProcess(handle) };
println!("Result {:?}", result);
thread::sleep(Duration::from_secs(3));
let result = unsafe { NtResumeProcess(handle) };
println!("Result {:?}", result);
```

## Documentation

Detailed documentation for each API and type can be found [here](https://docs.rs/windows-native/).

## Contributing

Contributions are welcome! If you find a bug or want to add new features to the library, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

## Disclaimer

**Windows-Native** is provided as-is and does not guarantee compatibility with future Windows versions. Using undocumented APIs can have unintended consequences, including system instability and security vulnerabilities. Use at your own risk.
