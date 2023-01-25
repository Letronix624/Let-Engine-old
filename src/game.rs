mod client;
mod discord;
mod sound;

use std::collections::HashMap;

#[allow(unused_imports)]
use super::{delta_time, fps, window, BACKGROUND, BACKGROUND_ID, SQUARE};
use crate::data::SQUARE_ID;

#[allow(unused_imports)]
use client::{get_ping, Client};

#[derive(Clone, Copy)]
pub struct InputState {
    pub w: bool,
    pub s: bool,
    pub a: bool,
    pub d: bool,
    pub q: bool,
    pub e: bool,
    pub r: bool,
    pub mouse: (f32, f32),
    pub lmb: bool,
    pub mmb: bool,
    pub rmb: bool,
    pub vsd: f32,
}
impl InputState {
    pub fn new() -> Self {
        Self {
            w: false,
            s: false,
            a: false,
            d: false,
            q: false,
            e: false,
            r: false,
            mouse: (0.0, 0.0),
            lmb: false,
            mmb: false,
            rmb: false,
            vsd: 0.0,
        }
    }
    pub fn get_xy(&self) -> (f32, f32) {
        let x = (self.d as i32 - self.a as i32) as f32;
        let y = (self.w as i32 - self.s as i32) as f32;
        let sx = x.abs() * 4.0 - x.abs() * y.abs() * 4.0 / 2.0;
        let sy = y.abs() * 4.0 - y.abs() * x.abs() * 4.0 / 2.0;

        (x * (sx.sqrt() / 2.0), -y * (sy.sqrt() / 2.0))
    }
}

#[derive(Clone, Debug)]
pub struct Object {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub rotation: f32,
    pub color: [f32; 4],
    //pub texture: Option<>
    pub data: Vec<super::Vertex>,
    pub indices: Vec<u16>,
}
impl Object {
    pub fn empty() -> Self {
        Self {
            position: [0.0, 0.0],
            size: [0.0, 0.0],
            rotation: 0.0,
            color: [0.0, 0.0, 0.0, 0.0],
            data: vec![],
            indices: vec![],
        }
    }
}
#[derive(Clone)]
#[allow(dead_code)]
pub struct Game {
    pub objects: HashMap<String, Object>,
    pub renderorder: Vec<String>, //variable that has the order of object render
    pub input: InputState,
    client: Client,
    olddata: Object,
    direction: [i8; 2]
}

impl Game {
    pub fn init() -> Self {
        Self {
            objects: HashMap::new(),
            renderorder: vec![],
            input: InputState::new(),
            client: Client::new(),
            olddata: Object::empty(),
            direction: [1, 1],
        }
    }
    pub fn getobject(&self, name: String) -> Object {
        return self.objects[&name].clone();
    }
    pub fn setobject(&mut self, name: String, object: Object) {
        self.objects.insert(name, object);
    }
    #[allow(unused)]
    fn deleteobject(&mut self, name: String) {
        self.objects.remove(&name);
        let index = self.renderorder.iter().position(|x| *x == name).unwrap();
        self.renderorder.remove(index);
    }
    fn newobject(
        &mut self,
        name: String,
        color: [f32; 4],
        data: Vec<super::Vertex>,
        indices: Vec<u16>,
        position: [f32; 2],
        size: [f32; 2],
        rotation: f32,
    ) {
        self.objects.insert(
            name.clone(),
            Object {
                position,
                size,
                rotation,
                color,
                data,
                indices,
            },
        );
        self.renderorder.push(name);
    }
    pub fn start(&mut self) {
        //Runs one time before the first Frame.
        // self.newobject(
        //     "background".to_string(),
        //     [0.1, 0.3, 0.9, 1.0],
        //     BACKGROUND.to_vec(),
        //     BACKGROUND_ID.to_vec(),
        //     [0.0, 0.0],
        //     [1.0, 1.0],
        //     0.0,
        // );
        self.newobject(
            "player1".to_string(),
            [0.1, 0.0, 0.0, 1.0],
            SQUARE.into(),
            SQUARE_ID.into(),
            [0.0, 0.0],
            [0.15, 0.15],
            0.0,
        );

        //let _ = self.client.connect(); //Connects to the server (seflon.ddns.net) if its available

        println!("{:?}", self.renderorder);

        discord::start();

        //sound::memeloop();
    }
    pub fn main(&mut self) {
        //Runs every single frame once.


        
    }

    pub fn late_main(&mut self) {
        //Runs every time after the redraw events are done.
    }

    pub fn tick(&mut self) {
        //Runs 62.4 times per second.

        // println!("FPS:{} Ping:{}", fps(), get_ping());
        let mut player = self.getobject("player1".to_string());

        player.position = [
            player.position[0] + self.direction[0] as f32 / 100.0,
            player.position[1] + self.direction[1] as f32 / 100.0,
        ];
        

        //vec2 resolutionscaler = vec2(pc.resolution.y / (pc.resolution.x + pc.resolution.y), pc.resolution.x / (pc.resolution.x + pc.resolution.y));
        //vec2 resolutionscaler = vec2(sin(atan(pc.resolution.y, pc.resolution.x)), cos(atan(pc.resolution.y, pc.resolution.x)))  / (sqrt(2) / 2);
        // player.position = [
        //     (window().width as f32 / 1000 as f32),
        //     (window().height as f32 / 1000 as f32)
        // ];

        // println!("{}", (window().width as f32 / window().height as f32));
        
        if player.position[0] > (window().width as f32 / 1000 as f32) - player.size[0] {
            self.direction[0] = -1;
        }
        else if player.position[0] < -(window().width as f32 / 1000 as f32) + player.size[0] {
            self.direction[0] = 1;
        }
        if player.position[1] > (window().height as f32 / 1000 as f32) - player.size[1] {
            self.direction[1] = -1;
        }
        else if player.position[1] < -(window().height as f32 / 1000 as f32) + player.size[1] {
            self.direction[1] = 1;
        }

        
        if self.input.r {
            player.position = [0.0, 0.0];
            player.rotation = 0.0;
            self.direction = [0, 0];
        }
        // if player.data.len() <= 9 && self.input.vsd == -1.0 {
        //     self.input.vsd = 0.0;
        // }
        // player.data = data::make_circle(
        //     ((player.data.len() / 3) as isize + self.input.vsd as isize) as usize,
        // );

        self.setobject("player1".to_string(), player);





        // if self.client.connected {
        //     //Client data sender
        //     let player = self.getobject("player1".to_string());
        //     if self.olddata.position != player.position || self.olddata.size != player.size {
        //         match self.client.sendobject(player) {
        //             _ => (),
        //         };
        //         self.olddata = self.getobject("player1".to_string());
        //     }
        //     {
        //         let objects = client::GAMEOBJECTS.lock().unwrap();
        //         for object in objects.iter() {
        //             if self.objects.contains_key(object.0) {
        //                 self.setobject(object.0.clone(), object.1.clone());
        //             } else {
        //                 self.newobject(
        //                     object.0.clone(),
        //                     object.1.clone().data,
        //                     object.1.position,
        //                     object.1.position,
        //                     0.0,
        //                 )
        //             }
        //         }
        //     }
        // }
    }
}
