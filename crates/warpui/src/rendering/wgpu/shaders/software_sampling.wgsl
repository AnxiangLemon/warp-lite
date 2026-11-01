// UI textures have one mip level. Explicit bilinear loads avoid WARP's expensive sampler path.
override software_sampling: bool = false;

fn bilinear_load(source: texture_2d<f32>, uv: vec2<f32>) -> vec4<f32> {
    let dimensions = textureDimensions(source);
    let point = uv * vec2<f32>(dimensions) - vec2(0.5);
    let base = vec2<i32>(floor(point));
    let weight = fract(point);
    let last = vec2<i32>(dimensions) - vec2<i32>(1);
    let p00 = textureLoad(source, clamp(base, vec2<i32>(0), last), 0);
    let p10 = textureLoad(source, clamp(base + vec2<i32>(1, 0), vec2<i32>(0), last), 0);
    let p01 = textureLoad(source, clamp(base + vec2<i32>(0, 1), vec2<i32>(0), last), 0);
    let p11 = textureLoad(source, clamp(base + vec2<i32>(1, 1), vec2<i32>(0), last), 0);
    return mix(mix(p00, p10, weight.x), mix(p01, p11, weight.x), weight.y);
}
