A reproducer for https://github.com/rust-lang/rust/issues/164915 .

Builds on MacOS if `lto = false` is set in `Cargo.toml`, fails with `lto = true` with

```
$ cargo build -r
   Compiling link_fail_reproducer v0.1.0 (/Users/pestix/git/link_fail_reproducer)
error: linking with `cc` failed: exit status: 1
  |
  = note:  "cc" "<1 object files omitted>" "-lnative" "<sysroot>/lib/rustlib/aarch64-apple-darwin/lib/libcompiler_builtins-*.rlib" "-lSystem" "-lc" "-lm" "-arch" "arm64" "-mmacosx-version-min=11.0.0" "-L" "/opt/homebrew/lib" "-L" "/Users/pestix/git/link_fail_reproducer/target/release/build/link_fail_reproducer-c02b07d5378bc5ea/out" "-o" "/Users/pestix/git/link_fail_reproducer/target/release/deps/link_fail_reproducer-b916688ce59f10ee" "-Wl,-dead_strip" "-nodefaultlibs"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: Undefined symbols for architecture arm64:
            "___isPlatformVersionAtLeast", referenced from:
                _check_ventura in libnative.a[2](d56d5010e2cefa2b-native.o)
          ld: symbol(s) not found for architecture arm64
          clang: error: linker command failed with exit code 1 (use -v to see invocation)


error: could not compile `link_fail_reproducer` (bin "link_fail_reproducer") due to 1 previous error
```