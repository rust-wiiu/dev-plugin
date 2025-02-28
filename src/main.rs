#![no_std]
#![no_main]

// when I put wups above wut the program wont compile wtf
// Order seems to matter (somehow):
// wups > wut -> doesn't compile (unless -Wl,--allow-multiple-definition but then it wont run)
// wut > wups -> compiles normally
use wut::prelude::*;
use wut::*;

use wups::prelude::*;
use wups::*;

use wut::font::icons;
// use wut::gamepad::Button;

// mod menu;
// mod overlay;
// mod test;

use overlay;

WUPS_PLUGIN_NAME!("Rust Plugin");

static HANDLE: sync::RwLock<Option<thread::JoinHandle>> = sync::RwLock::new(None);

struct MyMenu;
impl ConfigMenu for MyMenu {
    fn open(root: config::MenuRoot) -> Result<(), config::MenuError> {
        root.add(config::Label::new("Label 1"))?;

        let sub = config::Menu::new("Menu 1")?;
        sub.add(config::Label::new("Menu 1 Label"))?;
        sub.add(config::Label::new("Menu 2 Label"))?;
        root.add(sub)?;

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
            vec!["A", "B", "C"],
        ))?;

        Ok(())
    }
}

static mut INPUT: gamepad::GamepadState = gamepad::GamepadState::empty();

#[function_hook(module = VPAD, function = VPADRead)]
fn my_VPADRead(
    chan: wut::bindings::VPADChan::Type,
    buffers: *mut wut::bindings::VPADStatus,
    count: u32,
    error: *mut wut::bindings::VPADReadError::Type,
) -> i32 {
    let status = unsafe { hooked(chan, buffers, count, error) };

    use gamepad::Button as B;
    unsafe {
        INPUT = gamepad::GamepadState::from(*buffers);

        if INPUT.hold.contains(B::L | B::R) {
            (*buffers).hold = 0;
            (*buffers).trigger = 0;
        }
    }

    status
}

#[on_initialize(Udp)]
fn init() {
    println!("init");

    let _ = MyMenu::init("My Menu Rust Plugin");
}

#[on_application_start(Udp)]
fn start() {
    println!("start");

    //     notifications::info("info: abc").show().unwrap();
    //     notifications::error("error: abc")
    //         .callback(|| {
    //             logger::init(logger::Udp);
    //             println!("error callback");
    //             logger::deinit();
    //         })
    //         .show()
    //         .unwrap();

    let mut thread = HANDLE.write();
    if thread.is_none() {
        *thread = Some(
            thread::Builder::default()
                .name("Overlay")
                .attribute(thread::thread::ThreadAttribute::Cpu2)
                .priority(30)
                .spawn(my_thread)
                .unwrap(),
        );
    }
}

fn my_thread() {
    let _ = logger::init(logger::Udp);

    use overlay::*;

    let mut overlay = OverlayNotification::new(Menu::new(
        "Root",
        vec![
            Button::new("Search", || unsafe {
                println!("Start search");
                let start = 0x1000_0000;
                let to = start + 0x0100_0000;

                for (_i, x) in (start..=to).step_by(4).enumerate() {
                    let ptr = x as *const f32;
                    let _value = core::ptr::read_volatile(ptr);
                }

                println!("End search");
            }),
            Button::new("Speed", || unsafe {
                println!("--- speed ---");

                // let ptr = 0x10989c74 as *mut [u8; 4];
                // let value = core::ptr::read_volatile(ptr);
                // println!("{:?}", value);

                let ptr = 0x1096ef10 as *mut [u8; 4];
                let value = core::ptr::read_volatile(ptr);
                println!(
                    "{:?}, {}",
                    value,
                    core::mem::transmute::<[u8; 4], f32>(value)
                );

                let ptr = 0x1096ef48 as *mut [u8; 4];
                let value = core::ptr::read_volatile(ptr);
                println!(
                    "{:?}, {}",
                    value,
                    core::mem::transmute::<[u8; 4], f32>(value)
                );

                let ptr = 0x1096ef4c as *mut [u8; 4];
                let value = core::ptr::read_volatile(ptr);
                println!(
                    "{:?}, {}",
                    value,
                    core::mem::transmute::<[u8; 4], f32>(value)
                );

                let ptr = 0x1096ef50 as *mut [u8; 4];
                let value = core::ptr::read_volatile(ptr);
                println!(
                    "{:?}, {}",
                    value,
                    core::mem::transmute::<[u8; 4], f32>(value)
                );

                // let ptr = 0x48723ec4 as *mut [u8; 4];
                // let value = core::ptr::read_volatile(ptr);
                // println!("{:?}", value);

                // let ptr = 0x48723ec8 as *mut [u8; 4];
                // let value = core::ptr::read_volatile(ptr);
                // println!("{:?}", value);

                // let ptr = 0x48723ecc as *mut [u8; 4];
                // let value = core::ptr::read_volatile(ptr);
                // println!("{:?}", value);

                println!("--- speed ---");
            }),
            Menu::new(
                "Health",
                vec![
                    Number::new("Health", 1, 1, 0, 80, |v| unsafe {
                        core::ptr::write_volatile(0x1506b503 as *mut u8, *v);
                        wut::bindings::DCFlushRange(0x1506b503 as *mut core::ffi::c_void, 1);
                    }),
                    Number::new("Containers", 1, 1, 1, 20, |v| unsafe {
                        core::ptr::write_volatile(0x1506b501 as *mut u8, *v * 4);
                        wut::bindings::DCFlushRange(0x1506b501 as *mut core::ffi::c_void, 1);
                    }),
                ],
            ),
            Menu::new(
                "Items",
                vec![Toggle::new("Grappling Hook", false, |v| unsafe {
                    let v = if v { 0x25 } else { 0 };
                    core::ptr::write_volatile(0x1506b53f as *mut u8, v * 4);
                    wut::bindings::DCFlushRange(0x1506b53f as *mut core::ffi::c_void, 1);
                })],
            ),
            Menu::new(
                "Spoofs",
                vec![
                    Select::new(
                        &format!("{}", icons::BTN_X),
                        vec![("Grappling Hook", 0x25), ("Hookshot", 0x2f)],
                        |_, v| unsafe {
                            let address = 0x10976e6b;

                            core::ptr::write_volatile(address as *mut u8, v.value);
                            wut::bindings::DCFlushRange(address as *mut core::ffi::c_void, 1);
                        },
                    ),
                    Select::new(
                        &format!("{}", icons::BTN_Y),
                        vec![("Grappling Hook", 0x25), ("Hookshot", 0x2f)],
                        |_, v| unsafe {
                            let address = 0x10976e6c;

                            core::ptr::write_volatile(address as *mut u8, v.value);
                            wut::bindings::DCFlushRange(address as *mut core::ffi::c_void, 1);
                        },
                    ),
                    Select::new(
                        &format!("{}", icons::BTN_R),
                        vec![("Grappling Hook", 0x25), ("Hookshot", 0x2f)],
                        |_, v| unsafe {
                            let address = 0x10976e6d;

                            core::ptr::write_volatile(address as *mut u8, v.value);
                            wut::bindings::DCFlushRange(address as *mut core::ffi::c_void, 1);
                        },
                    ),
                ],
            ),
            Menu::new(
                "Stage",
                vec![
                    Button::new("Current", || unsafe {
                        let stage = 0x109763f0 as *mut [u8; 8];
                        println!("stage: {:02x?}", core::ptr::read(stage));

                        let spawn = 0x109763f9 as *mut u8;
                        println!("spawn: {:02x?}", core::ptr::read(spawn));

                        let room = 0x109763fa as *mut u8;
                        println!("room: {:02x?}", core::ptr::read(room));

                        let layer = 0x109763fb as *mut u8;
                        println!("layer: {:02x?}", core::ptr::read(layer));
                    }),
                    Select::new(
                        "Great Sea",
                        vec![
                            "Forsaken Fortress",
                            "Star Island",
                            "N. Fairy Island",
                            "Gale Island",
                            "Crescent Moon Island",
                            "Seven-Star Isles",
                            "Overlook Island",
                            "Four-Eye Reef",
                            "Mother & Child Isle",
                            "Spectacle Island",
                            "Windfall",
                            "Pawprint Isle",
                            "Dragon Roost Mt",
                            "Flight Control Platform",
                        ],
                        |i, _| unsafe {
                            // map
                            let stage = 0x109763f0 as *mut [u8; 8];
                            core::ptr::write(stage, *b"sea\0\0\0\0\0");

                            // spawn ID
                            let spawn = 0x109763f9 as *mut u8;
                            core::ptr::write(spawn, 0);

                            // room ID
                            let room = 0x109763fa as *mut u8;
                            core::ptr::write(room, i as u8 + 1);

                            // layer ID
                            let layer = 0x109763fb as *mut u8;
                            core::ptr::write(layer, 0xff);

                            // responsible for reload?
                            let ptr = 0x109763fc as *mut u8;
                            core::ptr::write(ptr, 0x01);
                        },
                    ),
                ],
            ),
        ],
    ));

    let mut input = unsafe { INPUT };

    while thread::current().running() {
        // println!("thread: {}", time::DateTime::now());

        if input != unsafe { INPUT } {
            input = unsafe { INPUT };

            overlay.run(input);
        }

        unsafe {
            wut::bindings::GX2WaitForFlip();
        }
    }

    logger::deinit();
}

#[on_application_exit(Udp)]
fn stop() {
    //     // println!("stop");

    let mut h = HANDLE.write();
    if let Some(handle) = h.take() {
        handle.thread().cancel();
        println!("{:?}", handle.join());
    }
}

// #[on_deinitialize]
// fn deinit() {
//     // println!("deinit");
// }
