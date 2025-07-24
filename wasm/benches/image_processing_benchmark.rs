use criterion::{criterion_group, criterion_main, Criterion, black_box};
use image2ascii::image_to_ascii_art;
use image::ImageReader;

fn load_test_image() -> image::DynamicImage {
    // Make sure the path to the test image is correct.
    // This path is relative to the root of the crate.
    ImageReader::open("../assert/EVA.jpg").unwrap().decode().unwrap()
}

fn benchmark_image_to_ascii_art(c: &mut Criterion) {
    let img = load_test_image();
    let ascii_chars_edge: Vec<char> = " -/|\\".chars().collect();
    let ascii_chars_gray: Vec<char> = "@?OPoc:. ".chars().collect();

    c.bench_function("image_to_ascii_art_benchmark", |b| {
        b.iter(|| {
            image_to_ascii_art(
                black_box(&img),
                black_box(8),   // downsample_rate
                black_box(50),  // edge_sobel_threshold
                black_box(&ascii_chars_edge),
                black_box(&ascii_chars_gray),
            )
        })
    });
}

criterion_group!(benches, benchmark_image_to_ascii_art);
criterion_main!(benches);