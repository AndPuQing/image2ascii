use std::f32::consts::PI;

use image::{GenericImageView, GrayImage, Luma, Pixel, Rgb, imageops};
use imageproc::gradients;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

fn quantize_gray_image(image: &GrayImage, num_levels: u32) -> GrayImage {
    if num_levels == 0 {
        return image.clone();
    } // Or handle error appropriately
    // Ensure step is at least 1 to avoid division by zero if num_levels > 256
    let step = (256.0f32 / num_levels as f32).max(1.0f32) as u8;
    if step == 0 {
        return image.clone();
    } // Should be caught by max(1.0) but as a safeguard

    let mut quantized_img = GrayImage::new(image.width(), image.height());
    for y in 0..image.height() {
        for x in 0..image.width() {
            let val = image.get_pixel(x, y)[0];
            let quantized_val = (val / step) * step; // Integer division then multiplication
            quantized_img.put_pixel(x, y, Luma([quantized_val]));
        }
    }
    quantized_img
}

fn max_pooling_gray_image(image: &GrayImage, pool_size: u32) -> GrayImage {
    if pool_size == 0 || pool_size == 1 {
        return image.clone();
    }
    let (width, height) = image.dimensions();
    let pooled_width = (width / pool_size).max(1);
    let pooled_height = (height / pool_size).max(1);
    let mut pooled_image = GrayImage::new(pooled_width, pooled_height);

    for y_out in 0..pooled_height {
        for x_out in 0..pooled_width {
            let x_start = x_out * pool_size;
            let y_start = y_out * pool_size;
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
            pooled_image.put_pixel(x_out, y_out, Luma([max_val]));
        }
    }
    pooled_image
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

    if downsample_rate <= 0 {
        return Err(JsValue::from_str("downsample_rate must be positive."));
    }

    let img = match image::load_from_memory(image_bytes) {
        Ok(i) => i,
        Err(e) => return Err(JsValue::from_str(&format!("Failed to load image: {}", e))),
    };

    let ascii_chars_gray: Vec<char> = ascii_chars_gray_str.chars().collect();
    let ascii_chars_edge: Vec<char> = ascii_chars_edge_str.chars().collect();
    if ascii_chars_gray.is_empty() {
        return Err(JsValue::from_str(
            "Grayscale ASCII characters cannot be empty.",
        ));
    }
    let num_edge_levels = ascii_chars_edge.len() as u32;
    let num_gray_levels = ascii_chars_gray.len() as u32;

    let (orig_width, orig_height) = img.dimensions();

    let output_grid_width = (orig_width / downsample_rate).max(1);
    let output_grid_height = ((orig_height / downsample_rate) / 2).max(1);

    let gray_for_base_processing = img.to_luma8();
    let small_img_for_color_sampling = imageops::resize(
        &img, // Original color image
        output_grid_width,
        output_grid_height,
        imageops::FilterType::Triangle, // Good for downscaling
    );
    let gray_base_resized = imageops::resize(
        &gray_for_base_processing,
        output_grid_width,
        output_grid_height,
        imageops::FilterType::Triangle, // INTER_LINEAR equivalent
    );
    let quantized_gray_base = quantize_gray_image(&gray_base_resized, num_gray_levels);
    let gray_quant_step = (256.0f32 / num_gray_levels as f32).max(1.0f32) as u8;

    // --- Edge Detection ---
    let gray_img_base = img.into_luma8();
    let sobel_h = gradients::horizontal_sobel(&gray_img_base);
    let sobel_v = gradients::vertical_sobel(&gray_img_base);
    let mut theta_normalized_map = GrayImage::new(orig_width, orig_height);

    for y in 0..orig_height {
        for x in 0..orig_width {
            let sx = sobel_h.get_pixel(x, y)[0] as f32;
            let sy = sobel_v.get_pixel(x, y)[0] as f32;

            let magnitude = (sx * sx + sy * sy).sqrt();
            let mut normalized_angle_val = 0.0f32;

            if magnitude >= edge_sobel_threshold as f32 {
                let angle_rad = sy.atan2(sx); // Range: -PI to PI
                // Normalize to 0.0 - 1.0
                normalized_angle_val = (angle_rad / PI) * 0.5 + 0.5;
                normalized_angle_val = normalized_angle_val.clamp(0.0, 1.0);
            }
            theta_normalized_map.put_pixel(x, y, Luma([(normalized_angle_val * 255.0) as u8]));
        }
    }
    let theta_quantized_fullsize = quantize_gray_image(&theta_normalized_map, num_edge_levels);

    let pooled_theta_map = max_pooling_gray_image(&theta_quantized_fullsize, downsample_rate);

    // Resize pooled theta map to final output grid dimensions
    let edge_map_resized = imageops::resize(
        &pooled_theta_map,
        output_grid_width,
        output_grid_height,
        imageops::FilterType::Nearest, // INTER_NEAREST equivalent
    );
    // Quantization step for edge character mapping (based on num_edge_levels)
    let edge_quant_step = (256.0f32 / num_edge_levels as f32).max(1.0f32) as u8;

    // --- 3. Combine and Map to ASCII ---
    let mut output_lines: Vec<Vec<AsciiCharInfo>> = Vec::new();
    let non_edge_char_indicator_val = 0; // The value that maps to ascii_chars_edge[0]

    for y_grid in 0..output_grid_height {
        let mut current_line: Vec<AsciiCharInfo> = Vec::new();
        for x_grid in 0..output_grid_width {
            let char_to_print: char;

            // Determine Character from Edge Map or Grayscale Base
            let edge_pixel_val = edge_map_resized.get_pixel(x_grid, y_grid)[0];

            // If edge_pixel_val is 0, it means it was a sub-threshold edge or became 0 through quantization.
            // This should map to the first character in ascii_chars_edge.
            let edge_char_idx = (edge_pixel_val / edge_quant_step) as usize;

            if edge_char_idx != non_edge_char_indicator_val
                || edge_pixel_val > (edge_quant_step / 2)
            {
                // Heuristic to prefer actual edges
                char_to_print = ascii_chars_edge
                    .get(edge_char_idx.min(ascii_chars_edge.len() - 1))
                    .copied()
                    .unwrap_or(ascii_chars_edge[0]); // Default to non-edge if out of bounds
            } else {
                let gray_pixel_val = quantized_gray_base.get_pixel(x_grid, y_grid)[0];
                let idx_gray = (gray_pixel_val / gray_quant_step) as usize;
                char_to_print = ascii_chars_gray
                    .get(idx_gray.min(ascii_chars_gray.len() - 1))
                    .copied()
                    .unwrap_or(' '); // Default to space
            }

            // Get color from the resized color image (small_img_for_color_sampling)
            let Rgb(color_pixel_data) = small_img_for_color_sampling
                .get_pixel(x_grid, y_grid)
                .to_rgb();

            current_line.push(AsciiCharInfo {
                char: char_to_print,
                r: color_pixel_data[0],
                g: color_pixel_data[1],
                b: color_pixel_data[2],
            });
        }
        output_lines.push(current_line);
    }

    let result = AsciiArtOutput {
        lines: output_lines,
        width: output_grid_width,
        height: output_grid_height,
    };

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_generate_ascii_art_wasm() {
        let image_bytes = include_bytes!("../../assert/EVA.jpg");
        let result = render(image_bytes, 1, 0, "@#%*+=-:. ", "@#%*+=-:. ");
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_set_panic_hook() {
        set_panic_hook();
        // This test just ensures that the panic hook can be set without panicking.
        assert!(true);
    }
}
