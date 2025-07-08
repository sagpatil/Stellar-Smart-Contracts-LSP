import * as path from 'path';
import * as vscode from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
  console.log('Activating Stellar Smart Contracts LSP extension...');

  // Create the language server
  const serverModule = context.asAbsolutePath(
    path.join('server', 'out', 'server.js')
  );
  
  // Debug options for the server
  const debugOptions = { execArgv: ['--nolazy', '--inspect=6009'] };

  // Server options
  const serverOptions: ServerOptions = {
    run: { module: serverModule, transport: TransportKind.ipc },
    debug: {
      module: serverModule,
      transport: TransportKind.ipc,
      options: debugOptions
    }
  };

  // Client options
  const clientOptions: LanguageClientOptions = {
    // Register the server for Rust files in Stellar projects
    documentSelector: [
      { scheme: 'file', language: 'rust' },
      { scheme: 'file', language: 'stellar' }
    ],
    synchronize: {
      // Notify the server about file changes to relevant files
      fileEvents: vscode.workspace.createFileSystemWatcher('**/*.{rs,toml}')
    },
    outputChannelName: 'Stellar LSP',
    traceOutputChannel: vscode.window.createOutputChannel('Stellar LSP Trace')
  };

  // Create the language client
  client = new LanguageClient(
    'stellar-lsp',
    'Stellar Smart Contracts LSP',
    serverOptions,
    clientOptions
  );

  // Register restart command
  const restartCommand = vscode.commands.registerCommand(
    'stellar-contracts-lsp.restart',
    async () => {
      await client.stop();
      await client.start();
      vscode.window.showInformationMessage('Stellar LSP server restarted');
    }
  );

  context.subscriptions.push(restartCommand);

  // Start the client (this will also launch the server)
  client.start().then(() => {
    console.log('Stellar LSP client started successfully');
  }).catch((error: any) => {
    console.error('Failed to start Stellar LSP client:', error);
    vscode.window.showErrorMessage('Failed to start Stellar LSP server');
  });
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
