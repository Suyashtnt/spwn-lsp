use std::path::PathBuf;

use spwn::errors::SyntaxError;
use spwn::shared::SpwnSource;
use tower_lsp::lsp_types::*;

use crate::utils::pos_to_range;

pub fn syntax_check(text: String, path: PathBuf) -> Option<Diagnostic> {
    let syntax_checks = spwn::parse_spwn(
        text.clone(),
        SpwnSource::File(path),
        spwn::builtins::BUILTIN_NAMES,
    );

    if let Err(error) = syntax_checks {
        return Some(match error {
            SyntaxError::ExpectedErr {
                expected,
                found,
                pos,
                file: _,
            } => Diagnostic {
                code: Some(NumberOrString::String("ExpectedErr".to_string())),
                message: format!("expected {}, found {}", expected, found),
                range: pos_to_range(&text, pos),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("SPWN Syntax".to_string()),
                ..Default::default()
            },
            SyntaxError::UnexpectedErr {
                found,
                pos,
                file: _,
            } => Diagnostic {
                code: Some(NumberOrString::String("UnexpectedErr".to_string())),
                message: format!("unexpected {}", found),
                range: pos_to_range(&text, pos),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("SPWN Syntax".to_string()),
                ..Default::default()
            },
            SyntaxError::SyntaxError {
                message,
                pos,
                file: _,
            } => Diagnostic {
                code: Some(NumberOrString::String("SyntaxError".to_string())),
                message,
                range: pos_to_range(&text, pos),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("SPWN Syntax".to_string()),
                ..Default::default()
            },
            SyntaxError::CustomError(err) => Diagnostic {
                code: Some(NumberOrString::String("CustomError".to_string())),
                message: err.message,
                range: pos_to_range(&text, err.info.position.pos),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("SPWN Syntax".to_string()),
                ..Default::default()
            },
        });
    }

    None
}
