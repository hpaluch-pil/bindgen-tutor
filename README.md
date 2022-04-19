# Finished tutorial for bindgen

Here is finished "bzip2 bindings tutorial" project from:
- https://fitzgeraldnick.com/2016/12/14/using-libbindgen-in-build-rs.html
- https://rust-lang.github.io/rust-bindgen/tutorial-0.html

Updates:
- bumped bindgen version from `0.20.0` to `0.59.2`
  to fix: https://github.com/rust-lang/rust/issues/68729
- commented out `.no_unstable_rust()` in `build.rs`
  (it was deprecated and later removed from builder)
- replaced  `` with `../README.md` in `src/lib.rs` to avoid possible Copyright issues

## Setup for Debian 11


Start with https://rust-lang.github.io/rust-bindgen/requirements.html

```bash
sudo apt install llvm-dev libclang-dev clang
# additionally these are needed (cargo will install rustc)
apt-get install libclang-11-dev cargo
# for this tutorial only
apt-get install libbz2-dev
```

**WARNING!**

Bindgen and its compoonents are heavily dependent
on specific Rust versions. Here are versions installed on my Debian 11:

```bash
$ rustc -V

rustc 1.48.0

$ cargo -V

cargo 1.46.0
```


Now you can run:
```bash
cargo build
```

Now cross your fingers and run tests:
```bash
cargo test
```

To find generated `bindings.rs` file try:
```bash
find . -name bindings.rs

./target/debug/build/bindgen-ex-d2edd0f08708c93d/out/bindings.rs
```

Unfortunately it is tightly packed. However you can install `rustfmt` and use it to format
above file, for example:

To install rustfmt (I was unable to find DEB package in stock Debian 11 as of 2022-04-19):
```bash
cd
curl -fLO https://github.com/rust-lang/rustfmt/releases/download/v1.4.38/rustfmt_linux-x86_64_v1.4.38.tar.gz
tar xvzf rustfmt_linux-x86_64_v1.4.38.tar.gz 
sudo cp rustfmt_linux-x86_64_v1.4.38/rustfmt /usr/local/bin/
```

And then try command like:
```bash
rustfmt ./target/debug/build/bindgen-ex-d2edd0f08708c93d/out/bindings.rs
```
And look inside with your favorite editor...

## Bonus

You can run this command that query bzlib2 version by calling generated bindings include in `src/lib.rs`:

```bash
$ cargo run

    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/bindgen-ex`
BZlib version is: 1.0.8, 13-Jul-2019
```

Source code is in `src/main.rs`. This is my contribution (so please
don't blame bindgen authors for bugs in this file ;-)

It looks like:
```rust
use std::ffi::CStr;
use std::str::Utf8Error;

fn main() -> Result<(),Utf8Error> {
    let bz_ver_ptr = unsafe { bindgen_ex::BZ2_bzlibVersion() };
    let bz_ver_cstr = unsafe { CStr::from_ptr(bz_ver_ptr) };
    let bz_ver_str = bz_ver_cstr.to_str()?;
    println!("bzip2 version is: '{}'", bz_ver_str);

    Ok(())
}
```




