# Lessons

Append-only. Symptom / Cause / Fix / Tell. Read before diagnosing, not after.

## 2026-10-12 — Windows oracle skips SetThreadDescription

- **Symptom** — `thread_name_matches` leaves the Rust thread named `obs-test` and the oracle thread with an empty description.
- **Cause** — `oracle/threading_windows.c` included `windows.h` before `UNICODE` / `_UNICODE`. Include guards then kept `LoadLibrary(L"KernelBase.dll")` as `LoadLibraryA`, the load failed, and `SetThreadDescription` never ran. A real OBS build defines `UNICODE`.
- **Fix** — define `UNICODE` and `_UNICODE` before the include, and do not include `windows.h` ahead of them.
- **The tell** — the Rust side reads the name back. Only the oracle side is empty. A crash in the `0x406D1388` exception is a different bug; this one returns normally.

## 2026-10-12 — `w"..."` does not exist in Rust

- **Symptom** — `prefix w is unknown` in a file that compiled a wide string as `w"KernelBase.dll"`.
- **Cause** — Rust has no wide-string literal, including edition 2024. The parser treats `w` as an identifier.
- **Fix** — `text.encode_utf16().chain(core::iter::once(0))`. `c"..."` is the narrow C-string literal.
- **The tell** — the error is a parse error at the `w`, not a type error about `u16`.

## 2026-10-12 — `cargo test -- threading` runs nothing

- **Symptom** — `soldr cargo test -p obs-util -- threading` reports 0 passed and filters every threading test.
- **Cause** — the filter matches the test function name. None of these functions contain `threading`.
- **Fix** — filter on the binary (`--test threading_windows`) or on a function name such as `thread_name`.
- **The tell** — the summary says `filtered out` for the tests you expected to run.

## 2026-10-12: Seat a per-OS port on the platform boundary

- **Symptom:** `ban_platform_cfg_outside_boundary` rejects `#[cfg(unix)]` in `threading.rs`, or a Windows link fails with duplicate `os_event_*` / `os_sem_*` symbols.
- **Cause:** #326 allows host cfg only under `src/platform/` and `src/ffi/`. #353 stopped compiling raw `threading-posix.c`. The MSVC oracle still compiled raw `threading-windows.c` beside `oracle/threading_windows.c`.
- **Fix:** One event and semaphore per OS under `platform/`, re-exported from `threading.rs` with no host cfg. Rename the oracle symbols on Windows too, and do not compile the raw `threading-windows.c`.
- **The tell:** A safe core still has `#[cfg(windows)]`, or `build.rs` still lists `.file(.../threading-windows.c)` next to the `oracle_` wrapper.

## 2026-10-12: macOS Tier 2 dies in configure

- **Symptom:** `rust/tools/tier2-validate/validate.sh` exits during configure. First `Required submodule 'obs-browser' not available` at `plugins/CMakeLists.txt:20`. After that, `Could not find a package configuration file provided by "CMocka"`.
- **Cause:** `plugins/obs-browser` and `plugins/obs-websocket` are empty gitlinks until initialized. `linux-validate` does `git submodule update --init --depth 1` for both. The macOS script does not. `brew --prefix cmocka` still prints `/opt/homebrew/opt/cmocka` when the formula is not installed, so `CMAKE_PREFIX_PATH` points at a directory that does not exist. The script is also not executable (`permission denied` if you run it directly).
- **Fix:** `git submodule update --init --depth 1 plugins/obs-browser plugins/obs-websocket`, then `brew install cmocka`, then `bash rust/tools/tier2-validate/validate.sh`. Confirm the brew prefix directory exists before configure.
- **The tell:** configure gets through FFmpeg under `.deps` and dies in `test/cmocka/CMakeLists.txt`, or it dies at the obs-browser submodule check. `ls $(brew --prefix cmocka)` fails.

## 2026-10-12: macOS Tier 2 aborts inside Homebrew cmocka

- **Symptom:** every cmocka test is `Subprocess aborted` before any `[ RUN ]` line. dyld reports `libcmocka.0.dylib` is not valid for use in process: `mapping process and mapped file (non-platform) have different Team IDs`.
- **Cause:** the `macos-ci` RelWithDebInfo build signs each test ad-hoc with the hardened runtime. Homebrew's cmocka dylib is a different ad-hoc signature, so library validation refuses it. Re-signing the tests so they honor `DYLD_*` is not enough on this OS: `/opt/homebrew/bin/ctest` is linker-signed, and dyld strips `DYLD_FRAMEWORK_PATH` and `DYLD_LIBRARY_PATH` out of its environment before the tests start. A shell wrapper on `PATH` loses the variables the same way, at `/bin/bash`.
- **Fix:** before the real ctest, re-sign each test executable ad-hoc with `com.apple.security.cs.disable-library-validation`, and `install_name_tool -add_rpath` the `libobs.framework` directory and the obs-deps `lib` directory onto the test. Absolute rpaths, so ctest's per-test working directory does not matter. The macos-26 CI job has not needed this.
- **The tell:** dyld's tried list is only `@executable_path/../Frameworks`, or the mapped file is `Cellar/cmocka/.../libcmocka.1.0.2.dylib`. `codesign -dv` on the test shows `flags=0x10002(adhoc,runtime)`.

## 2026-10-12: threading-windows generator expression was unclosed

- **Symptom:** Windows CMake configure fails with `expected '>' to close '$<'` once `ENABLE_RUST_LIBOBS` is on.
- **Cause:** `os-windows.cmake` dropped the final `>` on the `threading-windows.c` generator expression. The `>>` before the colon only closes `BOOL` and `NOT`. The outer `$<...>` still needs a `>` after the filename. The `pipe-windows.c` line has it.
- **Fix:** end the line with `threading-windows.c>`.
- **The tell:** the pipe line ends in `.c>` and the threading line ends in `.c`.

## 2026-10-12: clang-format on a new rust .c file

- **Symptom:** the local-gate lint lane is green and CI `check-format` fails with `rust/.../*.c requires formatting changes`.
- **Cause:** `ci/local_gate.py` formats only `libobs`, `frontend`, `plugins`, `deps`, `shared`, and `test`, the same default glob as `build-aux/run-clang-format`. The CI action passes every changed `.c`/`.h` on the command line, and then the script formats those paths even when they sit under `rust/`.
- **Fix:** run `build-aux/run-clang-format --fail-error --check` on the changed C files under `rust/` before the stamp. clang-format 22.1.3.
- **The tell:** the failing path starts with `rust/` and the lint log says `clang-format --dry-run` over the libobs set only.

## 2026-10-14: windows-gnu profiler_parity access violation

- **Symptom:** `profiler_parity-*.exe` for `x86_64-pc-windows-gnu` exits `0xC0000005`. The fault is a read in `pthread_getspecific` at an address whose top 32 bits are zero. The same binary passes under Wine, and under a debugger.
- **Cause:** libgcc emutls calls `pthread_getspecific` and loads `pthread_key_t` with a 32-bit `mov`. The oracle's w32-pthreads defines that symbol with a pointer-sized key. The linker bound emutls to w32-pthreads. A low 4GB heap (Wine, or the Windows debug heap) makes the truncated pointer still valid.
- **Fix:** on `windows-gnu`, rename the oracle's `pthread_*`, `sem_*`, and `sched_*` symbols to `oracle_*` and link libwinpthread for the unprefixed names. `-include sys/stat.h` lets GCC 15 compile `platform.h`.
- **The tell:** `nm` shows `oracle_pthread_getspecific` as a real function and `pthread_getspecific` as a jmp to `__imp_pthread_getspecific`. A fault address below 4GB while other pointers in the crash context are above 4GB is this truncation, not a null mutex.

## 2026-10-14: `soldr cargo dylint` rustup storm on Windows

- **Symptom:** `soldr cargo dylint --all -- --workspace --lib --bins` prints only `soldr: cargo diagnostic capture still running after 60s` and never starts `rustc`. A second run floods the machine with short-lived `rustup.exe` processes. `Get-Process` and `tasklist` then hang. `taskkill /F /IM rustup.exe` reports "no running instance" for PIDs that already exited.
- **Cause:** not fully pinned. The driver soldr downloaded is a PE image named `dylint-driver` with no `.exe` suffix (`C:\Users\NV\.soldr\dylint\drivers\nightly-2026-05-28-x86_64-pc-windows-msvc\`). Copying it to `dylint-driver.exe` did not stop the storm. soldr's `rustup.exe` shim is `soldr.exe` itself, and `cargo-dylint` invokes `rustup` while soldr is capturing cargo stdout.
- **Fix:** do not retry `soldr cargo dylint` on this host. Kill the tree with `taskkill /F /IM rustup.exe`, then `cargo-dylint.exe` and `cargo.exe`. Leave Dylint to GitHub CI. Clippy and `cargo test` still run.
- **The tell:** heartbeats every 60 seconds and a flood of `rustup.exe`, with no `dylint-driver` and no `rustc`. A live Dylint run has both. The `.exe` copy is not the missing piece.

## 2026-10-14: Apple SDK unpack dies on MAX_PATH

- **Symptom:** `soldr cargo clippy --target aarch64-apple-darwin` ends with `apple SDK unavailable for aarch64-apple-darwin: archive error: tar unpack: The filename or extension is too long. (os error 206)` and `blessed Apple SDK preparation did not produce an SDK`. The log is full of `tar symlink fallback` / `A required privilege is not held by the client. (os error 1314)`.
- **Cause:** Windows will not create the SDK symlinks without the privilege, so soldr copies them, and a copied path then exceeds MAX_PATH. The staging directory is `C:\Users\NV\.soldr\bin\apple-sdk\14.5\.staging-<pid>-*`.
- **Fix:** leave the Darwin cross targets to GitHub CI on this host. `x86_64-pc-windows-msvc` clippy and `cargo check --tests` still run.
- **The tell:** error 206 after a long run of error 1314 symlink copies. A compile error names a `.rs` file. The MSVC target in the same loop can already have printed `OK`.
