//#![windows_subsystem = "windows"]
extern crate image;
extern crate vulkano;
use data::*;
use game::{Game, Object};
use server::Server;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::*;
use std::net::TcpListener;
use winit::event::{ElementState, VirtualKeyCode};
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::Window
};

mod data;
mod game;
mod vulkan;
mod server;

lazy_static::lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::init());
}

static mut FPS: u16 = 0;
static mut DELTA_TIME: f64 = 0.0;

#[allow(dead_code)]
fn delta_time() -> f64 {
    unsafe {
        return DELTA_TIME;
    }
}
#[allow(dead_code)]
fn fps() -> u16 {
    unsafe {
        return FPS;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let server_mode = args.contains(&"--server".to_string());
    if server_mode {
        match server() {
            Ok(_) => (),
            Err(e) => {
                println!("Server closed! Reason: {e}")
            }
        };
    } else {
        client();
    }
}

fn server() -> std::io::Result<()>{
    //start
    let socket = Arc::new(Mutex::new(match Server::init() {
        Ok(t) => t,
        Err(e) => {
            panic!("Couldn't start the server: {}", e);
        }
    }));

    socket.clone().lock().unwrap().start()?;


    //tick (62.4/s)
    let serv = socket.clone();
    thread::spawn(move || {
        loop{
            serv.lock().unwrap().broadcastobjs().unwrap();
            sleep(Duration::from_nanos(16025641));
        }
        
    });
    //main
    let listener: TcpListener;
    {
        let soc = socket.clone();
        let sock = soc.lock().unwrap();
        listener = TcpListener::bind(format!("{}:{}", &sock.ip, &sock.port))?;
    }

    for stream in listener.incoming() {
        let conn = stream?.try_clone()?;
        let addr = conn.try_clone()?;
        let addr = addr.peer_addr()?;
        thread::spawn(move || Server::tcpconnection(conn, addr));
    }
    Ok(())
}

fn client(){
    // let game = App::initialize();
    // GAME.lock().unwrap()mainloop();
    GAME.lock().unwrap().start();
    thread::spawn(|| {
        loop {
            {
                let mut game = GAME.lock().unwrap();
                game.tick();
            }
            //thread::sleep(Duration::from_secs(1));
            thread::sleep(Duration::from_nanos(16025641));
        }
    });
    let (mut app, event_loop) = vulkan::App::initialize();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            //Event::
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                app.recreate_swapchain = true;
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                if let Some(key_code) = input.virtual_keycode {
                    match key_code {
                        VirtualKeyCode::F11 => {
                            if input.state == ElementState::Released {
                                vulkan::App::fullscreen(app.surface.clone());
                            }
                        }
                        VirtualKeyCode::W => {
                            GAME.lock().unwrap().input.up =
                                input.state == ElementState::Pressed;
                        }
                        VirtualKeyCode::A => {
                            GAME.lock().unwrap().input.left =
                                input.state == ElementState::Pressed;
                        }
                        VirtualKeyCode::S => {
                            GAME.lock().unwrap().input.down =
                                input.state == ElementState::Pressed;
                        }
                        VirtualKeyCode::D => {
                            GAME.lock().unwrap().input.right =
                                input.state == ElementState::Pressed;
                        }
                        VirtualKeyCode::Q => {
                            GAME.lock().unwrap().input.smaller =
                                input.state == ElementState::Pressed;
                        }
                        VirtualKeyCode::E => {
                            GAME.lock().unwrap().input.bigger =
                                input.state == ElementState::Pressed;
                        }
                        VirtualKeyCode::G => if input.state == ElementState::Pressed {},

                        _ => (),
                    }
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                let dim = app.surface.object().unwrap().downcast_ref::<Window>().unwrap().inner_size();
                GAME.lock().unwrap().input.mouse = (
                    (position.x as f32 / dim.width as f32) * 2.0 - 1.0,
                    (position.y as f32 / dim.height as f32) * 2.0 - 1.0,
                )
            }
            Event::MainEventsCleared => {
                //game stuff
                unsafe {
                    DELTA_TIME = unix_timestamp() - app.dt1;
                    FPS = (1.0 / DELTA_TIME) as u16;
                }
                app.vertices = vec![];
                let objects: HashMap<String, Object>;
                objects = GAME.lock().unwrap().objects.clone();

                for object in GAME.lock().unwrap().renderorder.iter() {
                    let obj = objects.get(object).unwrap();
                    for vertex in obj.data.iter() {
                        app.vertices.push(Vertex {
                            position: [
                                vertex.position[0] * obj.size[0] + obj.position[0],
                                vertex.position[1] * obj.size[1] + obj.position[1],
                            ],
                        });
                    }
                }
                GAME.lock().unwrap().main();
            }
            Event::RedrawEventsCleared => {
                app.redrawevent();
            }
            _ => (),
        }
    });
}
fn unix_timestamp() -> f64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
}
