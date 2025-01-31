#![no_std]
#![no_main]

extern crate notifications;

// when I put wups above wut the program wont compile wtf
// Order seems to matter (somehow):
// wups > wut -> doesnt compile (unless -Wl,--allow-multiple-definition but then it wont run)
// wut > wups -> compiles normally
use wut::prelude::*;
use wut::*;

use wups::prelude::*;
use wups::*;

mod menu;

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
    let status = unsafe { hooked.unwrap()(chan, buffers, count, error) };

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
                .name("Rust Thread")
                .priority(30)
                .spawn(my_thread)
                .unwrap(),
        );
    }
}

fn my_thread() {
    let _ = logger::init(logger::Udp);

    let hud = notifications::dynamic("").show().unwrap();

    menu::NotificationMenu {
        hud: notifications::dynamic("").show().unwrap(),
        items: vec![
            menu::MenuItem::new("A", || println!("A")),
            menu::MenuItem::new("B", || println!("B")),
        ],
        pos: 0,
        icon: wups::config::glyphs::CafeGlyphs::ARROW_RIGHT,
    };

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

    let mut input = unsafe { INPUT };

    while thread::current().running() {
        // println!("thread: {}", time::DateTime::now());

        if input != unsafe { INPUT } {
            input = unsafe { INPUT };

            if input.hold.contains(B::L | B::R) && input.trigger.contains(B::Left) {
                println!("{} - {:?}", time::DateTime::now(), input);
                cursor.sub();
            }

            if input.hold.contains(B::L | B::R) && input.trigger.contains(B::Right) {
                println!("{} - {:?}", time::DateTime::now(), input);
                cursor.add();
            }

            if input.hold.contains(B::L | B::R) && input.trigger.contains(B::A) {
                println!("{:?}", &items[cursor.pos]);
            }
        }

        if cursor.has_changed() {
            let _ = hud.text(&format!("{}", render_menu(cursor.pos, &items, cursor.icon)));
            println!("update");
        }

        // unsafe {
        //     println!("{:?}", INPUT);
        // }
        // thread::sleep(time::Duration::from_secs(1));
        unsafe {
            wut::bindings::GX2WaitForFlip();
            // wut::bindings::GX2WaitForFlip();
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
