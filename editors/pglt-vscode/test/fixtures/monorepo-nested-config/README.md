# Simple Project

A mono repo that contains top-level dotfiles and several services in the `packages` folder.

## Expectations

The extension should recognize the `.vscode/settings.json` file and follow it to the `pglt.toml` file in the `packages/service-a` directory.

## Test protocol

0. Follow the instructions in `GLOBAL_SETUP.md`.
1. You should be prompted to download a file once you open the extension host.
2. Once downloaded, the extension should work as expected when you open the `packages/service-b/test.sql` file.
