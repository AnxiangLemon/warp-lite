use anyhow::Result;

use crate::FormattedText;
pub use crate::WARP_EMBED_ATTRIBUTE_NAME;

pub fn parse_html(html: &str) -> Result<FormattedText> {
    let _ = html;
    anyhow::bail!("HTML parsing is disabled in this build")
}
