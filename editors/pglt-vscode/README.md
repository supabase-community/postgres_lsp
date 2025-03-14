# PGLT Extension for VS Code

The **PGLT extension for Visual Studio Code** brings PostgreSQL inline suggestions, linting, and type checks to VSCode and VSCode-based editors.

## Installation

The PGLT extension for VS Code is not yet distributed but will soon be available via the Visual Studio Marketplace and the Open VSX Registry.

## Architecture

The VSCode extension looks for the `pglt` binary and uses it to start an LSP background process. It then creates a VSCode LSP Client and connects it to the server.

It'll try five strategies to find the binary, in the following order:

1. The `pglt.bin` VSCode setting can point to a binary with relative or absolute paths.
2. If you have installed pglt via node_modules, the extension will look for the matching binary in your `node_modules`.
3. If you have installed pglt via Yarn Plug'n'Play, the extension will check your `.pnp.cjs` file for a binary.
4. The extension will scan your $PATH for a `pglt` on Darwin/Linux or `pglt.exe` on Windows.
5. If no binary will be found, you will be prompted to download a binary from `pglt`'s Github Releases. You can always download a CLI version via VSCode's Command Palette. (If you want to download prereleases, set `pglt.allowDownloadPrereleaes` in your VSCode settings.)

To connect to your database, `pglt` needs to read a `pglt.toml` config file. By default, the extension will look for that file in your repository. You can specify an alternative path via the `pglt.configFile` VSCode setting.
