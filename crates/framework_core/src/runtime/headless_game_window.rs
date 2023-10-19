use crate::common::GameWindow;
use crate::graphics::{wgpu, Color, GraphicsContext, HasGraphicsContext, RenderTarget};
use crate::runtime::{GameWindowConfig, GameWindowLifecycle};
use math::*;

pub struct HeadlessGameWindow {
    graphics: GraphicsContext,
    render_target: Option<RenderTarget>,
    position: IVec2,
    size: UVec2,
    resolution: UVec2,
    locked_resolution: bool,
    clear_color: Option<Color>,
}

impl HeadlessGameWindow {
    pub(crate) async fn from_config(window_config: GameWindowConfig<()>) -> anyhow::Result<Self> {
        let graphics = GraphicsContext::new(wgpu::Instance::default(), None).await?;

        let size = window_config.size;
        let resolution = window_config.resolution.unwrap_or(size);
        let render_target = RenderTarget::new(&graphics, resolution);

        Ok(Self {
            graphics,
            render_target: Some(render_target),
            size: window_config.size,
            position: IVec2::new(0, 0),
            locked_resolution: window_config.resolution.is_some(),
            resolution,
            clear_color: None,
        })
    }
}

impl HasGraphicsContext for HeadlessGameWindow {
    fn graphics(&self) -> &GraphicsContext {
        &self.graphics
    }
}

impl GameWindowLifecycle for HeadlessGameWindow {
    fn rebuild_surface(&mut self) {}

    fn acquire_render_target(&mut self) -> Option<RenderTarget> {
        self.render_target.take()
    }

    fn present_frame(&mut self, render_target: RenderTarget) {
        self.render_target = Some(render_target);
    }

    fn moved(&mut self, position: IVec2) {
        self.position = position;
    }

    fn resized(&mut self, size: UVec2) {
        self.size = size;

        if !self.locked_resolution {
            self.resolution = size;
        }

        if let Some(render_target) = &mut self.render_target {
            render_target.resize(&self.graphics, self.resolution);
        }
    }

    fn set_accepting_text_input(&mut self, _accept: bool) {}
}

impl GameWindow for HeadlessGameWindow {
    fn position(&self) -> IVec2 {
        self.position
    }

    fn set_position(&mut self, position: IVec2) {
        self.position = position;
    }

    fn fullscreen(&self) -> bool {
        false
    }

    fn set_fullscreen(&mut self, _fullscreen: bool) {}

    fn size(&self) -> UVec2 {
        self.size
    }

    fn has_locked_resolution(&self) -> bool {
        self.locked_resolution
    }

    fn lock_resolution(&mut self, resolution: UVec2) {
        self.resolution = resolution;
        self.locked_resolution = true;
    }

    fn unlock_resolution(&mut self) {
        self.resolution = self.size;
        self.locked_resolution = false;
    }

    fn resolution(&self) -> UVec2 {
        self.resolution
    }

    fn set_title(&mut self, _title: &str) {}

    fn clear_color(&self) -> Option<Color> {
        self.clear_color
    }

    fn set_clear_color(&mut self, color: Option<Color>) {
        self.clear_color = color;
    }

    fn vsync_enabled(&self) -> bool {
        false
    }

    fn set_vsync_enabled(&mut self, _enabled: bool) {}
}
