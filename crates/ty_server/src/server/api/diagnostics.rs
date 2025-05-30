use lsp_server::ErrorCode;
use lsp_types::{PublishDiagnosticsParams, Url, notification::PublishDiagnostics};

use crate::server::Result;
use crate::server::client::Notifier;

use super::LSPResult;

pub(super) fn clear_diagnostics(uri: &Url, notifier: &Notifier) -> Result<()> {
    notifier
        .notify::<PublishDiagnostics>(PublishDiagnosticsParams {
            uri: uri.clone(),
            diagnostics: vec![],
            version: None,
        })
        .with_failure_code(ErrorCode::InternalError)?;
    Ok(())
}
