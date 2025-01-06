import { type ExtensionContext, window } from 'vscode';

import {
    type Executable,
    LanguageClient,
    type LanguageClientOptions,
    type ServerOptions
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(_context: ExtensionContext) {
    // If the extension is launched in debug mode then the debug server options are used
    // Otherwise the run options are used
    const run: Executable = {
        command: 'pg_cli',
        args: ['lsp-proxy']
    };

    const outputChannel = window.createOutputChannel('Postgres LSP', { log: true });

    const serverOptions: ServerOptions = {
        run,
        debug: run
    };

    // Options to control the language client
    const clientOptions: LanguageClientOptions = {
        // Register the server for plain text documents
        documentSelector: [{ scheme: 'file', language: 'sql' }],
        outputChannel
    };

    // Create the language client and start the client.
    client = new LanguageClient('postgres_lsp', 'Postgres LSP', serverOptions, clientOptions);

    // Start the client. This will also launch the server
    void client.start();
}

export function deactivate(): Thenable<void> | undefined {
    console.log('Deactivating client...');
    if (!client) {
        return undefined;
    }
    return client.stop();
}
