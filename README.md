# Finished tutorial for bindgen

Here is finished "bzip2 bindings tutorial" project from:
- https://rust-lang.github.io/rust-bindgen/tutorial-0.html
- original article (outdated by above "Book"):
  - https://fitzgeraldnick.com/2016/12/14/using-libbindgen-in-build-rs.html

Updates:
- (applies for Blog article only): bumped bindgen version from `0.20.0` to `0.59.2`
  to fix: https://github.com/rust-lang/rust/issues/68729
- (applies for Blog article only) :commented out `.no_unstable_rust()` in `build.rs`
  (it was deprecated and later removed from builder)
- replaced  `../futurama-quotes.txt` with `../README.md`
  in `src/lib.rs` to avoid possible Copyright issues

## Setup for Debian 11


Start with https://rust-lang.github.io/rust-bindgen/requirements.html

```bash
sudo apt install llvm-dev libclang-dev clang libclang-11-dev
# these needed for rustup and building native interfaces (libc-sys)
sudo apt-get install curl gcc libc-dev
# for this tutorial only
sudo apt-get install libbz2-dev
```

If you have not installed rust yet run these commands:
```bash
curl --proto '=https' --tlsv1.2 -sSf -o setup-rust.sh https://sh.rustup.rs
# review setup script
less setup-rust.sh
# run it
chmod +x setup-rust.sh
./setup-rust.sh
# accept defaults
# run this command to update our PATH
source ~/.bashrc
```

Here are rustc and cargo versions used

```bash
$ rustc -V

rustc 1.60.0 (7737e0b5c 2022-04-04)

$ cargo -V

cargo 1.60.0 (d1fd9fe 2022-03-01)
```

## Setup for openSUSE 15.3

After endless package conflicts (where `rustfmt` package
wanted other version of `cargo` package than `rls`, etc...)
I eneded up using just `rustup` (as in Debian) + clang for bindgen

Install these packages:
```bash
sudo zypper in clang-devel llvm-devel libbz2-devel rustup
```

If you have not yet setup `rustup` then run:
```bash
rustup-init
```
Accept defaults and you will have to `source ~/.bashrc` to get
proper Paths.


These versions were tested:
```bash
$ rustc -V

rustc 1.60.0 (7737e0b5c 2022-04-04)

$ cargo -V

cargo 1.60.0 (d1fd9fe 2022-03-01)
```

## Building and testing

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

If above file is compressed you can format it using
`rustfmt` (should be already available when using `rustup`):
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

