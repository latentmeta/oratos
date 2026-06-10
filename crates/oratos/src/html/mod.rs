//! HTML loading, parsing, and page extraction.

mod extract;
mod load;
mod page;
mod path;
mod robots;
mod sitemap;

pub use extract::parse_html;
pub use load::{load_pages, normalize_url, resolve_internal_path, CrawlOptions, LoadOptions};
pub use page::{Heading, HtmlPage, ImageInfo, JsonLdBlock, LinkInfo};
pub use path::{is_site_root_path, normalize_path_separators};
pub use robots::{disallowed_paths, is_disallowed};
pub use sitemap::urls_from_sitemap_xml;
