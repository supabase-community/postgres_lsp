# Setting Up a Test

1. Run the `docker-compose.yml` file.
2. Optional: If you want to work on the extension while debugging, run `npm run watch`. You'll see changes if you use the "Restart Extension Host" command in the debugging window.
3. Run `just init` (install `just` if you haven't, or use the shell command in the `justfile`).
4. Open the VSCode Debug window and choose the appropriate launch configuration. Press F5.

## Getting a Fresh State

In the extension host, follow the test's TestSetup in the relevant README.
Before the test assertions, use the "PGLT: Hard Reset" VSCode command. This will clear all previously installed binaries.
