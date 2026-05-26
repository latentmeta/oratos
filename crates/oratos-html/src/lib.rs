//! HTML loading, parsing, and page extraction.

mod extract;
mod load;
mod page;

pub use extract::parse_html;
pub use load::{load_file, load_pages, normalize_url, resolve_internal_path, LoadOptions};
pub use page::{Heading, HtmlPage, ImageInfo, JsonLdBlock, LinkInfo};
