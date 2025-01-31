#![no_std]
#![no_main]

extern crate notifications;

use core::any::Any;

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

// extern "C" {
//     #[link_section = ".data"]
//     static mut real_VPADRead: unsafe extern "C" fn(
//         channel: wut::bindings::VPADChan::Type,
//         buf: *mut wut::bindings::VPADStatus,
//         count: u32,
//         error: *mut wut::bindings::VPADReadError::Type,
//     ) -> i32;
// }

static mut INPUT: gamepad::GamepadState = gamepad::GamepadState::empty();

#[function_hook(library = VPAD, function = VPADRead)]
fn my_VPADRead(
    chan: wut::bindings::VPADChan::Type,
    buffers: *mut wut::bindings::VPADStatus,
    count: u32,
    error: *mut wut::bindings::VPADReadError::Type,
) -> i32 {
    let status = unsafe { hooked.unwrap()(chan, buffers, count, error) };

    unsafe { INPUT = gamepad::GamepadState::from(*buffers) };

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
                .name("Rust Thread")
                .spawn(my_thread)
                .unwrap(),
        );
    }
}

fn my_thread() {
    let _ = logger::init(logger::Udp);

    let hud = notifications::dynamic("").show().unwrap();

    struct Cursor {
        pos: usize,
        max: usize,
        changed: bool,
        icon: char,
    }

    impl Cursor {
        fn new(max: usize) -> Self {
            Self {
                pos: 0,
                max,
                changed: true,
                icon: config::glyphs::CafeGlyphs::ARROW_RIGHT,
            }
        }

        fn add(&mut self) {
            self.pos = (self.pos + 1) % self.max;
            self.changed = true;
        }

        fn sub(&mut self) {
            self.pos = (self.pos + self.max - 1) % self.max;
            self.changed = true;
        }

        fn has_changed(&mut self) -> bool {
            let x = self.changed;
            self.changed = false;
            x
        }
    }

    let items = vec!["A", "B", "C"];
    let mut cursor = Cursor::new(items.len());

    fn render_menu(pos: usize, options: &[&str], cursor: char) -> String {
        format!(
            "{}",
            options
                .iter()
                .enumerate()
                .map(|(i, &opt)| if i == pos {
                    format!("{cursor}{opt}")
                } else {
                    format!("\u{3000}{opt}")
                })
                .collect::<Vec<_>>()
                .join("    ") // Ensure consistent spacing
        )
    }

    use gamepad::Button as B;

    while thread::current().running() {
        // println!("thread: {}", time::DateTime::now());

        // let input = match gamepad.poll() {
        //     Ok(s) => s,
        //     Err(_) => gamepad::GamepadState::empty(),
        //     // Err(e) => panic!("{:?}", e),
        // };

        // // println!("{:?}", input.hold);

        // if input.hold.contains(B::L | B::R) && input.trigger.contains(B::Left) {
        //     cursor.sub();
        // }

        // if input.hold.contains(B::L | B::R) && input.trigger.contains(B::Right) {
        //     cursor.add();
        // }

        // if cursor.has_changed() {
        //     hud.text(&format!("{}", render_menu(cursor.pos, &items, cursor.icon)));
        // }

        unsafe {
            println!("{:?}", INPUT);
        }
        thread::sleep(time::Duration::from_secs(1));
        // unsafe {
        //     wut::bindings::GX2WaitForFlip();
        //     wut::bindings::GX2WaitForFlip();
        // }
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
