# Multiple Binary + Library Example

Relatively minimal Rust example of multiple binaries that rely on the same library.
One of the binaries uses the FFI, while the other uses the standard Rust ABI.
The library supports both calling conventions by using different functions.
Note that a real project would use one convention or the other.

# Usage Examples

```sh
cargo run --bin rlib_main a bb ccc de
cargo run --bin ffi_main 1 22 333 45
```

