# Simple Project

A bread-and-butter project with a `pglt.toml` on top level and SQL files in the `test` folder.
You should either let the `.vscode/settings.json` point to a binary OR add a folder containing the `pglt(.exe)` binary to your `PATH`.

## Expectations

The extension should recognize the `pglt.toml` file and connect with the right database. If the global binary is installed, it should not prompt for a local binary.

## Test protocol

### Via $PATH

0. Follow the instructions in `GLOBAL_SETUP.md`.
1. Make sure the `.vscode/settings.json` file does _not_ have a `pglt.bin` setting.
2. Make sure you have a folder containing a `pglt(.exe)` binary in your $PATH.
3. You should not be prompted for any binary installations when you start the extensions.
4. The extension should work when you open the `src/test.sql` file.

### Via VSCode Settings

0. Follow the instructions in `GLOBAL_SETUP.md`.
1. Make sure you have a valid `pglt.bin` configuration in your `.vscode/settings.json`.
2. You should not be prompted for any binary installations.
3. The extension should work when you open the `src/test.sql` file.
