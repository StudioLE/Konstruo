## Building from source

Konstruo is built with Rust so it's cross platform by default.

0. Install Rust

Follow the instructions to [install Rust](https://www.rust-lang.org/tools/install) for your platform.

1. Clone the repo

Clone the repository and enter the directory:

```bash
git clone git@github.com:StudioLE/Konstruo.git
cd Konstruo
```

2. Build a release binary with cargo

By default Konstruo is built with Bevy's [dynamic linking(https://bevy-cheatbook.github.io/setup/bevy-config.html#dynamic-linking). This makes recompilation much quicker for development but it creates a dependency on shared libraries so the generated binary isn't standalone.

Compile a binary **with** dynamic linking:

```bash
cargo build --release
```

Compile a standalone binary **without** dynamic linking:

```bash
cargo build --no-default-features --release
```

3. Run the compiled binary

Cargo compiles binaries to `target/release` therefore to run the binary directly : 

On Linux/Mac:

```bash
./target/release/konstruo
```

On windows:
```bash
./target/release/konstruo.exe
```

4. Run with Cargo

Run konstruo **with** dynamic linking:

```bash
cargo run --release
```

Run konstruo **without** dynamic linking:

```bash
cargo run --no-default-features --release
```