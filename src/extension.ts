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

  // Register build command
  const buildCommand = vscode.commands.registerCommand(
    'stellar-contracts-lsp.build',
    async () => {
      try {
        const config = vscode.workspace.getConfiguration('stellar');
        const cliPath = config.get('cli.path', 'stellar');
        
        vscode.window.showInformationMessage('Starting Stellar contract build...');
        
        const terminal = vscode.window.createTerminal('Stellar Build');
        terminal.show();
        terminal.sendText(`${cliPath} contract build`);
        
        vscode.window.showInformationMessage('Build command sent to terminal');
      } catch (error) {
        vscode.window.showErrorMessage(`Build error: ${error}`);
      }
    }
  );

  // Register test command
  const testCommand = vscode.commands.registerCommand(
    'stellar-contracts-lsp.test',
    async () => {
      const terminal = vscode.window.createTerminal('Stellar Test');
      terminal.show();
      terminal.sendText('cargo test');
    }
  );

  // Register deploy command
  const deployCommand = vscode.commands.registerCommand(
    'stellar-contracts-lsp.deploy',
    async () => {
      const config = vscode.workspace.getConfiguration('stellar');
      const cliPath = config.get('cli.path', 'stellar');
      const rpcUrl = config.get('network.rpc', 'https://soroban-testnet.stellar.org');
      const passphrase = config.get('network.passphrase', 'Test SDF Network ; September 2015');
      
      const wasmFiles = await vscode.workspace.findFiles('**/target/**/*.wasm', '**/node_modules/**');
      if (wasmFiles.length === 0) {
        vscode.window.showErrorMessage('No WASM files found. Please build the contract first.');
        return;
      }
      
      const selectedFile = wasmFiles[0]; // Use first WASM file found
      const terminal = vscode.window.createTerminal('Stellar Deploy');
      terminal.show();
      terminal.sendText(`${cliPath} contract deploy --wasm "${selectedFile.fsPath}" --rpc-url ${rpcUrl} --network-passphrase "${passphrase}"`);
    }
  );

  // Register invoke command
  const invokeCommand = vscode.commands.registerCommand(
    'stellar-contracts-lsp.invoke',
    async () => {
      const config = vscode.workspace.getConfiguration('stellar');
      const cliPath = config.get('cli.path', 'stellar');
      const rpcUrl = config.get('network.rpc', 'https://soroban-testnet.stellar.org');
      const passphrase = config.get('network.passphrase', 'Test SDF Network ; September 2015');
      
      const contractId = await vscode.window.showInputBox({
        prompt: 'Enter contract ID',
        placeHolder: 'CONTRACT_ID'
      });
      
      if (!contractId) {
        return;
      }
      
      const functionName = await vscode.window.showInputBox({
        prompt: 'Enter function name',
        placeHolder: 'function_name'
      });
      
      if (!functionName) {
        return;
      }
      
      const terminal = vscode.window.createTerminal('Stellar Invoke');
      terminal.show();
      terminal.sendText(`${cliPath} contract invoke --id ${contractId} --fn ${functionName} --rpc-url ${rpcUrl} --network-passphrase "${passphrase}"`);
    }
  );

  // Register generate types command
  const generateTypesCommand = vscode.commands.registerCommand(
    'stellar-contracts-lsp.generateTypes',
    async () => {
      const config = vscode.workspace.getConfiguration('stellar');
      const cliPath = config.get('cli.path', 'stellar');
      
      // Find WASM files in target directory (exclude VS Code internal files)
      const wasmFiles = await vscode.workspace.findFiles('**/target/**/*.wasm', '**/node_modules/**');
      if (wasmFiles.length === 0) {
        vscode.window.showErrorMessage('No WASM files found in target directory. Please build the contract first using "Stellar: Build Stellar Contract".');
        return;
      }
      
      // Let user select WASM file if multiple found
      let selectedFile = wasmFiles[0];
      if (wasmFiles.length > 1) {
        const selectedItem = await vscode.window.showQuickPick(
          wasmFiles.map(file => ({
            label: path.basename(file.fsPath),
            description: file.fsPath,
            file: file
          })),
          { placeHolder: 'Select WASM file to generate TypeScript bindings for' }
        );
        if (!selectedItem) {
          return;
        }
        selectedFile = selectedItem.file;
      }
      
      // Get output directory
      const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
      if (!workspaceFolder) {
        vscode.window.showErrorMessage('No workspace folder found');
        return;
      }
      
      const outputDir = path.join(workspaceFolder.uri.fsPath, 'bindings');
      
      const terminal = vscode.window.createTerminal('Stellar Generate Types');
      terminal.show();
      
      // Use quoted paths to handle spaces properly
      const wasmPath = `"${selectedFile.fsPath}"`;
      const outputPath = `"${outputDir}"`;
      
      terminal.sendText(`${cliPath} contract bindings typescript --wasm ${wasmPath} --output-dir ${outputPath} --overwrite`);
      
      vscode.window.showInformationMessage(`Generating TypeScript bindings from ${path.basename(selectedFile.fsPath)}...`);
    }
  );

  // Register debug command
  const debugCommand = vscode.commands.registerCommand(
    'stellar-contracts-lsp.debug',
    async () => {
      try {
        const config = vscode.workspace.getConfiguration('stellar');
        const debugEnabled = config.get('debug.enable', true);
        
        if (!debugEnabled) {
          vscode.window.showErrorMessage('Debugging is disabled. Enable it in settings.');
          return;
        }

        // Check if contract is built
        const wasmFiles = await vscode.workspace.findFiles('**/target/**/*.wasm', '**/node_modules/**');
        if (wasmFiles.length === 0) {
          const buildFirst = await vscode.window.showWarningMessage(
            'No WASM files found. Build the contract first?',
            'Build Now',
            'Cancel'
          );
          
          if (buildFirst === 'Build Now') {
            vscode.commands.executeCommand('stellar-contracts-lsp.build');
            return;
          }
          return;
        }

        // Start debugging session
        const debugConfig = {
          type: 'lldb',
          request: 'launch',
          name: 'Debug Stellar Contract',
          program: '${workspaceFolder}/target/debug/${workspaceFolderBasename}',
          args: [],
          cwd: '${workspaceFolder}',
          env: {},
          stopOnEntry: false,
          console: 'integratedTerminal'
        };

        await vscode.debug.startDebugging(undefined, debugConfig);
        vscode.window.showInformationMessage('Starting debug session for Stellar contract...');
        
      } catch (error) {
        vscode.window.showErrorMessage(`Debug error: ${error}`);
      }
    }
  );

  context.subscriptions.push(
    restartCommand,
    buildCommand,
    testCommand,
    deployCommand,
    invokeCommand,
    generateTypesCommand,
    debugCommand
  );

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
