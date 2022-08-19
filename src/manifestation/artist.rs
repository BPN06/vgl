use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
};

use crate::{manifestation::Screen, Engine};

pub mod commands;
pub mod pipeline;

impl Engine<Screen> {
    /*
    pub fn game_loop<F>(self, mut closure: F)
    where
        F: 'static + FnMut(&mut Engine),
    {
        self.event_loop(move |engine: &mut Engine| {
            closure(engine);

            engine.render();
        });
    }
    */

    pub fn event_loop<F>(self, closure: F)
    where
        F: 'static + FnMut(&mut Engine<Screen>) -> Result<(), ()>,
    {
        match self.config.any_thread {
            true => self.run_return(closure),
            false => self.run(closure),
        }
    }

    pub fn run<F>(mut self, mut closure: F)
    where
        F: 'static + FnMut(&mut Engine<Screen>) -> Result<(), ()>,
    {
        self.take_event_loop().run(move |event, _, control_flow| {
            self.event(event, control_flow, &mut closure);
        });
    }

    pub fn run_return<F>(mut self, mut closure: F)
    where
        F: FnMut(&mut Engine<Screen>) -> Result<(), ()>,
    {
        self.take_event_loop().run_return(|event, _, control_flow| {
            self.event(event, control_flow, &mut closure);
        });
    }

    pub fn run_once<F>(&mut self, mut closure: F)
    where
        F: FnMut(&mut Engine<Screen>) -> Result<(), ()>,
    {
        let mut event_loop = self.take_event_loop();

        event_loop.run_return(|event, _, control_flow| {
            self.event(event, control_flow, &mut |engine: &mut Engine<Screen>| {
                engine.config.control_flow = ControlFlow::Exit;
                closure(engine)
            });
        });

        self.renderer.event_loop = Some(event_loop);
    }

    pub fn event<F, T>(&mut self, event: Event<T>, control_flow: &mut ControlFlow, closure: &mut F)
    where
        F: FnMut(&mut Engine<Screen>) -> Result<(), ()>,
    {
        *control_flow = self.config.control_flow;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                self.resize(size);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
                self.config.control_flow = ControlFlow::Exit;
            }

            Event::RedrawRequested(_) => {}

            Event::MainEventsCleared => {
                if closure(self).is_ok() {
                    self.renderer.window.request_redraw();
                }
            }
            _ => {}
        }
    }

    pub fn take_event_loop(&mut self) -> EventLoop<()> {
        self.renderer.event_loop.take().unwrap()
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.renderer.size = new_size;

            self.renderer.config.width = new_size.width;
            self.renderer.config.height = new_size.height;

            self.configure_surface();
        }
    }

    pub fn configure_surface(&mut self) {
        self.renderer
            .surface
            .configure(&self.renderer.device, &self.renderer.config);
    }
}
