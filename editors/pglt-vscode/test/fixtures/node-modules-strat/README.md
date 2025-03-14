# Simple Project

A bread-and-butter project with a `pglt.toml` on top level and SQL files in the `test` folder. The extension is installed via node_modules.

## Expectations

The extension should recognize the `pglt.toml` file and connect with the right database. We shouldn't have to install a global binary.

## Test protocol

0. Follow the instructions in `GLOBAL_SETUP.md`.
1. Run `npm install`.
2. You shouldn't be prompted to download a file once you open the extension host.
3. The extension should work as expected when you open the `src/test.sql` file.
