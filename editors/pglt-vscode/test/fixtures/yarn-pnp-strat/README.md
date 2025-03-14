# Simple Project

A bread-and-butter project with a `pglt.toml` on top level and SQL files in the `test` folder. The extension is installed via Yarn Plug'n'Play.

## Expectations

The extension should recognize the `pglt.toml` file and connect with the right database. We shouldn't have to install a global binary.

## Test protocol

0. Follow the instructions in `GLOBAL_SETUP.md`.
1. Make sure you have `yarn` installed.
2. Run `yarn --version` to make sure you're using the yarn version specified in the `package.json#packageManager`.
3. Run `yarn install`.
4. Make sure you do not have a folder containing a `pglt(.exe)` binary in your $PATH.
5. You should not be prompted for any binary installations.
6. You should be able to open the `src/test.sql` file and the extension should work.
