use std::fs::File;
use std::sync::mpsc;
use std::io::BufReader;
use png::Decoder;
use tray_item::{TrayItem, IconSource};

enum Message {
    Quit,
    Red,
    Green
}

fn load_icon_source(file_path: &str) -> IconSource {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let decoder = Decoder::new(reader);
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    
    IconSource::Data {
        data: buf,
        height: info.height as i32,
        width: info.width as i32,
    }
}

fn main() {
    let icon_source = load_icon_source("assets/crab.png");

    let mut tray = TrayItem::new("My Tray App", icon_source).unwrap();

    tray.add_label("Tray Label").unwrap();

    let (tx, rx) = mpsc::sync_channel::<Message>(2);

    let green_tx = tx.clone();
    tray.add_menu_item("Set icon green", move || {
        green_tx.send(Message::Green).unwrap();
    })
    .unwrap();

    let red_tx = tx.clone();
    tray.add_menu_item("Set icon red", move || {
        red_tx.send(Message::Red).unwrap();
    })
    .unwrap();

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                println!("Quit");
                break
            },
            Ok(Message::Green) =>{
                println!("Green!");
                let icon_source: IconSource = load_icon_source("assets/tray_icon-green.png");
                tray.set_icon(icon_source).unwrap();
            },
            Ok(Message::Red) =>{
                println!("Red!");
                let icon_source: IconSource = load_icon_source("assets/tray_icon-red.png");
                tray.set_icon(icon_source).unwrap();
            },
            _ => {}
        }
    }
}