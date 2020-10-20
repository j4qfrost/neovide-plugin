use skulpin::winit::event::WindowEvent;
use skulpin::winit::event_loop::{
    ControlFlow, EventLoopProxy,
};
use skulpin::winit::window::{Window, WindowId};
use skulpin::{
    winit::dpi::LogicalSize, Renderer as SkulpinRenderer,
};

pub trait NeovideEventProcessor {
    fn process_event(
        &mut self,
        e: WindowEvent,
        proxy: &EventLoopProxy<NeovideEvent>,
    ) -> Option<ControlFlow>;
}

pub trait WindowHandle: NeovideEventProcessor {
    fn init() -> Self where Self: Sized;
    fn window(&mut self) -> Window;
    fn set_window(&mut self, window: Window);
    fn logical_size(&self) -> LogicalSize<u32>;
    fn update(&mut self) -> bool;
    fn should_draw(&self) -> bool;
    fn draw(&mut self, skulpin_renderer: &mut SkulpinRenderer) -> bool;
}

pub trait NoopEvent {
    fn noop() -> Self;
}

#[derive(Debug)]
pub enum NeovideEvent {
    // Pause(WindowId),
    SwitchHandle(WindowId),
    Noop,
}

impl NoopEvent for NeovideEvent {
    fn noop() -> Self {
        NeovideEvent::Noop
    }
}
