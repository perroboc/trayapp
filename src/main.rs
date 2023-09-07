use image::GenericImageView;
use std::path::Path;
use std::sync::mpsc;
use tray_item::IconSource;
use tray_item::TrayItem;

enum Message {
    Quit,
    Red,
    Green,
}

fn load_icon_source(file_path: &str) -> IconSource {
    let img = image::open(Path::new(&file_path)).unwrap();

    IconSource::Data {
        data: img.as_bytes().to_vec(),
        height: img.dimensions().1 as i32,
        width: img.dimensions().0 as i32,
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
                break;
            }
            Ok(Message::Green) => {
                println!("Green!");
                let icon_source: IconSource = load_icon_source("assets/tray_icon-green.png");
                tray.set_icon(icon_source).unwrap();
            }
            Ok(Message::Red) => {
                println!("Red!");
                let icon_source: IconSource = load_icon_source("assets/tray_icon-red.png");
                tray.set_icon(icon_source).unwrap();
            }
            _ => {}
        }
    }
}
