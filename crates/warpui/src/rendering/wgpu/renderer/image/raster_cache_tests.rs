use super::*;
use crate::assets::asset_cache::Asset;
use crate::image_cache::{CustomImageFormat, CustomImageHeader, ImageType};
use crate::scene::Radius;

fn image() -> Image {
    let data =
        CustomImageHeader::prepend_custom_header(vec![128; 16], 2, 2, CustomImageFormat::Rgba)
            .unwrap();
    let ImageType::StaticBitmap { image } = ImageType::try_from_bytes(&data).unwrap() else {
        panic!("expected bitmap")
    };
    Image {
        cache_raster: true,
        bounds: RectF::new(vec2f(-10., -5.), vec2f(120., 60.)),
        asset: image,
        opacity: 0.7,
        corner_radius: CornerRadius::default(),
    }
}

#[test]
fn unchanged_image_reuses_raster_but_pixel_or_geometry_changes_invalidate_it() {
    let image = image();
    let original = RasterKey::new(&image, None, 1., 100, 50).unwrap();
    assert!(original.matches(&RasterKey::new(&image.clone(), None, 1., 100, 50).unwrap()));
    for change in 0..7 {
        let mut changed = image.clone();
        let mut scale = 1.;
        let mut width = 100;
        let mut height = 50;
        let mut clip = None;
        match change {
            0 => changed.opacity = 0.5,
            1 => changed.bounds = RectF::new(Vector2F::zero(), changed.bounds.size()),
            2 => changed.corner_radius = CornerRadius::with_top_left(Radius::Pixels(8.)),
            3 => scale = 2.,
            4 => width = 101,
            5 => height = 51,
            6 => clip = Some(RectF::new(vec2f(1., 0.), vec2f(99., 50.))),
            _ => unreachable!(),
        }
        assert!(
            !original.matches(&RasterKey::new(&changed, clip, scale, width, height).unwrap()),
            "change {change}"
        );
    }
}

#[test]
fn asset_replacement_does_not_reuse_old_pixels_or_keep_the_old_image_alive() {
    let first = image();
    let key = RasterKey::new(&first, None, 1., 100, 50).unwrap();
    let replacement = image();
    assert!(!key.matches(&RasterKey::new(&replacement, None, 1., 100, 50).unwrap()));
    drop(first);
    assert!(key.asset.upgrade().is_none());
}

#[test]
fn clipping_and_allocation_limits_do_not_create_invalid_render_targets() {
    let image = image();
    assert!(RasterKey::new(&image, None, 1., 0, 50).is_none());
    assert!(RasterKey::new(&image, None, 1., 8192, 8192).is_none());
    assert!(
        RasterKey::new(
            &image,
            Some(RectF::new(vec2f(100., 50.), vec2f(10., 10.))),
            1.,
            100,
            50
        )
        .is_none()
    );
    assert!(
        RasterKey::new(
            &image,
            Some(RectF::new(vec2f(0.1, 0.1), vec2f(0.1, 0.1))),
            1.,
            100,
            50
        )
        .is_none()
    );
    let clipped = RasterKey::new(
        &image,
        Some(RectF::new(vec2f(-10., -5.), vec2f(30.2, 20.7))),
        1.,
        100,
        50,
    )
    .unwrap();
    assert_eq!(clipped.scissor, [0, 0, 20, 16]);
}
