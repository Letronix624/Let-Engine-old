mod client;

use std::{collections::HashMap, ops::Index};

use super::{delta_time, fps, BACKGROUND, SQUARE};

use client::{get_ping, Client};

#[derive(Clone, Copy)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub smaller: bool,
    pub bigger: bool,
    pub mouse: (f32, f32),
}
impl InputState {
    pub fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            smaller: false,
            bigger: false,
            mouse: (0.0, 0.0),
        }
    }
    pub fn get_xy(&self) -> (f32, f32) {
        let x = (self.right as i32 - self.left as i32) as f32;
        let y = (self.up as i32 - self.down as i32) as f32;
        let sx = x.abs() * 4.0 - x.abs() * y.abs() * 4.0 / 2.0;
        let sy = y.abs() * 4.0 - y.abs() * x.abs() * 4.0 / 2.0;

        (x * (sx.sqrt() / 2.0), -y * (sy.sqrt() / 2.0))
    }
}

#[derive(Clone, Debug)]
pub struct Object {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub data: Vec<super::Vertex>,
}
impl Object {
    pub fn empty() -> Self {
        Self {
            position: [0.0, 0.0],
            size: [0.0, 0.0],
            data: vec![],
        }
    }
}
#[derive(Clone)]
pub struct Game {
    pub objects: HashMap<String, Object>,
    pub renderorder: Vec<String>, //variable that has the order of object render
    pub input: InputState,
    client: Client,
    olddata: Object,
}

impl Game {
    pub fn init() -> Self {
        Self {
            objects: HashMap::new(),
            renderorder: vec![],
            input: InputState::new(),
            client: Client::new(),
            olddata: Object::empty(),
        }
    }
    fn getobject(&self, name: String) -> Object {
        return self.objects[&name].clone();
    }
    fn setobject(&mut self, name: String, object: Object) {
        self.objects.insert(name, object);
    }
    fn deleteobject(&mut self, name: String) {
        self.objects.remove(&name);
        let index = self.renderorder.iter().position(|x| *x == name).unwrap();
        self.renderorder.remove(index);
    }
    fn newobject(
        &mut self,
        name: String,
        data: Vec<super::Vertex>,
        mut position: [f32; 2],
        mut size: [f32; 2],
    ) {
        self.objects.insert(
            name.clone(),
            Object {
                position: position,
                size: size,
                data: data,
            },
        );
        self.renderorder.push(name);
    }
    pub fn start(&mut self) {
        //Runs one time before the first Frame.
        self.newobject("background".to_string(), BACKGROUND.to_vec(), [0.0, 0.0], [1.0, 1.0]);
        self.newobject(
            "player1".to_string(),
            SQUARE.to_vec(),
            [0.0, 0.0],
            [0.5, 0.5],
        );
        self.client.connect();

        println!("{:?}", self.renderorder);
    }
    pub fn main(&mut self) {
        //Runs every single frame once.
        println!("FPS:{} Ping:{}", fps(), get_ping());
        let mut player = self.getobject("player1".to_string());
        player.position = [
            player.position[0] + delta_time() as f32 * self.input.get_xy().0 * player.size[0] * 8.0,
            player.position[1] + delta_time() as f32 * self.input.get_xy().1 * player.size[1] * 8.0,
        ];
        player.size = [
            player.size[0]
                + player.size[0]
                    * delta_time() as f32
                    * (self.input.bigger as i32 - self.input.smaller as i32) as f32,
            player.size[1]
                + player.size[1]
                    * delta_time() as f32
                    * (self.input.bigger as i32 - self.input.smaller as i32) as f32,
        ];
        self.setobject("player1".to_string(), player);
    }

    pub fn tick(&mut self) {
        //Runs 62.4 times per second.
        if self.client.connected {
            let player = self.getobject("player1".to_string());
            if self.olddata.position != player.position || self.olddata.size != player.size {
                match self.client.sendobject(player) {
                    _ => (),
                };
                self.olddata = self.getobject("player1".to_string());
            }
            {
                let objects = client::GAMEOBJECTS.lock().unwrap();
                for object in objects.iter(){
                    if self.objects.contains_key(object.0){
                        self.setobject(object.0.clone(), object.1.clone());
                    }
                    else {
                        self.newobject(object.0.clone(), object.1.clone().data, object.1.position, object.1.position)
                    }
                }
            }

        }
    }

}
