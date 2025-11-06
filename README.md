```markdown
# first-simple-rust-project

This workspace now includes a minimal Rust project and VS Code configuration to make it easy to develop, build and debug Rust code.

What's included
- `Cargo.toml` and `src/main.rs` — minimal Rust app
- `rust-toolchain.toml` — pins the stable toolchain and enables rustfmt/clippy
- `.vscode/extensions.json` — recommends `rust-analyzer` and `CodeLLDB`
- `.vscode/settings.json` — useful Rust Analyzer settings
- `.vscode/tasks.json` — tasks for `cargo build` and `cargo run`
- `.vscode/launch.json` — debug configuration (uses CodeLLDB)

Quick start

1. Install Rust toolchain (if you don't have it):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

2. Open this folder in VS Code and install the recommended extensions when prompted (or install manually):
- `rust-analyzer` (matklad.rust-analyzer)
- `CodeLLDB` (vadimcn.vscode-lldb)

3. Build and run using VS Code tasks or the terminal:

```bash
cargo build
cargo run
```

4. To debug: run the `Debug executable` configuration in the Run view. It will run `cargo build` first and then launch the built binary under the debugger.

Notes and next steps
- If `cargo` is not available in the environment, follow step (1) to install `rustup`.
- The `rust-toolchain.toml` will ensure the workspace uses the stable channel and enables `rust-src`, `rustfmt`, and `clippy` for a better IDE experience.
- Consider using a devcontainer (`.devcontainer/`) if you want reproducible tooling in the cloud or for contributors.

Enjoy coding in Rust!
```
# first-simple-rust-project