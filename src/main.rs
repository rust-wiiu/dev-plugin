#![no_std]
#![no_main]

use alloc::ffi::CString;
// when I put wups above wut the program wont compile wtf
// Order seems to matter (somehow):
// wups > wut -> doesnt compile (unless -Wl,--allow-multiple-definition but then it wont run)
// wut > wups -> compiles normally
use wut::prelude::*;
use wut::*;

use wups::prelude::*;
use wups::*;

WUPS_PLUGIN_NAME!("Rust Plugin");

static HANDLE: sync::RwLock<Option<thread::JoinHandle>> = sync::RwLock::new(None);

struct MyMenu;
impl ConfigMenu for MyMenu {
    fn open(root: config::MenuRoot) -> Result<(), config::MenuError> {
        // todo!()

        root.add(config::Label::new("Label 1"))?;

        let sub = config::Menu::new("Menu 1")?;
        sub.add(config::Label::new("Menu 1 Label"))?;
        sub.add(config::Label::new("Menu 2 Label"))?;
        root.add(sub);

        root.add(config::Label::new("Label 2"))?;

        root.add(config::Toggle::new(
            "Toggle",
            "my_super_toggle",
            true,
            "On",
            "Off",
        ))?;

        root.add(config::Toggle::new(
            "Toggle 2",
            "my_super_toggle_2",
            true,
            "On",
            "Off",
        ))?;

        root.add(config::Range::new("Range", "my_insane_range", 0, -5, 5))?;

        root.add(config::Select::new(
            "Select",
            "my_awesome_select",
            0,
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
        ))?;

        Ok(())
    }
}

#[on_initialize(Udp)]
fn init() {
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

    let _ = MyMenu::init("My Menu Rust Plugin");
}

#[on_application_start(Udp)]
fn start() {
    println!("start");

    // let mut h = HANDLE.write();
    // if h.is_none() {
    //     *h = Some(
    //         thread::Builder::default()
    //             .name("My Custom Thread")
    //             .spawn(my_thread)
    //             .unwrap(),
    //     );
    // }
}

fn my_thread() {
    let _ = logger::init(logger::Udp);

    while thread::current().running() {
        println!("thread: {}", time::DateTime::now());
        thread::sleep(time::Duration::from_secs(1));
    }

    logger::deinit();
}

#[on_application_exit]
fn stop() {
    // println!("stop");

    // let mut h = HANDLE.write();
    // if let Some(handle) = h.take() {
    //     handle.thread().cancel();
    //     println!("{:?}", handle.join());
    // }
}

#[on_deinitialize]
fn deinit() {
    // println!("deinit");
}
