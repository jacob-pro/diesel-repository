# Tests

A very simple set of tests that use in-memory SQLite.

## Running on Windows

1. Download [SQLite Precompiled Binaries for Windows](https://www.sqlite.org/download.html)
2. Extract to disk, then open the directory in an `x64 Native Tools Command Prompt for VS 2019` shell.
3. Run `link /lib /def:sqlite3.def /MACHINE:X64` to generate the `sqlite3.lib` import library.
4. Add the directory under the `SQLITE3_LIB_DIR` environment variable (to build).
5. Add the directory under the `PATH` environment variable (to run).
