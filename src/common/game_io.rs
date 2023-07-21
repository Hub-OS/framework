use crate::prelude::*;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::future::Future;
use std::time::Duration;

pub struct GameIO {
    window: Window,
    graphics: GraphicsContext,
    resources: HashMap<TypeId, Box<dyn Any>>,
    async_executor: async_executor::LocalExecutor<'static>,
    input_manager: InputManager,
    target_fps: u16,
    game_start_instant: Instant,
    frame_start_instant: Instant,
    update_duration: Duration,
    draw_duration: Duration,
    frame_duration: Duration,
    sleep_duration: Duration,
    lost_duration: Duration,
    buffer_aquire_duration: Duration,
    transitioning: bool,
    quitting: bool,
}

impl GameIO {
    pub(crate) fn new(window: Window, graphics: GraphicsContext) -> Self {
        Self {
            window,
            graphics,
            resources: HashMap::new(),
            async_executor: async_executor::LocalExecutor::new(),
            input_manager: InputManager::new(),
            target_fps: 60,
            game_start_instant: Instant::now(),
            frame_start_instant: Instant::now(),
            update_duration: Duration::ZERO,
            draw_duration: Duration::ZERO,
            frame_duration: Duration::ZERO,
            sleep_duration: Duration::ZERO,
            lost_duration: Duration::ZERO,
            buffer_aquire_duration: Duration::ZERO,
            transitioning: false,
            quitting: false,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn graphics(&self) -> &GraphicsContext {
        &self.graphics
    }

    pub fn graphics_mut(&mut self) -> &mut GraphicsContext {
        &mut self.graphics
    }

    pub fn input(&self) -> &InputManager {
        &self.input_manager
    }

    pub fn input_mut(&mut self) -> &mut InputManager {
        &mut self.input_manager
    }

    pub fn resource<R: Any>(&self) -> Option<&R> {
        self.resources.get(&TypeId::of::<R>())?.downcast_ref::<R>()
    }

    pub fn resource_mut<R: Any>(&mut self) -> Option<&mut R> {
        self.resources
            .get_mut(&TypeId::of::<R>())?
            .downcast_mut::<R>()
    }

    pub fn set_resource<R: Any>(&mut self, r: R) {
        self.resources.insert(r.type_id(), Box::new(r));
    }

    pub fn target_fps(&self) -> u16 {
        self.target_fps
    }

    pub fn set_target_fps(&mut self, fps: u16) {
        self.target_fps = fps;
    }

    pub fn game_start_instant(&self) -> Instant {
        self.game_start_instant
    }

    pub fn frame_start_instant(&self) -> Instant {
        self.frame_start_instant
    }

    /// The target duration calculated from target_fps
    pub fn target_duration(&self) -> Duration {
        let target_seconds = 1.0 / self.target_fps() as f64;
        Duration::from_secs_f64(target_seconds)
    }

    /// Time spent in update functions and handling input for the last frame
    pub fn update_duration(&self) -> Duration {
        self.update_duration
    }

    /// Time spent in draw functions and setting up draw commands for the last frame
    pub fn draw_duration(&self) -> Duration {
        self.draw_duration
    }

    /// Time spent working on the last frame, update_duration and draw_duration are included in this time.
    /// Remaining time includes sleep_duration and waiting for a previous render
    pub fn frame_duration(&self) -> Duration {
        self.frame_duration
    }

    /// Time lost after the last frame due to over sleeping
    pub fn lost_duration(&self) -> Duration {
        self.lost_duration
    }

    /// Time spent sleeping after the last frame, includes attempted_sleep_duration and lost_duration
    pub fn sleep_duration(&self) -> Duration {
        self.sleep_duration + self.lost_duration
    }

    /// The duration the game tried to sleep for last frame
    pub fn attempted_sleep_duration(&self) -> Duration {
        self.sleep_duration
    }

    pub fn spawn_local_task<T: 'static>(
        &self,
        future: impl Future<Output = T> + 'static,
    ) -> AsyncTask<T> {
        let task = self.async_executor.spawn(future);
        AsyncTask::new(task)
    }

    pub fn is_in_transition(&self) -> bool {
        self.transitioning
    }

    pub(crate) fn set_transitioning(&mut self, transitioning: bool) {
        self.transitioning = transitioning;
    }

    pub fn is_quitting(&self) -> bool {
        self.quitting
    }

    pub fn quit(&mut self) {
        self.quitting = true;
    }

    pub fn cancel_quit(&mut self) {
        self.quitting = false;
    }

    pub(super) fn handle_tasks(&self) {
        while self.async_executor.try_tick() {}
    }

    pub(super) fn handle_events(&mut self, events: Vec<WindowEvent>) {
        if self.input_manager.accepting_text() != self.input_manager.accepting_text_last_frame() {
            self.window
                .set_text_input(self.input_manager.accepting_text());
        }

        self.input_manager.flush();

        for event in events {
            match event {
                WindowEvent::CloseRequested => {
                    self.quitting = true;
                }
                WindowEvent::Moved(position) => {
                    self.window.moved(position);
                }
                WindowEvent::Resized(size) => {
                    self.window.resized(size);
                    self.graphics.resized(size);
                }
                WindowEvent::InputEvent(mut input_event) => {
                    input_event.scale_mouse_event(&self.window);
                    self.input_manager.handle_event(input_event);
                }
            }
        }
    }

    pub(super) fn set_frame_start_instant(&mut self, instant: Instant) {
        self.frame_start_instant = instant;
    }

    pub(super) fn set_update_duration(&mut self, duration: Duration) {
        self.update_duration = duration;
    }

    pub(super) fn set_draw_duration(&mut self, duration: Duration) {
        self.draw_duration = duration;
    }

    pub(super) fn set_frame_duration(&mut self, duration: Duration) {
        self.frame_duration = duration;
    }

    pub(super) fn set_lost_duration(&mut self, duration: Duration) {
        self.lost_duration = duration;
    }

    pub(super) fn set_buffer_aquire_duration(&mut self, duration: Duration) {
        self.buffer_aquire_duration = duration;
    }

    pub(crate) fn update_sleep_duration(&mut self) {
        // adding lost_duration to frame_duration to catch up
        // subtracting buffer_aquire_duration to remain synced with vsync
        let used_duration = self.frame_duration + self.lost_duration - self.buffer_aquire_duration;

        let target_seconds = 1.0 / self.target_fps() as f64;
        let remaining_seconds = target_seconds - used_duration.as_secs_f64();

        self.sleep_duration = if remaining_seconds > 0.0 {
            Duration::from_secs_f64(remaining_seconds)
        } else {
            Duration::ZERO
        };
    }
}
