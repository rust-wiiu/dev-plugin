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

WUPS_PLUGIN_NAME!("Rust Plugin");

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

static mut INPUT: gamepad::GamepadState = gamepad::GamepadState::new();

#[function_hook(module = VPAD, function = VPADRead)]
fn my_VPADRead(
    chan: wut::bindings::VPADChan::Type,
    buffers: *mut wut::bindings::VPADStatus,
    count: u32,
    error: *mut wut::bindings::VPADReadError::Type,
) -> i32 {
    let status = unsafe { hooked(chan, buffers, count, error) };

    unsafe {
        INPUT = gamepad::GamepadState::from(*buffers);

        use gamepad::Button as B;

        if INPUT.trigger.contains(B::Y) {
            println!("Y");
            // foreground::browser::browser(None);
            // foreground::browser(Some("https://www.google.com/"));
            foreground::e_shop();
        }
    }

    status
}

#[on_initialize(Udp)]
fn init() {
    println!("init");

    // dynamic_loading::RplCallback::new(|| {
    //     println!("rpl loaded");
    // });

    let _ = MyMenu::init("My Menu Rust Plugin");

    // let s = m.data::<*const u32>("MyData").unwrap();

    // let r = *s;
}

#[on_application_start]
fn start() {
    let _ = logger::udp();

    println!("start");

    use dynamic_loading::Module;

    let m = Module::new("coreinit.rpl").unwrap();

    let s = m
        .function::<unsafe extern "C" fn() -> u64>("OSGetTitleID")
        .unwrap();

    println!("symbol: {:?}", unsafe { s.into_raw() });
}

#[on_application_exit]
fn stop() {
    logger::deinit();
}

// #[on_deinitialize]
// fn deinit() {
//     // println!("deinit");
// }
