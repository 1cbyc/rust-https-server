use crate::Error;
use std::path::Path;

pub fn get_mime_type(path: &str) -> &'static str {
    let extension = Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    match extension.to_lowercase().as_str() {
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "txt" => "text/plain",
        "pdf" => "application/pdf",
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "eot" => "application/vnd.ms-fontobject",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "zip" => "application/zip",
        "tar" => "application/x-tar",
        "gz" => "application/gzip",
        _ => "application/octet-stream",
    }
}

pub fn sanitize_path(path: &str) -> Result<String, Error> {
    let path = path.trim_start_matches('/');
    
    if path.contains("..") {
        return Err(Error::InvalidPath("Path traversal not allowed".to_string()));
    }
    
    if path.contains('\\') {
        return Err(Error::InvalidPath("Invalid path separator".to_string()));
    }
    
    Ok(path.to_string())
}

pub fn validate_file_extension(filename: &str, allowed_extensions: &[String]) -> Result<(), Error> {
    if allowed_extensions.is_empty() {
        return Ok(());
    }
    
    let extension = Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    if !allowed_extensions.contains(&extension) {
        return Err(Error::InvalidPath(format!("File extension '{}' not allowed", extension)));
    }
    
    Ok(())
}

pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    match bytes {
        0..KB => format!("{} B", bytes),
        KB..MB => format!("{:.1} KB", bytes as f64 / KB as f64),
        MB..GB => format!("{:.1} MB", bytes as f64 / MB as f64),
        _ => format!("{:.1} GB", bytes as f64 / GB as f64),
    }
}

pub fn generate_directory_listing(path: &str, entries: &[std::fs::DirEntry]) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<title>Directory Listing</title>\n");
    html.push_str("<style>\n");
    html.push_str("body { font-family: Arial, sans-serif; margin: 20px; }\n");
    html.push_str("table { border-collapse: collapse; width: 100%; }\n");
    html.push_str("th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }\n");
    html.push_str("th { background-color: #f2f2f2; }\n");
    html.push_str("a { text-decoration: none; color: #0066cc; }\n");
    html.push_str("a:hover { text-decoration: underline; }\n");
    html.push_str("</style>\n</head>\n<body>\n");
    html.push_str(&format!("<h1>Directory Listing: {}</h1>\n", path));
    html.push_str("<table>\n");
    html.push_str("<tr><th>Name</th><th>Size</th><th>Modified</th></tr>\n");
    
    for entry in entries {
        let name = entry.file_name().to_string_lossy().to_string();
        let metadata = entry.metadata().unwrap_or_else(|_| std::fs::metadata(".").unwrap_or_else(|_| panic!("Cannot get metadata")));
        let size = if metadata.is_file() {
            format_file_size(metadata.len())
        } else {
            "-".to_string()
        };
        let modified = metadata
            .modified()
            .map(|t| {
                let datetime: chrono::DateTime<chrono::Utc> = t.into();
                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            })
            .unwrap_or_else(|_| "-".to_string());
        
        let link = if metadata.is_dir() {
            format!("<a href=\"{}/\">{}/</a>", name, name)
        } else {
            format!("<a href=\"{}\">{}</a>", name, name)
        };
        
        html.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            link, size, modified
        ));
    }
    
    html.push_str("</table>\n</body>\n</html>");
    html
}

pub fn parse_query_string(query: &str) -> std::collections::HashMap<String, String> {
    let mut params = std::collections::HashMap::new();
    
    for pair in query.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            let key = urlencoding::decode(key).unwrap_or_else(|_| key.to_string().into()).to_string();
            let value = urlencoding::decode(value).unwrap_or_else(|_| value.to_string().into()).to_string();
            params.insert(key, value);
        }
    }
    
    params
}

pub fn is_safe_path(path: &str) -> bool {
    !path.contains("..") && !path.contains('\\') && !path.starts_with('/')
}

pub fn normalize_path(path: &str) -> String {
    path.trim_start_matches('/').to_string()
} 