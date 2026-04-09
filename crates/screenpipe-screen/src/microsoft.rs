use anyhow::Result;
use image::{DynamicImage, GenericImageView};
use screenpipe_core::Language;
use tracing::{debug, warn};

#[cfg(target_os = "windows")]
pub async fn perform_ocr_windows(
    image: &DynamicImage,
    languages: &[Language],
) -> Result<(String, String, Option<f64>)> {
    use std::io::Cursor;
    use windows::{
        Graphics::Imaging::BitmapDecoder,
        Storage::Streams::{DataWriter, InMemoryRandomAccessStream},
    };

    // Check image dimensions
    let (width, height) = image.dimensions();
    if width == 0 || height == 0 {
        // Return an empty result instead of panicking
        return Ok(("".to_string(), "[]".to_string(), None));
    }

    let mut buffer = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png)
        .map_err(|e| anyhow::anyhow!("Failed to write image to buffer: {}", e))?;

    let stream = InMemoryRandomAccessStream::new()?;
    let writer = DataWriter::CreateDataWriter(&stream)?;
    writer.WriteBytes(&buffer)?;
    writer.StoreAsync()?.get()?;
    writer.FlushAsync()?.get()?;
    stream.Seek(0)?;

    let decoder =
        BitmapDecoder::CreateWithIdAsync(BitmapDecoder::PngDecoderId()?, &stream)?.get()?;

    let bitmap = decoder.GetSoftwareBitmapAsync()?.get()?;

    let engine = resolve_windows_ocr_engine(languages)?;
    let result = engine.RecognizeAsync(&bitmap)?.get()?;

    let mut full_text = String::new();
    let mut ocr_results: Vec<serde_json::Value> = Vec::new();

    // Try to iterate through lines and words to get bounding boxes
    // The Windows OCR API returns lines, each containing words with bounding rects
    let lines = result.Lines()?;
    for line in lines {
        let words = line.Words()?;
        for word in words {
            let text = word.Text()?;
            let text_str = text.to_string();
            if !text_str.is_empty() {
                if !full_text.is_empty() {
                    full_text.push(' ');
                }
                full_text.push_str(&text_str);

                // Get bounding box and normalize to 0-1 range (matching Apple Vision output)
                let rect = word.BoundingRect()?;
                let img_w = width as f32;
                let img_h = height as f32;
                ocr_results.push(serde_json::json!({
                    "text": text_str,
                    "left": (rect.X / img_w).to_string(),
                    "top": (rect.Y / img_h).to_string(),
                    "width": (rect.Width / img_w).to_string(),
                    "height": (rect.Height / img_h).to_string(),
                    "conf": "1.0"  // Windows OCR doesn't provide word-level confidence
                }));
            }
        }
    }

    // Fallback if no words were extracted
    if full_text.is_empty() {
        full_text = result.Text()?.to_string();
    }

    let json_output = serde_json::to_string(&ocr_results).unwrap_or_else(|_| "[]".to_string());

    Ok((full_text, json_output, Some(1.0)))
}

#[cfg(target_os = "windows")]
fn resolve_windows_ocr_engine(
    languages: &[Language],
) -> Result<windows::Media::Ocr::OcrEngine> {
    use windows::{
        core::HSTRING,
        Globalization::Language as WindowsLanguage,
        Media::Ocr::OcrEngine as WindowsOcrEngine,
    };

    for language in languages {
        for tag in windows_language_tags(language) {
            let windows_language = WindowsLanguage::CreateLanguage(&HSTRING::from(tag))?;
            if WindowsOcrEngine::IsLanguageSupported(&windows_language)? {
                debug!("windows OCR using explicit language hint {}", tag);
                return WindowsOcrEngine::TryCreateFromLanguage(&windows_language)
                    .map_err(anyhow::Error::from);
            }
        }
    }

    if !languages.is_empty() {
        warn!(
            "windows OCR did not find an installed recognizer for requested languages; falling back to user profile languages"
        );
    }

    WindowsOcrEngine::TryCreateFromUserProfileLanguages().map_err(anyhow::Error::from)
}

#[cfg(target_os = "windows")]
fn windows_language_tags(language: &Language) -> Vec<&'static str> {
    match language {
        Language::Chinese => vec!["zh-Hans", "zh-Hant", "zh-CN", "zh-TW", "zh"],
        Language::English => vec!["en-US", "en-GB", "en"],
        Language::French => vec!["fr-FR", "fr-CA", "fr"],
        Language::Portuguese => vec!["pt-BR", "pt-PT", "pt"],
        Language::Spanish => vec!["es-ES", "es-MX", "es"],
        _ => vec![language.as_lang_code()],
    }
}
