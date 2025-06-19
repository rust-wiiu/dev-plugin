#![no_std]
#![no_main]

// when I put wups above wut the program wont compile wtf
// Order seems to matter (somehow):
// wups > wut -> doesn't compile (unless -Wl,--allow-multiple-definition but then it wont run)
// wut > wups -> compiles normally
use wups::prelude::*;
use wups::*;
use wut::prelude::*;
use wut::*;

use wupf::{
    hook_on_input, hook_on_update, hook_plugin, Handler, OnInput, OnUpdate, Plugin, StaticHandler,
};

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

struct MyPlugin {
    a: u32,
}

impl StaticHandler for MyPlugin {
    fn handler() -> &'static Handler<Self> {
        static HANDLER: Handler<MyPlugin> = Handler::new();
        &HANDLER
    }
}

hook_plugin!(MyPlugin);
impl Plugin for MyPlugin {
    fn on_init() -> Self {
        let _ = MyMenu::init(PLUGIN_NAME).unwrap();

        Self { a: 0 }
    }

    fn on_deinit(&mut self) {}

    fn on_start(&mut self) {
        let _ = logger::udp();

        self.a += 1;
        println!("start: {}", self.a);
    }

    fn on_exit(&mut self) {
        self.a += 1;
        println!("end: {}", self.a);

        logger::deinit();
    }
}

hook_on_input!(MyPlugin);
impl OnInput for MyPlugin {
    fn on_input(&mut self, port: gamepad::Port, state: gamepad::State) -> Option<gamepad::State> {
        if !state.hold.is_empty() {
            println!("port: {:?}, hold: {:?}", port, state.hold);
        }
        None
    }
}

hook_on_update!(MyPlugin);
impl OnUpdate for MyPlugin {
    fn on_update(&mut self) {
        // println!("Update");
    }
}
