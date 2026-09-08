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

Compile a standalone binary:

```bash
cargo build --release
```

To iterate on the source, [dynamic linking](https://bevy.org/learn/book/development-practices/fast-compiles/) cuts link time:

```bash
cargo build --features bevy/dynamic_linking
```

A binary built this way is not standalone. It needs `libbevy_dylib` beside it, so use it for development only.

3. Run the compiled binary

Cargo compiles binaries to `target/release` therefore to run the binary directly:

On Linux/Mac:

```bash
./target/release/konstruo
```

On Windows:

```bash
./target/release/konstruo.exe
```

4. Run with Cargo

Run konstruo:

```bash
cargo run --release
```

Or with dynamic linking for a faster edit-compile-run loop:

```bash
cargo run --features bevy/dynamic_linking
```

## Building the preprocessor

`konstruo-preprocessor` resamples Environment Agency LIDAR tiles into the terrain height chunks Konstruo loads at runtime. Build it only to regenerate those chunks.

1. Build the binary

```bash
cargo build --release -p konstruo-preprocessor
```

Dynamic linking does not apply here. The preprocessor has no Bevy dependency, so cargo rejects `--features bevy/dynamic_linking`.

2. Generate the height chunks

```bash
cargo run --release -p konstruo-preprocessor -- height-chunks
```

Source files are read from `../konstruo-gis/src/environment-agency` and chunks are written to `assets/terrain`. Both defaults resolve from the repository root, so run it from there or override them with `--source` and `--output`.
