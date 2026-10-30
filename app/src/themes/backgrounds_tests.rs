use std::io::Write as _;

use warpui::assets::asset_cache::Asset;
use warpui::image_cache::ImageType;

use super::*;

fn decoded_background(path: &Path) -> ImageType {
    let image = load_background(path).unwrap();
    let AssetSource::Async { fetch, .. } = image.source else {
        panic!("Expected an immutable startup snapshot");
    };
    let bytes = warpui::r#async::block_on(fetch()).unwrap();
    ImageType::try_from_bytes(&bytes).unwrap()
}

#[test]
fn missing_and_corrupt_backgrounds_are_rejected() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("background.png");
    assert!(load_background(&path).is_err());
    std::fs::write(&path, b"not a png").unwrap();
    assert!(load_background(&path).is_err());
    assert!(load_background(directory.path()).is_err());
}

#[test]
fn oversized_input_is_rejected_before_decoding() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("background.png");
    let file = File::create(&path).unwrap();
    file.set_len(MAX_FILE_BYTES + 1).unwrap();
    assert!(load_background(&path).is_err());
    image::RgbaImage::from_pixel(MAX_INPUT_DIMENSION + 1, 1, image::Rgba([0, 0, 0, 255]))
        .save(&path)
        .unwrap();
    assert!(load_background(&path).is_err());
}

#[test]
fn png_backgrounds_are_bounded_before_uploading_to_the_gpu() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("background.png");
    image::RgbaImage::from_pixel(2560, 1440, image::Rgba([12, 34, 56, 255]))
        .save(&path)
        .unwrap();
    let ImageType::StaticBitmap { image } = decoded_background(&path) else {
        panic!("Expected a static bitmap");
    };
    assert_eq!(image.width(), 2048);
    assert_eq!(image.height(), 1152);
    assert_eq!(&image.rgba_bytes()[..4], &[12, 34, 56, 255]);
}

#[test]
fn startup_pixels_survive_replacement_and_deletion_of_the_file() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("background.png");
    image::RgbaImage::from_pixel(2, 2, image::Rgba([12, 34, 56, 255]))
        .save(&path)
        .unwrap();
    let image = load_background(&path).unwrap();
    let AssetSource::Async { fetch, .. } = image.source else {
        panic!("Expected an immutable startup snapshot");
    };
    let original = warpui::r#async::block_on(fetch()).unwrap();
    File::create(&path)
        .unwrap()
        .write_all(b"corrupt replacement")
        .unwrap();
    assert_eq!(warpui::r#async::block_on(fetch()).unwrap(), original);
    std::fs::remove_file(&path).unwrap();
    assert_eq!(warpui::r#async::block_on(fetch()).unwrap(), original);
}

#[test]
fn jpeg_is_detected_by_contents_even_with_a_png_extension() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("background.png");
    image::RgbImage::from_pixel(2, 2, image::Rgb([12, 34, 56]))
        .save_with_format(&path, ImageFormat::Jpeg)
        .unwrap();
    let ImageType::StaticBitmap { image } = decoded_background(&path) else {
        panic!("Expected a static bitmap");
    };
    assert_eq!((image.width(), image.height()), (2, 2));
    assert_eq!(image.rgba_bytes()[3], 255);
}

#[test]
fn jpeg_backgrounds_are_bounded_before_uploading_to_the_gpu() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("background.jpeg");
    image::RgbImage::from_pixel(2560, 1440, image::Rgb([12, 34, 56]))
        .save(&path)
        .unwrap();
    let ImageType::StaticBitmap { image } = decoded_background(&path) else {
        panic!("Expected a static bitmap");
    };
    assert_eq!((image.width(), image.height()), (2048, 1152));
}

#[test]
fn oversized_jpeg_dimensions_are_rejected() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("background.jpg");
    image::RgbImage::from_pixel(MAX_INPUT_DIMENSION + 1, 1, image::Rgb([12, 34, 56]))
        .save(&path)
        .unwrap();
    assert!(load_background(&path).is_err());
}

#[test]
fn jpeg_exif_orientation_is_applied() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("background.jpg");
    image::RgbImage::from_pixel(2, 3, image::Rgb([12, 34, 56]))
        .save(&path)
        .unwrap();
    let mut jpeg = std::fs::read(&path).unwrap();
    // EXIF orientation 6 rotates a portrait image clockwise.
    jpeg.splice(
        2..2,
        [
            0xff, 0xe1, 0, 34, b'E', b'x', b'i', b'f', 0, 0, b'M', b'M', 0, 42, 0, 0, 0, 8, 0, 1,
            1, 0x12, 0, 3, 0, 0, 0, 1, 0, 6, 0, 0, 0, 0, 0, 0,
        ],
    );
    std::fs::write(&path, jpeg).unwrap();
    let ImageType::StaticBitmap { image } = decoded_background(&path) else {
        panic!("Expected a static bitmap");
    };
    assert_eq!((image.width(), image.height()), (3, 2));
}

#[test]
fn corrupt_png_falls_back_to_jpeg() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("background.png"), b"corrupt").unwrap();
    image::RgbImage::from_pixel(2, 2, image::Rgb([12, 34, 56]))
        .save(directory.path().join("background.jpeg"))
        .unwrap();
    assert!(load_background_from_directory(directory.path()).is_some());
}

#[test]
fn png_takes_precedence_over_jpeg() {
    let directory = tempfile::tempdir().unwrap();
    image::RgbaImage::from_pixel(2, 2, image::Rgba([12, 34, 56, 255]))
        .save(directory.path().join("background.png"))
        .unwrap();
    image::RgbImage::from_pixel(3, 3, image::Rgb([56, 34, 12]))
        .save(directory.path().join("background.jpg"))
        .unwrap();
    let background = load_background_from_directory(directory.path()).unwrap();
    let AssetSource::Async { fetch, .. } = background.source else {
        panic!("Expected an immutable startup snapshot");
    };
    let bytes = warpui::r#async::block_on(fetch()).unwrap();
    let ImageType::StaticBitmap { image } = ImageType::try_from_bytes(&bytes).unwrap() else {
        panic!("Expected a static bitmap");
    };
    assert_eq!((image.width(), image.height()), (2, 2));
    assert_eq!(&image.rgba_bytes()[..4], &[12, 34, 56, 255]);
}
