use std::path::Path;

pub enum MimeType {
    TextHtml,
    TextCss,
    TextJavascript,
    ImageJpeg,
    TextPlain,
}

impl MimeType {
    pub fn as_str(&self) -> &str {
        match self {
            MimeType::TextHtml => "text/html",
            MimeType::TextCss => "text/css",
            MimeType::TextJavascript => "text/javascript",
            MimeType::ImageJpeg => "image/jpeg",
            MimeType::TextPlain => "text/plain",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mime_type_to_str() {
        assert_eq!(MimeType::TextHtml.to_str(), "text/html");
    }
}

pub fn get_mime_type<P: AsRef<Path>>(path: P) -> MimeType {
    let extension = path.as_ref().extension().unwrap_or_default();
    match extension.to_str() {
        Some("html") => MimeType::TextHtml,
        Some("css") => MimeType::TextCss,
        Some("js") => MimeType::TextJavascript,
        Some("jpg") => MimeType::ImageJpeg,
        _ => MimeType::TextPlain,
    }
}