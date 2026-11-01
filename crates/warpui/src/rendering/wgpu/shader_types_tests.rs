use bytemuck::Zeroable;

use super::*;

#[test]
fn solid_fast_path_preserves_opacity_and_rejects_every_complex_edge() {
    let mut rect = RectData::zeroed();
    for alpha in [0., 0.5, 1.] {
        rect.background_color.start_color.a = alpha;
        rect.background_color.end_color.a = alpha;
        assert!(rect.is_plain_solid());
    }
    for edge in [
        vec4f(1., 0., 0., 0.),
        vec4f(0., 1., 0., 0.),
        vec4f(0., 0., 1., 0.),
        vec4f(0., 0., 0., 1.),
    ] {
        let mut bordered = rect;
        bordered.border_width = edge;
        assert!(!bordered.is_plain_solid());
        let mut rounded = rect;
        rounded.corner_radius = edge;
        assert!(!rounded.is_plain_solid());
    }
    let mut gradient = rect;
    gradient.background_color.end_color.r = 1.;
    assert!(!gradient.is_plain_solid());
    let mut alpha_gradient = rect;
    alpha_gradient.background_color.end_color.a = 0.5;
    assert!(!alpha_gradient.is_plain_solid());
    let mut shadow = rect;
    shadow.drop_shadow_sigma = 3.;
    assert!(!shadow.is_plain_solid());
}
