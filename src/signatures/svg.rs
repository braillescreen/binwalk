use crate::extractors::svg::extract_svg_image;
use crate::signatures::common::{SignatureError, SignatureResult, CONFIDENCE_MEDIUM};

/// Human readable description
pub const DESCRIPTION: &str = "SVG image";

/// SVG magic bytes
pub fn svg_magic() -> Vec<Vec<u8>> {
    return vec![b"<svg ".to_vec()];
}

/// Parse an SVG image
pub fn svg_parser(file_data: &Vec<u8>, offset: usize) -> Result<SignatureResult, SignatureError> {
    // Successful return value
    let mut result = SignatureResult {
        offset: offset,
        description: DESCRIPTION.to_string(),
        confidence: CONFIDENCE_MEDIUM,
        ..Default::default()
    };

    // Perform an extraction dry-run
    let dry_run = extract_svg_image(file_data, offset, None);

    // If the dry-run was a success, this is probably a valid JPEG file
    if dry_run.success == true {
        // Get the total size of the SVG
        if let Some(svg_size) = dry_run.size {
            // If this file, from start to finish, is just a SVG, there's no need to extract it
            if offset == 0 && file_data.len() == svg_size {
                result.extraction_declined = true;
            }

            // Report signature result
            result.size = svg_size;
            result.description =
                format!("{}, total size: {} bytes", result.description, result.size);
            return Ok(result);
        }
    }

    return Err(SignatureError);
}
