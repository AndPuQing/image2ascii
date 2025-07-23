use image::{DynamicImage, GenericImageView, GrayImage, Luma, Pixel, Rgb, imageops};
use imageproc::gradients;
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;
use rayon::prelude::*;
use wasm_bindgen::prelude::*;

// --- Data Structures for Serialization ---

#[derive(Serialize, Deserialize)]
pub struct AsciiCharInfo {
    char: char,
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Serialize, Deserialize)]
pub struct AsciiArtOutput {
    lines: Vec<Vec<AsciiCharInfo>>,
    width: u32,
    height: u32,
}

// --- Public WASM-bindgen Functions ---

#[wasm_bindgen]
pub fn set_panic_hook() {
    // Set a panic hook for better error messages in the browser console.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

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
fn image_to_ascii_art(
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

    // Calculate Sobel gradients
    let sobel_h = gradients::horizontal_sobel(&gray_img_base);
    let sobel_v = gradients::vertical_sobel(&gray_img_base);

    // Create a map of normalized gradient angles
    let mut theta_normalized_map = GrayImage::new(orig_width, orig_height);
    theta_normalized_map
        .as_mut()
        .par_chunks_mut(orig_width as usize)
        .enumerate()
        .for_each(|(y, row)| {
            for (x, pixel) in row.iter_mut().enumerate() {
                let sx = sobel_h.get_pixel(x as u32, y as u32)[0] as f32;
                let sy = sobel_v.get_pixel(x as u32, y as u32)[0] as f32;
                let magnitude = (sx * sx + sy * sy).sqrt();
                let mut normalized_angle_val = 0.0f32;

                if magnitude >= edge_sobel_threshold as f32 {
                    let angle_rad = sy.atan2(sx); // Range: -PI to PI
                    normalized_angle_val = (angle_rad / PI) * 0.5 + 0.5; // Normalize to 0.0 - 1.0
                }
                *pixel = (normalized_angle_val * 255.0) as u8;
            }
        });

    // Quantize, pool, and resize the edge map
    let theta_quantized_fullsize = quantize_gray_image(&theta_normalized_map, num_edge_levels);
    let pooled_theta_map = max_pooling_gray_image(&theta_quantized_fullsize, downsample_rate);
    imageops::resize(
        &pooled_theta_map,
        output_grid_width,
        output_grid_height,
        imageops::FilterType::Nearest,
    )
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

/// Downsamples a grayscale image using max pooling.
fn max_pooling_gray_image(image: &GrayImage, pool_size: u32) -> GrayImage {
    if pool_size <= 1 { return image.clone(); }

    let (width, height) = image.dimensions();
    let pooled_width = (width / pool_size).max(1);
    let pooled_height = (height / pool_size).max(1);
    let mut pooled_image = GrayImage::new(pooled_width, pooled_height);

    pooled_image
        .par_chunks_mut(pooled_width as usize)
        .enumerate()
        .for_each(|(y_out, row)| {
            for (x_out, pixel) in row.iter_mut().enumerate() {
                let x_start = (x_out as u32) * pool_size;
                let y_start = (y_out as u32) * pool_size;
                let mut max_val = 0u8;
                for r_pool in 0..pool_size {
                    for c_pool in 0..pool_size {
                        let current_x = x_start + c_pool;
                        let current_y = y_start + r_pool;
                        if current_x < width && current_y < height {
                            max_val = max_val.max(image.get_pixel(current_x, current_y)[0]);
                        }
                    }
                }
                *pixel = max_val;
            }
        });
    pooled_image
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
