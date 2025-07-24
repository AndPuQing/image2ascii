use image::{DynamicImage, GenericImageView, GrayImage, Pixel, Rgb, imageops};
use imageproc::gradients;
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;
use rayon::prelude::*;
use wasm_bindgen::prelude::*;

// --- Data Structures for Serialization ---

#[derive(Serialize, Deserialize)]
pub struct AsciiCharInfo {
    pub char: char,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Serialize, Deserialize)]
pub struct AsciiArtOutput {
    pub lines: Vec<Vec<AsciiCharInfo>>,
    pub width: u32,
    pub height: u32,
}

// --- Public WASM-bindgen Functions ---

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn set_panic_hook() {
    // Set a panic hook for better error messages in the browser console.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn render(
    image_bytes: &[u8],
    downsample_rate: u32,
    edge_sobel_threshold: u32,
    ascii_chars_edge_str: &str,
    ascii_chars_gray_str: &str,
) -> Result<JsValue, JsValue> {
    set_panic_hook();

    // --- 1. Input Validation and Setup ---
    if downsample_rate == 0 {
        return Err(JsValue::from_str("downsample_rate must be positive."));
    }
    if ascii_chars_gray_str.is_empty() {
        return Err(JsValue::from_str("Grayscale ASCII characters cannot be empty."));
    }

    let img = image::load_from_memory(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to load image: {}", e)))?;

    let ascii_chars_gray: Vec<char> = ascii_chars_gray_str.chars().collect();
    let ascii_chars_edge: Vec<char> = ascii_chars_edge_str.chars().collect();

    // --- 2. Core Image Processing ---
    let result = image_to_ascii_art(
        &img,
        downsample_rate,
        edge_sobel_threshold,
        &ascii_chars_edge,
        &ascii_chars_gray,
    );

    // --- 3. Serialize Output for JavaScript ---
    match result {
        Ok(output) => serde_wasm_bindgen::to_value(&output)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e))),
        Err(e) => Err(JsValue::from_str(&e)),
    }
}

// --- Core Logic ---

/// Converts an image to ASCII art. This is the main processing function.
pub fn image_to_ascii_art(
    img: &DynamicImage,
    downsample_rate: u32,
    edge_sobel_threshold: u32,
    ascii_chars_edge: &[char],
    ascii_chars_gray: &[char],
) -> Result<AsciiArtOutput, String> {
    let num_edge_levels = ascii_chars_edge.len() as u32;
    let num_gray_levels = ascii_chars_gray.len() as u32;

    // --- 1. Calculate Output Dimensions ---
    let (orig_width, orig_height) = img.dimensions();
    let output_grid_width = (orig_width / downsample_rate).max(1);
    // Halve the height to account for non-square character aspect ratio
    let output_grid_height = ((orig_height / downsample_rate) / 2).max(1);

    // --- 2. Prepare Image Maps ---
    // Color map for final character coloring
    let small_img_for_color_sampling = imageops::resize(
        img,
        output_grid_width,
        output_grid_height,
        imageops::FilterType::Triangle,
    );

    // Grayscale map for non-edge character selection
    let quantized_gray_base =
        prepare_base_gray_image(img, output_grid_width, output_grid_height, num_gray_levels);

    // Edge map for edge character selection
    let edge_map_resized = create_edge_map(
        img,
        edge_sobel_threshold,
        num_edge_levels,
        downsample_rate,
        output_grid_width,
        output_grid_height,
    );

    // --- 3. Combine Maps and Generate ASCII ---
    let color_map_dynamic = image::DynamicImage::ImageRgba8(small_img_for_color_sampling);
    let output_lines = combine_and_map_to_ascii(
        output_grid_width,
        output_grid_height,
        &edge_map_resized,
        &quantized_gray_base,
        &color_map_dynamic,
        ascii_chars_edge,
        ascii_chars_gray,
    );

    Ok(AsciiArtOutput {
        lines: output_lines,
        width: output_grid_width,
        height: output_grid_height,
    })
}

// --- Helper Functions for Image Processing ---

/// Creates a map of edges from the image using Sobel operator.
/// This optimized version combines gradient calculation, pooling, and quantization
/// into a single loop to reduce memory allocations and improve performance.
fn create_edge_map(
    img: &DynamicImage,
    edge_sobel_threshold: u32,
    num_edge_levels: u32,
    downsample_rate: u32,
    output_grid_width: u32,
    output_grid_height: u32,
) -> GrayImage {
    let (orig_width, orig_height) = img.dimensions();
    let gray_img_base = img.to_luma8();

    // Calculate Sobel gradients once. This is memory-intensive but might be faster
    // than re-calculating inside the loop, as the imageproc crate is likely optimized.
    let sobel_h = gradients::horizontal_sobel(&gray_img_base);
    let sobel_v = gradients::vertical_sobel(&gray_img_base);

    let mut final_edge_map = GrayImage::new(output_grid_width, output_grid_height);
    let edge_sobel_threshold_f32 = edge_sobel_threshold as f32;

    // Aspect ratio correction for character cells (usually taller than wide)
    let y_downsample_rate = downsample_rate * 2;

    // Pre-calculate quantization step
    let quant_step = if num_edge_levels > 0 {
        (256.0f32 / num_edge_levels as f32).max(1.0f32) as u8
    } else {
        0
    };

    final_edge_map
        .par_chunks_mut(output_grid_width as usize)
        .enumerate()
        .for_each(|(y_out, row)| {
            for (x_out, pixel) in row.iter_mut().enumerate() {
                let x_start = (x_out as u32) * downsample_rate;
                let y_start = (y_out as u32) * y_downsample_rate;
                let x_end = (x_start + downsample_rate).min(orig_width);
                let y_end = (y_start + y_downsample_rate).min(orig_height);

                let mut max_magnitude = 0.0f32;
                let mut angle_at_max_magnitude = 0.0f32;

                // Max pooling for gradient magnitude within the target window
                for y in y_start..y_end {
                    for x in x_start..x_end {
                        let sx = sobel_h.get_pixel(x, y)[0] as f32;
                        let sy = sobel_v.get_pixel(x, y)[0] as f32;
                        let magnitude = (sx * sx + sy * sy).sqrt();
                        if magnitude > max_magnitude {
                            max_magnitude = magnitude;
                            angle_at_max_magnitude = sy.atan2(sx);
                        }
                    }
                }

                let mut final_pixel_value = 0;
                if max_magnitude >= edge_sobel_threshold_f32 {
                    // Normalize angle to 0.0 - 1.0
                    let normalized_angle = (angle_at_max_magnitude / PI) * 0.5 + 0.5;
                    let angle_val_u8 = (normalized_angle * 255.0) as u8;

                    // Quantize the value, similar to the original `quantize_gray_image`
                    if quant_step > 0 {
                        final_pixel_value = (angle_val_u8 / quant_step) * quant_step;
                    } else {
                        final_pixel_value = angle_val_u8;
                    }
                }
                *pixel = final_pixel_value;
            }
        });

    final_edge_map
}

/// Prepares a resized and quantized grayscale version of the image.
fn prepare_base_gray_image(
    img: &DynamicImage,
    output_grid_width: u32,
    output_grid_height: u32,
    num_gray_levels: u32,
) -> GrayImage {
    let gray_for_base_processing = img.to_luma8();
    let gray_base_resized = imageops::resize(
        &gray_for_base_processing,
        output_grid_width,
        output_grid_height,
        imageops::FilterType::Triangle,
    );
    quantize_gray_image(&gray_base_resized, num_gray_levels)
}

/// Combines edge, grayscale, and color data to generate the final ASCII art lines.
fn combine_and_map_to_ascii(
    width: u32,
    height: u32,
    edge_map: &GrayImage,
    gray_map: &GrayImage,
    color_map: &DynamicImage,
    ascii_chars_edge: &[char],
    ascii_chars_gray: &[char],
) -> Vec<Vec<AsciiCharInfo>> {
    let num_edge_levels = ascii_chars_edge.len() as u32;
    let num_gray_levels = ascii_chars_gray.len() as u32;
    let edge_quant_step = (256.0f32 / num_edge_levels as f32).max(1.0f32) as u8;
    let gray_quant_step = (256.0f32 / num_gray_levels as f32).max(1.0f32) as u8;
    let non_edge_char_indicator_val = 0;

    (0..height)
        .into_par_iter()
        .map(|y_grid| {
            (0..width)
                .map(|x_grid| {
                    // Determine character from edge map or grayscale map
                    let edge_pixel_val = edge_map.get_pixel(x_grid, y_grid)[0];
                    let edge_char_idx = (edge_pixel_val / edge_quant_step) as usize;

                    let char_to_print = if edge_char_idx != non_edge_char_indicator_val
                        || edge_pixel_val > (edge_quant_step / 2)
                    {
                        // Use edge character
                        ascii_chars_edge
                            .get(edge_char_idx.min(ascii_chars_edge.len() - 1))
                            .copied()
                            .unwrap_or(ascii_chars_edge[0])
                    } else {
                        // Use grayscale character
                        let gray_pixel_val = gray_map.get_pixel(x_grid, y_grid)[0];
                        let idx_gray = (gray_pixel_val / gray_quant_step) as usize;
                        ascii_chars_gray
                            .get(idx_gray.min(ascii_chars_gray.len() - 1))
                            .copied()
                            .unwrap_or(' ')
                    };

                    // Get color from the resized color image
                    let Rgb(color_pixel_data) = color_map.get_pixel(x_grid, y_grid).to_rgb();

                    AsciiCharInfo {
                        char: char_to_print,
                        r: color_pixel_data[0],
                        g: color_pixel_data[1],
                        b: color_pixel_data[2],
                    }
                })
                .collect::<Vec<AsciiCharInfo>>()
        })
        .collect()
}

// --- Utility Functions ---

/// Reduces the number of gray levels in an image.
fn quantize_gray_image(image: &GrayImage, num_levels: u32) -> GrayImage {
    if num_levels == 0 { return image.clone(); }
    let step = (256.0f32 / num_levels as f32).max(1.0f32) as u8;
    if step == 0 { return image.clone(); }

    let mut quantized_img = GrayImage::new(image.width(), image.height());
    quantized_img
        .as_mut()
        .par_iter_mut()
        .zip(image.as_raw().par_iter())
        .for_each(|(quantized_pixel, original_pixel)| {
            *quantized_pixel = (*original_pixel / step) * step;
        });
    quantized_img
}


// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_generate_ascii_art_wasm() {
        let image_bytes = include_bytes!("../../assert/EVA.jpg");
        let result = render(image_bytes, 4, 50, " -/|\\", "@?OPoc:. ");
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_set_panic_hook() {
        set_panic_hook();
        assert!(true); // Just ensures it doesn't panic
    }
}
