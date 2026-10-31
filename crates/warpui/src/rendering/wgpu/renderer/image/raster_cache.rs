use std::borrow::Cow;
use std::sync::{Arc, Weak};

use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::vector::{Vector2F, vec2f};
use wgpu::{
    BindGroup, BindGroupLayout, ColorTargetState, Device, RenderPipeline, Texture, TextureView,
};

use crate::image_cache::StaticImage;
use crate::rendering::wgpu::resources::Resources;
use crate::rendering::wgpu::{resources, shader_types};
use crate::scene::{CornerRadius, Image};

#[derive(Clone)]
pub(super) struct RasterKey {
    asset: Weak<StaticImage>,
    bounds: RectF,
    opacity: f32,
    corner_radius: CornerRadius,
    scale_factor: f32,
    width: u32,
    height: u32,
    scissor: [u32; 4],
}

impl RasterKey {
    pub fn new(
        image: &Image,
        clip: Option<RectF>,
        scale_factor: f32,
        width: u32,
        height: u32,
    ) -> Option<Self> {
        // Bound the extra allocation to one 64 MiB bitmap per window.
        if width == 0 || height == 0 || u64::from(width) * u64::from(height) > 16 * 1024 * 1024 {
            return None;
        }
        let viewport = RectF::new(Vector2F::zero(), vec2f(width as f32, height as f32));
        let clip = clip.map_or(Some(viewport), |clip| {
            (clip * scale_factor).intersection(viewport)
        })?;
        let x = clip.min_x().round() as u32;
        let y = clip.min_y().round() as u32;
        let scissor = [
            x,
            y,
            (clip.max_x().round() as u32).saturating_sub(x),
            (clip.max_y().round() as u32).saturating_sub(y),
        ];
        if scissor[2] == 0 || scissor[3] == 0 {
            return None;
        }
        Some(Self {
            asset: Arc::downgrade(&image.asset),
            bounds: image.bounds,
            opacity: image.opacity,
            corner_radius: image.corner_radius,
            scale_factor,
            width,
            height,
            scissor,
        })
    }

    fn matches(&self, other: &Self) -> bool {
        self.asset.ptr_eq(&other.asset)
            && self.bounds == other.bounds
            && self.opacity == other.opacity
            && self.corner_radius == other.corner_radius
            && self.scale_factor == other.scale_factor
            && self.width == other.width
            && self.height == other.height
            && self.scissor == other.scissor
    }
}

struct CachedRaster {
    key: RasterKey,
    texture: Texture,
    view: TextureView,
    bind_group: BindGroup,
}

pub(super) struct RasterCache {
    pipeline: RenderPipeline,
    format: wgpu::TextureFormat,
    raster: Option<CachedRaster>,
}

impl RasterCache {
    pub fn new(
        device: &Device,
        layout: &wgpu::PipelineLayout,
        mut color_target: ColorTargetState,
    ) -> Self {
        let format = color_target.format;
        // Rasterizing onto transparent black already multiplied RGB by alpha.
        color_target.blend = Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING);
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Cached image shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "../../shaders/cached_image_shader.wgsl"
            ))),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Cached image blit"),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(shader_types::Vertex::desc())],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(color_target)],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        Self {
            pipeline,
            format,
            raster: None,
        }
    }

    pub fn clear(&mut self) {
        self.raster = None;
    }

    #[allow(clippy::too_many_arguments)]
    pub fn prepare(
        &mut self,
        resources: &Resources,
        encoder: &mut wgpu::CommandEncoder,
        key: &RasterKey,
        source_pipeline: &RenderPipeline,
        instances: &wgpu::Buffer,
        instance_index: usize,
        source: &BindGroup,
        layout: &BindGroupLayout,
        sampler: &wgpu::Sampler,
    ) {
        if self
            .raster
            .as_ref()
            .is_some_and(|raster| raster.key.matches(key))
        {
            return;
        }
        let texture = resources.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Cached background raster"),
            size: wgpu::Extent3d {
                width: key.width,
                height: key.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        let bind_group = resources
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Cached background binding"),
                layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(sampler),
                    },
                ],
            });
        let raster = CachedRaster {
            key: key.clone(),
            texture,
            view,
            bind_group,
        };
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Rasterize static background"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &raster.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            resources.configure_render_pass(
                &mut pass,
                vec2f(
                    raster.texture.width() as f32,
                    raster.texture.height() as f32,
                ),
            );
            let [x, y, width, height] = key.scissor;
            pass.set_scissor_rect(x, y, width, height);
            pass.set_pipeline(source_pipeline);
            pass.set_vertex_buffer(1, instances.slice(..));
            pass.set_bind_group(1, source, &[]);
            pass.draw_indexed(
                0..resources::quad::INDICES.len() as u32,
                0,
                instance_index as u32..instance_index as u32 + 1,
            );
        }
        self.raster = Some(raster);
    }

    pub fn copy_to_surface(&self, encoder: &mut wgpu::CommandEncoder, surface: &Texture) -> bool {
        let Some(raster) = &self.raster else {
            return false;
        };
        if !surface.usage().contains(wgpu::TextureUsages::COPY_DST)
            || surface.size() != raster.texture.size()
            || surface.format() != self.format
        {
            return false;
        }
        encoder.copy_texture_to_texture(
            raster.texture.as_image_copy(),
            surface.as_image_copy(),
            raster.texture.size(),
        );
        true
    }

    pub fn draw<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) -> bool {
        let Some(raster) = &self.raster else {
            return false;
        };
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(1, &raster.bind_group, &[]);
        pass.draw_indexed(0..resources::quad::INDICES.len() as u32, 0, 0..1);
        true
    }
}

#[cfg(test)]
#[path = "raster_cache_tests.rs"]
mod tests;
