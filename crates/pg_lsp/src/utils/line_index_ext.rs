use line_index::{LineCol, LineColUtf16, LineIndex};
use lsp_types::{Position, Range};
use text_size::{TextRange, TextSize};

pub trait LineIndexExt {
    fn offset_lsp(&self, line_col: Position) -> Option<TextSize>;

    fn offset_lsp_range(&self, line_col: Range) -> Option<TextRange>;

    fn line_col_lsp(&self, offset: TextSize) -> Option<Position>;

    fn line_col_lsp_range(&self, offset: TextRange) -> Option<Range>;
}

impl LineIndexExt for LineIndex {
    fn offset_lsp(&self, line_col: Position) -> Option<TextSize> {
        let line_col = LineColUtf16 {
            line: line_col.line,
            col: line_col.character,
        };

        let line_col = self.to_utf8(line_col)?;
        self.offset(line_col)
    }

    fn offset_lsp_range(&self, line_col: Range) -> Option<TextRange> {
        let start = self.offset_lsp(line_col.start)?;
        let end = self.offset_lsp(line_col.end)?;
        Some(TextRange::new(start, end))
    }

    fn line_col_lsp(&self, offset: TextSize) -> Option<Position> {
        let line_col = self.line_col(offset);
        let line_col = self.to_utf16(line_col)?;
        Some(Position::new(line_col.line, line_col.col))
    }

    fn line_col_lsp_range(&self, offset: TextRange) -> Option<Range> {
        let start = self.line_col_lsp(offset.start())?;
        let mut end = self.line_col_lsp(offset.end())?;
        if end.line != start.line && end.character == 0 {
            // Prefer keeping multi-line ranges on the same line
            let line_end = self.offset(LineCol {
                line: end.line,
                col: 0,
            })?;

            end = self.line_col_lsp(line_end - TextSize::from(1))?;
        }

        Some(Range::new(start, end))
    }
}

#[cfg(test)]
mod tests {
    use base_db::{Document, DocumentParams, PgLspPath};
    use cstree::text::{TextRange, TextSize};

    use super::LineIndexExt;

    #[test]
    fn test_line_col_lsp_range() {
        let url = PgLspPath::new("test.sql");

        let d = Document::new(DocumentParams {
            url,
            text: "select 1 from contact;\nselect 1;\nalter table test drop column id;".to_string(),
        });

        println!(
            "{:#?}",
            d.line_index
                .line_col_lsp_range(TextRange::new(TextSize::new(52), TextSize::new(66)))
        );
    }
}
