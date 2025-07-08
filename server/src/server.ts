import {
  createConnection,
  TextDocuments,
  Diagnostic,
  DiagnosticSeverity,
  ProposedFeatures,
  InitializeParams,
  DidChangeConfigurationNotification,
  CompletionItem,
  CompletionItemKind,
  TextDocumentPositionParams,
  TextDocumentSyncKind,
  InitializeResult,
  HoverParams,
  Hover,
  MarkupKind,
  DefinitionParams,
  Location,
  Range,
  Position
} from 'vscode-languageserver/node';

import {
  TextDocument
} from 'vscode-languageserver-textdocument';

// Create a connection for the server
let connection = createConnection(ProposedFeatures.all);

// Create a simple text document manager
let documents: TextDocuments<TextDocument> = new TextDocuments(TextDocument);

let hasConfigurationCapability = false;
let hasWorkspaceFolderCapability = false;
let hasDiagnosticRelatedInformationCapability = false;

connection.onInitialize((params: InitializeParams) => {
  const capabilities = params.capabilities;

  hasConfigurationCapability = !!(
    capabilities.workspace && !!capabilities.workspace.configuration
  );
  hasWorkspaceFolderCapability = !!(
    capabilities.workspace && !!capabilities.workspace.workspaceFolders
  );
  hasDiagnosticRelatedInformationCapability = !!(
    capabilities.textDocument &&
    capabilities.textDocument.publishDiagnostics &&
    capabilities.textDocument.publishDiagnostics.relatedInformation
  );

  const result: InitializeResult = {
    capabilities: {
      textDocumentSync: TextDocumentSyncKind.Incremental,
      completionProvider: {
        resolveProvider: true,
        triggerCharacters: ['.', ':', '::', '#']
      },
      hoverProvider: true,
      definitionProvider: true,
      documentSymbolProvider: true,
      workspaceSymbolProvider: true
    }
  };

  if (hasWorkspaceFolderCapability) {
    result.capabilities.workspace = {
      workspaceFolders: {
        supported: true
      }
    };
  }

  return result;
});

connection.onInitialized(() => {
  if (hasConfigurationCapability) {
    connection.client.register(DidChangeConfigurationNotification.type, undefined);
  }
  if (hasWorkspaceFolderCapability) {
    connection.workspace.onDidChangeWorkspaceFolders(_event => {
      connection.console.log('Workspace folder change event received.');
    });
  }
});

// Configuration interface
interface StellarSettings {
  lsp: {
    enable: boolean;
    trace: {
      server: string;
    };
  };
  diagnostics: {
    enable: boolean;
  };
}

const defaultSettings: StellarSettings = {
  lsp: {
    enable: true,
    trace: {
      server: 'off'
    }
  },
  diagnostics: {
    enable: true
  }
};

let globalSettings: StellarSettings = defaultSettings;
let documentSettings: Map<string, Thenable<StellarSettings>> = new Map();

connection.onDidChangeConfiguration((change: any) => {
  if (hasConfigurationCapability) {
    documentSettings.clear();
  } else {
    globalSettings = <StellarSettings>(
      (change.settings.stellar || defaultSettings)
    );
  }
  documents.all().forEach(validateTextDocument);
});

function getDocumentSettings(resource: string): Thenable<StellarSettings> {
  if (!hasConfigurationCapability) {
    return Promise.resolve(globalSettings);
  }
  let result = documentSettings.get(resource);
  if (!result) {
    result = connection.workspace.getConfiguration({
      scopeUri: resource,
      section: 'stellar'
    });
    if (result) {
      documentSettings.set(resource, result);
    }
  }
  return result || Promise.resolve(globalSettings);
}

documents.onDidClose(e => {
  documentSettings.delete(e.document.uri);
});

documents.onDidChangeContent(change => {
  validateTextDocument(change.document);
});

// Stellar-specific validation
async function validateTextDocument(textDocument: TextDocument): Promise<void> {
  const settings = await getDocumentSettings(textDocument.uri);
  
  if (!settings.diagnostics.enable) {
    return;
  }

  const text = textDocument.getText();
  const diagnostics: Diagnostic[] = [];

  // Basic Stellar contract validation
  const lines = text.split('\n');
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    
    // Check for missing #[contract] attribute
    if (line.includes('impl') && line.includes('Contract') && !text.includes('#[contract]')) {
      diagnostics.push({
        severity: DiagnosticSeverity.Warning,
        range: {
          start: { line: i, character: 0 },
          end: { line: i, character: line.length }
        },
        message: 'Stellar contract implementation should have #[contract] attribute',
        source: 'stellar-lsp'
      });
    }

    // Check for missing #[contractimpl] attribute
    if (line.includes('impl') && !line.includes('Contract') && text.includes('#[contract]')) {
      diagnostics.push({
        severity: DiagnosticSeverity.Warning,
        range: {
          start: { line: i, character: 0 },
          end: { line: i, character: line.length }
        },
        message: 'Stellar contract implementation should have #[contractimpl] attribute',
        source: 'stellar-lsp'
      });
    }
  }

  connection.sendDiagnostics({ uri: textDocument.uri, diagnostics });
}

// Stellar-specific completions
connection.onCompletion(
  (textDocumentPosition: TextDocumentPositionParams): CompletionItem[] => {
    const completions: CompletionItem[] = [
      // Stellar attributes
      {
        label: '#[contract]',
        kind: CompletionItemKind.Snippet,
        data: 1,
        insertText: '#[contract]',
        documentation: 'Stellar contract attribute'
      },
      {
        label: '#[contractimpl]',
        kind: CompletionItemKind.Snippet,
        data: 2,
        insertText: '#[contractimpl]',
        documentation: 'Stellar contract implementation attribute'
      },
      {
        label: '#[contracttype]',
        kind: CompletionItemKind.Snippet,
        data: 3,
        insertText: '#[contracttype]',
        documentation: 'Stellar contract type attribute'
      },
      
      // Stellar types
      {
        label: 'Env',
        kind: CompletionItemKind.Class,
        data: 4,
        documentation: 'Stellar environment'
      },
      {
        label: 'Address',
        kind: CompletionItemKind.Class,
        data: 5,
        documentation: 'Stellar address type'
      },
      {
        label: 'Symbol',
        kind: CompletionItemKind.Class,
        data: 6,
        documentation: 'Stellar symbol type'
      },
      {
        label: 'Bytes',
        kind: CompletionItemKind.Class,
        data: 7,
        documentation: 'Stellar bytes type'
      },
      {
        label: 'Map',
        kind: CompletionItemKind.Class,
        data: 8,
        documentation: 'Stellar map type'
      },
      {
        label: 'Vec',
        kind: CompletionItemKind.Class,
        data: 9,
        documentation: 'Stellar vector type'
      }
    ];

    return completions;
  }
);

connection.onCompletionResolve(
  (item: CompletionItem): CompletionItem => {
    if (item.data === 1) {
      item.detail = 'Stellar Contract Attribute';
      item.documentation = 'Marks a struct as a Stellar contract';
    } else if (item.data === 2) {
      item.detail = 'Stellar Contract Implementation Attribute';
      item.documentation = 'Marks an impl block as a Stellar contract implementation';
    } else if (item.data === 3) {
      item.detail = 'Stellar Contract Type Attribute';
      item.documentation = 'Marks a type as a Stellar contract type';
    } else if (item.data === 4) {
      item.detail = 'Stellar Environment';
      item.documentation = 'The Stellar environment provides access to host functions';
    } else if (item.data === 5) {
      item.detail = 'Stellar Address';
      item.documentation = 'Represents a Stellar address';
    } else if (item.data === 6) {
      item.detail = 'Stellar Symbol';
      item.documentation = 'Represents a symbol in Stellar';
    }
    return item;
  }
);

// Hover provider
connection.onHover((params: HoverParams): Hover | null => {
  const document = documents.get(params.textDocument.uri);
  if (!document) {
    return null;
  }

  const position = params.position;
  const text = document.getText();
  const offset = document.offsetAt(position);
  
  // Simple word extraction around cursor
  const wordRange = getWordRangeAtPosition(text, offset);
  if (!wordRange) {
    return null;
  }

  const word = text.substring(wordRange.start, wordRange.end);
  
  // Stellar-specific hover information
  const stellarInfo = getStellarInfo(word);
  if (stellarInfo) {
    return {
      contents: {
        kind: MarkupKind.Markdown,
        value: stellarInfo
      }
    };
  }

  return null;
});

function getWordRangeAtPosition(text: string, offset: number): { start: number; end: number } | null {
  const wordRegex = /\b\w+\b/g;
  let match;
  
  while ((match = wordRegex.exec(text)) !== null) {
    if (match.index <= offset && offset <= match.index + match[0].length) {
      return {
        start: match.index,
        end: match.index + match[0].length
      };
    }
  }
  
  return null;
}

function getStellarInfo(word: string): string | null {
  const stellarDocs: { [key: string]: string } = {
    'Env': '## Env\n\nThe Stellar environment provides access to host functions and the execution context.',
    'Address': '## Address\n\nRepresents a Stellar address. Can be a user account or a contract.',
    'Symbol': '## Symbol\n\nA symbol type in Stellar, used for efficient string-like identifiers.',
    'Bytes': '## Bytes\n\nArbitrary byte data in Stellar.',
    'Map': '## Map\n\nA key-value mapping in Stellar.',
    'Vec': '## Vec\n\nA vector (array) type in Stellar.',
    'contract': '## #[contract]\n\nAttribute to mark a struct as a Stellar contract.',
    'contractimpl': '## #[contractimpl]\n\nAttribute to mark an impl block as a Stellar contract implementation.',
    'contracttype': '## #[contracttype]\n\nAttribute to mark a type as a Stellar contract type.'
  };

  return stellarDocs[word] || null;
}

connection.onDidChangeWatchedFiles(_change => {
  connection.console.log('Watched files have changed');
});

// Make the text document manager listen on the connection
documents.listen(connection);

// Listen on the connection
connection.listen();
