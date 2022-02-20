mod syntax;
mod utils;

use syntax::syntax_check;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                definition_provider: Some(OneOf::Right(DefinitionOptions {
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: Some(false),
                    },
                })),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        let file = params.content_changes.remove(0).text;

        if let Some(diag) = syntax_check(
            file,
            params
                .text_document
                .uri
                .to_file_path()
                .expect("Could not turn file url to PathBuf"),
        ) {
            self.client
                .publish_diagnostics(params.text_document.uri, vec![diag], None)
                .await;
        } else {
            self.client
                .publish_diagnostics(params.text_document.uri, vec![], None)
                .await;
        }
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let file = params.text_document.text;

        if let Some(diag) = syntax_check(
            file,
            params
                .text_document
                .uri
                .to_file_path()
                .expect("Could not turn file url to PathBuf"),
        ) {
            self.client
                .publish_diagnostics(params.text_document.uri, vec![diag], None)
                .await;
        } else {
            self.client
                .publish_diagnostics(params.text_document.uri, vec![], None)
                .await;
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, messages) = LspService::new(|client| Backend { client });

    Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service)
        .await;
}
