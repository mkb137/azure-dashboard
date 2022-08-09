use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use once_cell::sync::OnceCell;
use std::path::{Path, PathBuf};

// The index.html path
static INDEX_HTML: OnceCell<PathBuf> = OnceCell::new();
// The WWW root path
static WWWROOT: OnceCell<PathBuf> = OnceCell::new();

// Returns the index.html path.
fn index_html() -> &'static PathBuf {
    INDEX_HTML.get_or_init(|| PathBuf::from("wwwroot/index.html"))
}

// Returns the wwwroot path, which is used as the base for static file requests.
fn wwwroot() -> &'static PathBuf {
    WWWROOT.get_or_init(|| PathBuf::from("wwwroot"))
}

// Serves a static file at the given path.
pub async fn static_file(req: HttpRequest) -> Result<NamedFile> {
    log::debug!("static_file");
    // Get the path in the request
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    log::debug!(" - path = {:?}", path);
    // Get the path's extension
    match path.extension() {
        // If the path has an extension...
        Some(_) => {
            // Join the two to get the full path
            let full_path = wwwroot().join(path);
            log::debug!(" - full path = {:?}", full_path);
            // Open the file at the full path
            Ok(NamedFile::open(full_path)?)
        }
        // If the path has no extension...
        None => {
            // The path has no extension, so is presumably to be routed to the svelte-router at index.html.
            log::debug!(" - routing to index.html");
            // Open index.html
            Ok(NamedFile::open(index_html())?)
        }
    }
}
