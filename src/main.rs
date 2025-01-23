#![no_std]
#![no_main]

use wut::prelude::*;
use wut::*;

// use wups::prelude::*;
use wups::*;

WUPS_PLUGIN_NAME!("Rust Plugin");
WUPS_PLUGIN_DESCRIPTION!("Example plugin");
WUPS_PLUGIN_VERSION!("v0.1");
WUPS_PLUGIN_AUTHOR!("29th-day");
WUPS_PLUGIN_LICENSE!("EUPL");

#[on_initialize(Udp)]
fn init() {
    wut::logger::init(wut::logger::Udp);
    println!("init");

    // println!("{:?}", storage::delete("abc"));
    // println!("{:?}", storage::load::<String>("abc"));
    // println!(
    //     "{:?}",
    //     storage::store::<String>("abc", "Hello there, Kenobi".to_string())
    // );
    // println!("{:?}", storage::load::<String>("abc"));

    // let ui = Root {
    //     name: PLUGIN_NAME.to_string(),
    //     items: vec![
    //         Label {
    //             text: "X".to_string(),
    //         },
    //         Toggle {
    //             text: "T".to_string(),
    //             value: true,
    //             trueValue: "on".to_string(),
    //             falseValue: "off".to_string(),
    //             changed: (),
    //         },
    //         Range {
    //             text: "R1".to_string(),
    //             value: 0,
    //             min: -5,
    //             max: 5,
    //             changed: (),
    //         },
    //         Range {
    //             text: "R2".to_string(),
    //             value: 0,
    //             min: 0,
    //             max: 10,
    //             changed: (),
    //         },
    //         Select {
    //             text: "S".to_string(),
    //             index: 0,
    //             options: vec![c"A", c"B", c"C"],
    //             changed: (),
    //         },
    //     ],
    // };
    // let _ = ui::MenuUI::new(ui).unwrap();
}

#[on_application_start(Udp)]
fn start() {
    println!("start");
}

#[on_deinitialize(Udp)]
fn deinit() {
    println!("deinit");
    wut::thread::sleep(wut::time::Duration::from_secs(3));
}
