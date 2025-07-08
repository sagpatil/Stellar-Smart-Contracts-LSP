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
  const lines = text.split('\n');
  
  let hasContractStruct = false;
  let hasContractImpl = false;
  let hasContractAttribute = false;
  let hasContractImplAttribute = false;
  let hasSorobanSdkImport = false;
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const trimmedLine = line.trim();
    
    // Check for soroban_sdk import
    if (trimmedLine.includes('use soroban_sdk')) {
      hasSorobanSdkImport = true;
    }
    
    // Check for contract struct
    if (trimmedLine.includes('struct') && trimmedLine.includes('Contract')) {
      hasContractStruct = true;
    }
    
    // Check for contract implementation
    if (trimmedLine.includes('impl') && trimmedLine.includes('Contract')) {
      hasContractImpl = true;
    }
    
    // Check for contract attributes
    if (trimmedLine.includes('#[contract]')) {
      hasContractAttribute = true;
    }
    
    if (trimmedLine.includes('#[contractimpl]')) {
      hasContractImplAttribute = true;
    }
    
    // Check for missing #[contract] attribute before struct
    if (trimmedLine.includes('struct') && trimmedLine.includes('Contract') && 
        (i === 0 || !lines[i-1].trim().includes('#[contract]'))) {
      diagnostics.push({
        severity: DiagnosticSeverity.Error,
        range: {
          start: { line: i, character: 0 },
          end: { line: i, character: line.length }
        },
        message: 'Stellar contract struct must have #[contract] attribute',
        source: 'stellar-lsp'
      });
    }
    
    // Check for missing #[contractimpl] attribute before impl
    if (trimmedLine.includes('impl') && trimmedLine.includes('Contract') && 
        (i === 0 || !lines[i-1].trim().includes('#[contractimpl]'))) {
      diagnostics.push({
        severity: DiagnosticSeverity.Error,
        range: {
          start: { line: i, character: 0 },
          end: { line: i, character: line.length }
        },
        message: 'Stellar contract implementation must have #[contractimpl] attribute',
        source: 'stellar-lsp'
      });
    }
    
    // Check for missing #[contracttype] attribute before enum/struct
    if ((trimmedLine.includes('enum') || trimmedLine.includes('struct')) && 
        !trimmedLine.includes('Contract') &&
        (i === 0 || !lines[i-1].trim().includes('#[contracttype]')) &&
        hasSorobanSdkImport) {
      diagnostics.push({
        severity: DiagnosticSeverity.Warning,
        range: {
          start: { line: i, character: 0 },
          end: { line: i, character: line.length }
        },
        message: 'Consider adding #[contracttype] attribute for Stellar contract data types',
        source: 'stellar-lsp'
      });
    }
    
    // Check for missing require_auth() calls
    if (trimmedLine.includes('pub fn') && !trimmedLine.includes('view')) {
      let hasRequireAuth = false;
      // Look ahead for require_auth in the function
      for (let j = i + 1; j < Math.min(i + 10, lines.length); j++) {
        if (lines[j].includes('require_auth()')) {
          hasRequireAuth = true;
          break;
        }
        if (lines[j].includes('}')) {
          break;
        }
      }
      
      if (!hasRequireAuth && trimmedLine.includes('Address')) {
        diagnostics.push({
          severity: DiagnosticSeverity.Warning,
          range: {
            start: { line: i, character: 0 },
            end: { line: i, character: line.length }
          },
          message: 'Consider adding require_auth() for address parameters in state-changing functions',
          source: 'stellar-lsp'
        });
      }
    }
    
    // Check for deprecated or incorrect patterns
    if (trimmedLine.includes('unwrap()') && !trimmedLine.includes('unwrap_or')) {
      diagnostics.push({
        severity: DiagnosticSeverity.Warning,
        range: {
          start: { line: i, character: trimmedLine.indexOf('unwrap()') },
          end: { line: i, character: trimmedLine.indexOf('unwrap()') + 8 }
        },
        message: 'Consider using unwrap_or() or proper error handling instead of unwrap()',
        source: 'stellar-lsp'
      });
    }
    
    // Check for proper storage usage
    if (trimmedLine.includes('env.storage()') && !trimmedLine.includes('.instance()') && 
        !trimmedLine.includes('.persistent()') && !trimmedLine.includes('.temporary()')) {
      diagnostics.push({
        severity: DiagnosticSeverity.Error,
        range: {
          start: { line: i, character: 0 },
          end: { line: i, character: line.length }
        },
        message: 'Storage access must specify type: .instance(), .persistent(), or .temporary()',
        source: 'stellar-lsp'
      });
    }
  }
  
  // Global validations
  if (hasSorobanSdkImport && hasContractStruct && !hasContractAttribute) {
    diagnostics.push({
      severity: DiagnosticSeverity.Error,
      range: {
        start: { line: 0, character: 0 },
        end: { line: 0, character: 0 }
      },
      message: 'Contract struct is missing #[contract] attribute',
      source: 'stellar-lsp'
    });
  }
  
  if (hasSorobanSdkImport && hasContractImpl && !hasContractImplAttribute) {
    diagnostics.push({
      severity: DiagnosticSeverity.Error,
      range: {
        start: { line: 0, character: 0 },
        end: { line: 0, character: 0 }
      },
      message: 'Contract implementation is missing #[contractimpl] attribute',
      source: 'stellar-lsp'
    });
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
      {
        label: '#[contractmeta]',
        kind: CompletionItemKind.Snippet,
        data: 4,
        insertText: '#[contractmeta(\n    key = "${1:key}",\n    val = "${2:value}"\n)]',
        insertTextFormat: 2, // Snippet format
        documentation: 'Stellar contract metadata attribute'
      },
      
      // Stellar types
      {
        label: 'Env',
        kind: CompletionItemKind.Class,
        data: 5,
        documentation: 'Stellar environment'
      },
      {
        label: 'Address',
        kind: CompletionItemKind.Class,
        data: 6,
        documentation: 'Stellar address type'
      },
      {
        label: 'Symbol',
        kind: CompletionItemKind.Class,
        data: 7,
        documentation: 'Stellar symbol type'
      },
      {
        label: 'Bytes',
        kind: CompletionItemKind.Class,
        data: 8,
        documentation: 'Stellar bytes type'
      },
      {
        label: 'Map',
        kind: CompletionItemKind.Class,
        data: 9,
        documentation: 'Stellar map type'
      },
      {
        label: 'Vec',
        kind: CompletionItemKind.Class,
        data: 10,
        documentation: 'Stellar vector type'
      },
      
      // Stellar SDK imports
      {
        label: 'use soroban_sdk::{contract, contractimpl, Env};',
        kind: CompletionItemKind.Snippet,
        data: 11,
        insertText: 'use soroban_sdk::{contract, contractimpl, Env};',
        documentation: 'Common Stellar SDK imports'
      },
      {
        label: 'use soroban_sdk::{contracttype, Address, Symbol};',
        kind: CompletionItemKind.Snippet,
        data: 12,
        insertText: 'use soroban_sdk::{contracttype, Address, Symbol};',
        documentation: 'Additional Stellar SDK imports'
      },
      
      // Stellar storage methods
      {
        label: 'env.storage().instance()',
        kind: CompletionItemKind.Method,
        data: 13,
        insertText: 'env.storage().instance()',
        documentation: 'Access instance storage'
      },
      {
        label: 'env.storage().persistent()',
        kind: CompletionItemKind.Method,
        data: 14,
        insertText: 'env.storage().persistent()',
        documentation: 'Access persistent storage'
      },
      {
        label: 'env.storage().temporary()',
        kind: CompletionItemKind.Method,
        data: 15,
        insertText: 'env.storage().temporary()',
        documentation: 'Access temporary storage'
      },
      
      // Stellar environment methods
      {
        label: 'env.ledger().timestamp()',
        kind: CompletionItemKind.Method,
        data: 16,
        insertText: 'env.ledger().timestamp()',
        documentation: 'Get current ledger timestamp'
      },
      {
        label: 'env.ledger().sequence()',
        kind: CompletionItemKind.Method,
        data: 17,
        insertText: 'env.ledger().sequence()',
        documentation: 'Get current ledger sequence'
      },
      {
        label: 'require_auth()',
        kind: CompletionItemKind.Method,
        data: 18,
        insertText: 'require_auth()',
        documentation: 'Require authentication for address'
      },
      
      // Stellar macros
      {
        label: 'symbol_short!',
        kind: CompletionItemKind.Function,
        data: 19,
        insertText: 'symbol_short!("${1:symbol}")',
        insertTextFormat: 2,
        documentation: 'Create a short symbol (up to 14 characters)'
      },
      {
        label: 'panic!',
        kind: CompletionItemKind.Function,
        data: 20,
        insertText: 'panic!("${1:message}")',
        insertTextFormat: 2,
        documentation: 'Panic with a message'
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
      item.detail = 'Stellar Contract Metadata Attribute';
      item.documentation = 'Adds metadata to a Stellar contract';
    } else if (item.data === 5) {
      item.detail = 'Stellar Environment';
      item.documentation = 'The Stellar environment provides access to host functions';
    } else if (item.data === 6) {
      item.detail = 'Stellar Address';
      item.documentation = 'Represents a Stellar address';
    } else if (item.data === 7) {
      item.detail = 'Stellar Symbol';
      item.documentation = 'Represents a symbol in Stellar';
    } else if (item.data === 8) {
      item.detail = 'Stellar Bytes';
      item.documentation = 'Arbitrary byte data in Stellar';
    } else if (item.data === 9) {
      item.detail = 'Stellar Map';
      item.documentation = 'A key-value mapping in Stellar';
    } else if (item.data === 10) {
      item.detail = 'Stellar Vec';
      item.documentation = 'A vector (array) type in Stellar';
    } else if (item.data === 11) {
      item.detail = 'Basic Stellar SDK Imports';
      item.documentation = 'Common imports for Stellar contract development';
    } else if (item.data === 12) {
      item.detail = 'Additional Stellar SDK Imports';
      item.documentation = 'Additional imports for Stellar contract development';
    } else if (item.data === 13) {
      item.detail = 'Instance Storage';
      item.documentation = 'Access to instance storage (lifetime of contract instance)';
    } else if (item.data === 14) {
      item.detail = 'Persistent Storage';
      item.documentation = 'Access to persistent storage (survives contract updates)';
    } else if (item.data === 15) {
      item.detail = 'Temporary Storage';
      item.documentation = 'Access to temporary storage (lifetime of transaction)';
    } else if (item.data === 16) {
      item.detail = 'Ledger Timestamp';
      item.documentation = 'Get the current ledger timestamp';
    } else if (item.data === 17) {
      item.detail = 'Ledger Sequence';
      item.documentation = 'Get the current ledger sequence number';
    } else if (item.data === 18) {
      item.detail = 'Require Authentication';
      item.documentation = 'Require authentication for the address';
    } else if (item.data === 19) {
      item.detail = 'Short Symbol Macro';
      item.documentation = 'Create a short symbol (up to 14 characters)';
    } else if (item.data === 20) {
      item.detail = 'Panic Macro';
      item.documentation = 'Panic with a message';
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
    'Env': `## Env
The Stellar environment provides access to host functions and the execution context.

### Common Methods:
- \`env.storage()\` - Access contract storage
- \`env.ledger()\` - Access ledger information
- \`env.current_contract_address()\` - Get current contract address
- \`env.invoker()\` - Get the invoker of the current contract call`,

    'Address': `## Address
Represents a Stellar address. Can be a user account or a contract address.

### Common Methods:
- \`address.require_auth()\` - Require authentication for this address
- \`address.clone()\` - Clone the address

### Usage:
Used in contract functions to identify accounts and contracts.`,

    'Symbol': `## Symbol
A symbol type in Stellar, used for efficient string-like identifiers.

### Creation:
- \`symbol_short!("text")\` - Create short symbol (≤14 chars)
- \`Symbol::new(&env, "text")\` - Create symbol from string

### Usage:
Commonly used for token names, function names, and identifiers.`,

    'Bytes': `## Bytes
Arbitrary byte data in Stellar contracts.

### Common Methods:
- \`bytes.len()\` - Get length
- \`bytes.get(index)\` - Get byte at index
- \`bytes.slice(start, end)\` - Get slice

### Usage:
Used for storing arbitrary data, file contents, or serialized data.`,

    'Map': `## Map
A key-value mapping in Stellar contracts.

### Common Methods:
- \`map.get(key)\` - Get value by key
- \`map.set(key, value)\` - Set key-value pair
- \`map.has(key)\` - Check if key exists
- \`map.remove(key)\` - Remove key

### Usage:
Used for storing associative data in contracts.`,

    'Vec': `## Vec
A vector (array) type in Stellar contracts.

### Common Methods:
- \`vec.len()\` - Get length
- \`vec.get(index)\` - Get element at index
- \`vec.push_back(value)\` - Add element to end
- \`vec.pop_back()\` - Remove last element

### Usage:
Used for storing ordered collections of data.`,

    'contract': `## #[contract]
Attribute to mark a struct as a Stellar contract.

### Usage:
\`\`\`rust
#[contract]
pub struct MyContract;
\`\`\`

### Requirements:
- Must be applied to a struct
- Struct will become the contract implementation base`,

    'contractimpl': `## #[contractimpl]
Attribute to mark an impl block as a Stellar contract implementation.

### Usage:
\`\`\`rust
#[contractimpl]
impl MyContract {
    pub fn my_function(env: Env) -> u32 {
        // Implementation
    }
}
\`\`\`

### Requirements:
- Must be applied to impl block for contract struct
- Contains the contract's public functions`,

    'contracttype': `## #[contracttype]
Attribute to mark a type as a Stellar contract type.

### Usage:
\`\`\`rust
#[contracttype]
pub enum DataKey {
    Balance(Address),
    Config,
}

#[contracttype]
pub struct TokenInfo {
    pub name: Symbol,
    pub symbol: Symbol,
}
\`\`\`

### Requirements:
- Applied to structs and enums used in contract storage
- Makes types serializable for contract storage`,

    'contractmeta': `## #[contractmeta]
Attribute to add metadata to a Stellar contract.

### Usage:
\`\`\`rust
#[contractmeta(
    key = "Description",
    val = "Token contract for payments"
)]
\`\`\`

### Purpose:
- Adds descriptive metadata to contracts
- Useful for documentation and contract identification`,

    'storage': `## Contract Storage
Stellar contracts have three types of storage:

### Instance Storage
- \`env.storage().instance()\`
- Lifetime: Contract instance
- Use for: Contract configuration, admin settings

### Persistent Storage
- \`env.storage().persistent()\`
- Lifetime: Survives contract updates
- Use for: User data, balances, permanent state

### Temporary Storage
- \`env.storage().temporary()\`
- Lifetime: Current transaction
- Use for: Temporary calculations, caching`,

    'require_auth': `## require_auth()
Require authentication for an address.

### Usage:
\`\`\`rust
pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
    from.require_auth();
    // ... transfer logic
}
\`\`\`

### Purpose:
- Ensures the address holder has authorized the transaction
- Must be called for state-changing operations involving the address`,

    'ledger': `## Ledger Information
Access current ledger information through \`env.ledger()\`.

### Common Methods:
- \`env.ledger().timestamp()\` - Current ledger close time
- \`env.ledger().sequence()\` - Current ledger sequence number
- \`env.ledger().network_id()\` - Network identifier

### Usage:
Used for time-based logic, sequence tracking, and network identification.`,

    'panic': `## panic!
Panic with a message, terminating contract execution.

### Usage:
\`\`\`rust
if amount <= 0 {
    panic!("Amount must be positive");
}
\`\`\`

### Behavior:
- Terminates contract execution
- Reverts all state changes
- Returns error to caller`
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
