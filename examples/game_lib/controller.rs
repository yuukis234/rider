use std::{thread, time};
use std::rc::Rc;
use std::cell::{Cell, RefCell};

use winit::{
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Wait,
    WaitUntil,
    Poll,
}

const WAIT_TIME: time::Duration = time::Duration::from_millis(100);
const POLL_SLEEP_TIME: time::Duration = time::Duration::from_millis(100);


// 値の型を調べる関数
fn type_of<T>(_: T) -> String{
    let a = std::any::type_name::<T>();
    return a.to_string();
}

#[derive(Debug, Clone, Copy)]
struct AppEvents {
    close_requested: bool,
}

impl AppEvents {
    pub fn close(&mut self) {
        self.close_requested = true;
    }
}

#[derive(Debug)]
struct Player {
    key_code: winit::event::VirtualKeyCode,
    app_events: RefCell<AppEvents>,
}

impl Player {
    pub fn hello(&self) {
        if (self.key_code == winit::event::VirtualKeyCode::A) {
            println!("hello world!");
        }
    }

    pub fn escape(&mut self) {
        if (self.key_code == winit::event::VirtualKeyCode::Escape) {
            println!("keycode: escape");
            self.app_events.borrow_mut().close();
        }
    }

    pub fn close(&mut self) {
        self.app_events.borrow_mut().close();
    }
}

fn main() {

    println!("Press '1' to switch to Wait mode.");
    println!("Press '2' to switch to WaitUntil mode.");
    println!("Press '3' to switch to Poll mode.");
    println!("Press 'R' to toggle request_redraw() calls.");
    println!("Press 'Esc' to close the window.");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Press 1, 2, 3 to change control flow mode. Press R to toggle redraw requests.")
        .build(&event_loop)
        .unwrap();

    let mut mode = Mode::Wait;
    let mut request_redraw = false;
    let mut wait_cancelled = false;
    let mut app = RefCell::new(AppEvents{close_requested: false});
    let mut player = Player{key_code: winit::event::VirtualKeyCode::Sleep, app_events: app};
    

    event_loop.run(move |event, _, control_flow| {
        use winit::event::{ElementState, StartCause, VirtualKeyCode};
        println!("{:?}", event);
        match event {
            Event::NewEvents(start_cause) => {
                wait_cancelled = match start_cause {
                    StartCause::WaitCancelled { .. } => mode == Mode::WaitUntil,
                    _ => false,
                }
            }
            // 画面イベントを集約している
            Event::WindowEvent { event, .. } => match event {
                // close処理
                WindowEvent::CloseRequested => {
                    player.close()
                }

                // kyeイベント処理
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {

                    println!("key_code: {:?}.", Some(virtual_code));
                    println!("type: {:?}", type_of(virtual_code));
                    player.key_code = virtual_code;
                    player.hello();
                    player.escape();
                },
                _ => (),
            },
            Event::MainEventsCleared => {
                if request_redraw && !wait_cancelled && !player.app_events.get_mut().close_requested {
                    window.request_redraw();
                }
                if player.app_events.get_mut().close_requested {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::RedrawRequested(_window_id) => {}
            Event::RedrawEventsCleared => {
                *control_flow = match mode {
                    Mode::Wait => ControlFlow::Wait,
                    Mode::WaitUntil => {
                        if wait_cancelled {
                            *control_flow
                        } else {
                            ControlFlow::WaitUntil(time::Instant::now() + WAIT_TIME)
                        }
                    }
                    Mode::Poll => {
                        thread::sleep(POLL_SLEEP_TIME);
                        ControlFlow::Poll
                    }
                };
            }
            _ => (),
        }
    });
}