use base_db::PgLspPath;
use text_size::TextSize;

pub struct HoverParams {
    pub position: TextSize,
    pub url: PgLspPath,
}
