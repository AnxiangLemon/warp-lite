use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::vector::Vector2F;
use wgpu::{CommandEncoder, RenderPass, SurfaceTexture};

use crate::Scene;
use crate::rendering::wgpu::Resources;
use crate::rendering::wgpu::renderer::{WGPUContext, glyph, image, rect};
use crate::scene::Layer;

#[derive(Default)]
struct PerFrameState {
    rect: rect::PerFrameState,
    glyph: glyph::PerFrameState,
    image: image::PerFrameState,
}

/// Struct responsible for rendering a frame by issuing draw calls.
pub(super) struct Frame<'a> {
    scene: &'a Scene,
    layer_state: Vec<LayerState<'a>>,
    per_frame_state: PerFrameState,
    rect_pipeline: &'a rect::Pipeline,
    glyph_pipeline: &'a mut glyph::Pipeline,
    image_pipeline: &'a mut image::Pipeline,
}

impl<'a> Frame<'a> {
    pub(super) fn new(
        scene: &'a Scene,
        ctx: &'a mut WGPUContext<'a>,
        rect_pipeline: &'a rect::Pipeline,
        glyph_pipeline: &'a mut glyph::Pipeline,
        image_pipeline: &'a mut image::Pipeline,
    ) -> Self {
        glyph_pipeline.update_config(&scene.rendering_config().glyphs);

        let mut layer_state = vec![];
        let mut per_frame_state = PerFrameState::default();

        for layer in scene.layers() {
            let rect_layer_state =
                rect_pipeline.initialize_for_layer(layer, scene, &mut per_frame_state.rect);
            let glyph_layer_state =
                glyph_pipeline.initialize_for_layer(layer, scene, &mut per_frame_state.glyph, ctx);
            let image_layer_state =
                image_pipeline.initialize_for_layer(layer, scene, &mut per_frame_state.image, ctx);
            layer_state.push(LayerState {
                layer,
                rect_layer_state,
                glyph_layer_state,
                image_layer_state,
            });
        }

        rect::Pipeline::finalize_per_frame_state(
            &mut per_frame_state.rect,
            &ctx.resources.device,
            &ctx.resources.device_lost,
        );
        glyph::Pipeline::finalize_per_frame_state(
            &mut per_frame_state.glyph,
            &ctx.resources.device,
            &ctx.resources.device_lost,
        );
        image::Pipeline::finalize_per_frame_state(
            &mut per_frame_state.image,
            &ctx.resources.device,
            &ctx.resources.device_lost,
        );

        Self {
            scene,
            layer_state,
            per_frame_state,
            rect_pipeline,
            glyph_pipeline,
            image_pipeline,
        }
    }

    /// Encodes draw calls into the [`wgpu::CommandEncoder`] to render the [`Scene`]. Callers are
    /// responsible for finishing the [`wgpu::CommandEncoder`] and actually presenting the current
    /// drawable on the screen.
    pub(super) fn draw(
        mut self,
        resources: &Resources,
        encoder: &mut CommandEncoder,
        surface_texture: &SurfaceTexture,
    ) {
        #[cfg(feature = "integration_tests")]
        let skip_draw = std::env::var("WARP_TEST_SKIP_DRAW").ok();
        #[cfg(not(feature = "integration_tests"))]
        let skip_draw: Option<String> = None;

        let surface_size = Vector2F::new(
            surface_texture.texture.width() as f32,
            surface_texture.texture.height() as f32,
        );

        if skip_draw.as_deref() != Some("images") {
            self.image_pipeline.prepare_raster_cache(
                resources,
                encoder,
                &self.per_frame_state.image,
            );
        }

        // A premultiplied raster can replace blending only when it is the first draw over clear pixels.
        let first_layer = self.layer_state.iter().find(|state| {
            let layer = state.layer;
            !layer.rects.is_empty()
                || !layer.images.is_empty()
                || !layer.icons.is_empty()
                || !layer.glyphs.is_empty()
        });
        let can_prefill = first_layer.is_some_and(|state| {
            state.layer.rects.is_empty()
                && state
                    .layer
                    .images
                    .first()
                    .is_some_and(|image| image.cache_raster)
        });
        let prefilled = skip_draw.as_deref() != Some("images")
            && can_prefill
            && self.image_pipeline.prefill_surface(
                encoder,
                &surface_texture.texture,
                &mut self.per_frame_state.image,
            );

        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(surface_texture.texture.format()),
                ..Default::default()
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: if prefilled {
                        wgpu::LoadOp::Load
                    } else {
                        wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
                    },
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        resources.configure_render_pass(&mut render_pass, surface_size);

        let device_bounds = RectF::new(Vector2F::zero(), surface_size);

        for layer_state in &self.layer_state {
            if let Some(bounds) = layer_state.layer.clip_bounds {
                // Make sure the scissor rect doesn't extend beyond the boundaries
                // of the window.
                let bounds = (bounds * self.scene.scale_factor()).intersection(device_bounds);
                let Some(intersection) = bounds else {
                    // The layer's clip bounds don't intersect the window bounds
                    // at all; we can skip drawing anything in this layer.
                    continue;
                };

                Self::set_scissor_rect(&mut render_pass, intersection);
            } else {
                Self::set_scissor_rect(&mut render_pass, device_bounds);
            }

            if let Some(rect_layer_state) = &layer_state.rect_layer_state
                && skip_draw.as_deref() != Some("rects")
            {
                self.rect_pipeline.draw(
                    &mut render_pass,
                    rect_layer_state,
                    &self.per_frame_state.rect,
                );
            }

            if let Some(image_layer_state) = &layer_state.image_layer_state
                && skip_draw.as_deref() != Some("images")
            {
                self.image_pipeline.draw(
                    &mut render_pass,
                    image_layer_state,
                    &self.per_frame_state.image,
                );
            }

            if let Some(glyph_layer_state) = &layer_state.glyph_layer_state
                && skip_draw.as_deref() != Some("glyphs")
            {
                self.glyph_pipeline.draw(
                    &mut render_pass,
                    glyph_layer_state,
                    &self.per_frame_state.glyph,
                );
            }
        }
    }

    fn set_scissor_rect(render_pass: &mut RenderPass<'_>, scissor_rect_bounds: RectF) {
        // Round the corners independently and derive width/height from those. Rounding origin and
        // size independently can produce a rect that extends beyond the surface when the origin
        // rounds up and the size also rounds up.
        let origin_x = scissor_rect_bounds.origin_x().round() as u32;
        let origin_y = scissor_rect_bounds.origin_y().round() as u32;
        let max_x = scissor_rect_bounds.max_x().round() as u32;
        let max_y = scissor_rect_bounds.max_y().round() as u32;
        let width = max_x.saturating_sub(origin_x);
        let height = max_y.saturating_sub(origin_y);

        // wgpu runtime assertions will fail if a scissor rect is set with a 0 width or height. See
        // https://github.com/gfx-rs/wgpu/issues/1750
        if height != 0 && width != 0 {
            render_pass.set_scissor_rect(origin_x, origin_y, width, height);
        }
    }
}

impl Drop for Frame<'_> {
    fn drop(&mut self) {
        // Let the image pipeline know that we've finished the frame so it can
        // perform cache cleanup.
        self.image_pipeline.end_frame();
    }
}

/// State for rendering a given [`Layer`] onto the screen.
struct LayerState<'a> {
    layer: &'a Layer,
    rect_layer_state: Option<rect::LayerState>,
    glyph_layer_state: Option<glyph::LayerState>,
    image_layer_state: Option<image::LayerState>,
}
