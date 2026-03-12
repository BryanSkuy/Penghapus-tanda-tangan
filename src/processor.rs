use image::{DynamicImage, ImageBuffer, Rgba};

/// Adjust contrast of the image to sharpen edges.
pub fn adjust_contrast(px: [u8; 4], contrast: f32) -> [u8; 4] {
    let [r, g, b, a] = px;
    let factor = (259.0 * (contrast + 255.0)) / (255.0 * (259.0 - contrast));
    
    let r = ((factor * (r as f32 - 128.0) + 128.0).clamp(0.0, 255.0)) as u8;
    let g = ((factor * (g as f32 - 128.0) + 128.0).clamp(0.0, 255.0)) as u8;
    let b = ((factor * (b as f32 - 128.0) + 128.0).clamp(0.0, 255.0)) as u8;
    
    [r, g, b, a]
}

/// Remove the background from a signature image.
pub fn remove_background(
    img: &DynamicImage,
    threshold: u8,
    color_tolerance: u8,
    contrast: f32,
) -> DynamicImage {
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    // First pass: mark transparent pixels
    let mut alpha_map: Vec<u8> = vec![255u8; (width * height) as usize];

    for y in 0..height {
        for x in 0..width {
            let px = rgba.get_pixel(x, y);
            let [r, g, b, _a] = adjust_contrast(px.0, contrast);

            // Perceived luminance (ITU-R BT.601)
            let lum = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;

            let min_chan = 255u8.saturating_sub(color_tolerance);
            let near_white = r >= min_chan && g >= min_chan && b >= min_chan;

            if lum >= threshold && near_white {
                alpha_map[(y * width + x) as usize] = 0;
            }
        }
    }

    // Edge feathering pass (simple 3x3 average on the alpha map)
    let feather_passes = 1u8;
    let mut feathered = alpha_map.clone();

    for _ in 0..feather_passes {
        let prev = feathered.clone();
        for y in 1..height.saturating_sub(1) {
            for x in 1..width.saturating_sub(1) {
                let idx = (y * width + x) as usize;
                // Only feather pixels on the boundary between INK and TRANSPARENT
                if prev[idx] == 255 {
                    let sum: u32 = [
                        prev[((y - 1) * width + x - 1) as usize] as u32,
                        prev[((y - 1) * width + x) as usize] as u32,
                        prev[((y - 1) * width + x + 1) as usize] as u32,
                        prev[(y * width + x - 1) as usize] as u32,
                        255u32,
                        prev[(y * width + x + 1) as usize] as u32,
                        prev[((y + 1) * width + x - 1) as usize] as u32,
                        prev[((y + 1) * width + x) as usize] as u32,
                        prev[((y + 1) * width + x + 1) as usize] as u32,
                    ]
                    .iter()
                    .sum();
                    feathered[idx] = (sum / 9) as u8;
                }
            }
        }
    }

    // Build result image
    let mut out: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let px = rgba.get_pixel(x, y);
            let [r, g, b, _] = px.0;
            let alpha = feathered[(y * width + x) as usize];
            out.put_pixel(x, y, Rgba([r, g, b, alpha]));
        }
    }

    DynamicImage::ImageRgba8(out)
}

/// Encode a `DynamicImage` as a base64 PNG data-URI.
pub fn to_data_uri(img: &DynamicImage) -> String {
    let mut buf: Vec<u8> = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)
        .expect("PNG encode failed");
    let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &buf);
    format!("data:image/png;base64,{}", b64)
}

/// Load any image file from `path` and return as DynamicImage + data-URI of original.
pub fn load_image(path: &str) -> Result<(DynamicImage, String), String> {
    let img = image::open(path).map_err(|e| format!("Gagal membuka gambar: {}", e))?;
    let data_uri = to_data_uri(&img);
    Ok((img, data_uri))
}
