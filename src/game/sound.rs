use rodio::{
    OutputStream,
    Decoder
};
use rand::{
    thread_rng,
    Rng
};
use std::{
    thread,
    thread::{
        sleep
    },
    time::{
        Duration
    },
    collections::{
        HashMap
    },
    io::{
        Cursor,
        Read
    }
};

pub fn memeloop() {
    thread::spawn(|| {
        let (_stream, soundhandle) = OutputStream::try_default().unwrap();
        let bsink = rodio::Sink::try_new(&soundhandle).unwrap();
        bsink.set_volume(0.8);

        let mut sounds: HashMap<&str, &[u8]> = HashMap::new();
        sounds.insert("boom", include_bytes!("../../assets/sounds/boom.mp3"));
        sounds.insert("auuuugh", include_bytes!("../../assets/sounds/auuuugh.mp3"));

        let mut rng = thread_rng();

        loop {
            bsink.append(
                Decoder::new_mp3(Cursor::new(
                        *sounds.get("boom").unwrap()
                    )
                ).unwrap()
            );
            let asink = soundhandle.play_once(
                Cursor::new(
                    *sounds.get("auuuugh").unwrap()
                )
            ).unwrap();
            asink.set_volume(rng.gen_range(0.1..50.0));
            asink.set_speed(rng.gen_range(0.8..2.0));
            bsink.sleep_until_end();
        }
    });
}