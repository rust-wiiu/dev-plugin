use core::{cell::RefCell, fmt::Display};
use notifications;
use wups::config::glyphs::CafeGlyphs;
use wut::{
    alloc::{boxed::Box, rc::Rc},
    gamepad::GamepadState,
    prelude::*,
};

type Node = Rc<RefCell<Box<dyn MenuItem>>>;

pub trait MenuItem {
    fn render(&self) -> String;

    fn control(&mut self, input: GamepadState, stack: &mut Vec<Node>) -> bool;

    fn focus(&mut self) {}

    fn focusable(&self) -> bool {
        false
    }
}

// region: Menu

pub struct Menu {
    name: String,
    items: Vec<Node>,
    pos: usize,
    focused: bool,
}

impl Menu {
    pub fn new(name: &str, items: Vec<Node>) -> Node {
        Rc::new(RefCell::new(Box::new(Self {
            name: String::from(name),
            items,
            pos: 0,
            focused: false,
        })))
    }
}

impl MenuItem for Menu {
    fn focus(&mut self) {
        self.focused = true;
    }

    fn focusable(&self) -> bool {
        true
    }

    fn render(&self) -> String {
        if self.focused {
            format!(
                "{}\u{3000}{}\u{3000}{}",
                CafeGlyphs::BTN_LEFT,
                &self.items[self.pos].borrow().render(),
                CafeGlyphs::BTN_RIGHT
            )
        } else {
            format!("{} {}", self.name, CafeGlyphs::KBD_RETURN)
        }
    }

    fn control(&mut self, input: GamepadState, stack: &mut Vec<Node>) -> bool {
        use wut::gamepad::Button as B;
        let mut changed = false;

        let item = self.items[self.pos].clone();

        if item.borrow().focusable() && input.trigger.contains(B::A) {
            item.borrow_mut().focus();
            stack.push(item);
            changed = true;
        } else if input.trigger.contains(B::B) {
            if stack.len() > 1 {
                self.focused = false;
                stack.pop();
                changed = true;
            }
        } else if input.trigger.contains(B::Left) {
            self.pos = (self.pos + self.items.len() - 1) % self.items.len();
            changed = true;
        } else if input.trigger.contains(B::Right) {
            self.pos = (self.pos + 1) % self.items.len();
            changed = true;
        } else {
            changed = self.items[self.pos].borrow_mut().control(input, stack);
        }

        changed
    }
}

// endregion

// region: Button

pub struct Button {
    text: String,
    f: Box<dyn Fn() + Send>,
}

impl Button {
    pub fn new<F>(text: &str, f: F) -> Node
    where
        F: 'static + Fn() + Send,
    {
        Rc::new(RefCell::new(Box::new(Self {
            text: String::from(text),
            f: Box::new(f),
        })))
    }
}

impl MenuItem for Button {
    fn render(&self) -> String {
        format!("<{}>", self.text)
    }

    fn control(&mut self, input: GamepadState, _stack: &mut Vec<Node>) -> bool {
        use wut::gamepad::Button as B;
        if input.trigger.contains(B::A) {
            (self.f)();
        }
        false
    }
}

// endregion

// region: Number

pub struct Number<T: Display + core::ops::AddAssign + core::ops::SubAssign + PartialOrd + Clone> {
    text: String,
    value: T,
    inc: T,
    min: T,
    max: T,
    f: Box<dyn Fn(&T) + Send>,
}

impl<T: 'static + Display + core::ops::AddAssign + core::ops::SubAssign + PartialOrd + Clone>
    Number<T>
{
    pub fn new<F>(text: &str, value: T, inc: T, min: T, max: T, f: F) -> Node
    where
        F: 'static + Fn(&T) + Send,
    {
        Rc::new(RefCell::new(Box::new(Self {
            text: String::from(text),
            value,
            inc,
            min,
            max,
            f: Box::new(f),
        })))
    }
}

impl<T: Display + core::ops::AddAssign + core::ops::SubAssign + PartialOrd + Clone> MenuItem
    for Number<T>
{
    fn render(&self) -> String {
        format!(
            "{}: {} {}",
            self.text,
            self.value,
            CafeGlyphs::ARROW_UP_DOWN
        )
    }

    fn control(&mut self, input: GamepadState, _stack: &mut Vec<Node>) -> bool {
        use wut::gamepad::Button as B;
        let mut changed = false;
        if input.trigger.contains(B::Up) {
            if self.value < self.max {
                self.value += self.inc.clone();
            };
            changed = true;
        }

        if input.trigger.contains(B::Down) {
            if self.value > self.min {
                self.value -= self.inc.clone();
            };
            changed = true;
        }

        if input.trigger.contains(B::A) {
            (self.f)(&self.value);
        }

        changed
    }
}

// endregion

// region: Select

pub struct Select<T: Display> {
    text: String,
    options: Vec<T>,
    index: usize,
    f: Box<dyn Fn(&T) + Send>,
}

impl<T: 'static + Display> Select<T> {
    pub fn new<F>(text: &str, options: Vec<T>, f: F) -> Node
    where
        F: 'static + Fn(&T) + Send,
    {
        Rc::new(RefCell::new(Box::new(Self {
            text: String::from(text),
            options,
            index: 0,
            f: Box::new(f),
        })))
    }
}

impl<T: Display> MenuItem for Select<T> {
    fn render(&self) -> String {
        format!(
            "{}: {} {}",
            self.text,
            self.options[self.index],
            CafeGlyphs::ARROW_UP_DOWN
        )
    }

    fn control(&mut self, input: GamepadState, _stack: &mut Vec<Node>) -> bool {
        use wut::gamepad::Button as B;
        let mut changed = false;
        if input.trigger.contains(B::Up) {
            if self.index < self.options.len() - 1 {
                self.index += 1
            };
            changed = true;
        }

        if input.trigger.contains(B::Down) {
            if self.index > 0 {
                self.index -= 1;
            }
            changed = true;
        }

        if input.trigger.contains(B::A) {
            (self.f)(&self.options[self.index]);
        }

        changed
    }
}

// endregion

// region: Manager

pub struct OverlayNotification {
    hud: Option<notifications::Notification>,
    root: Node,
    stack: Vec<Node>,
}

impl OverlayNotification {
    pub fn new(root: Node) -> Self {
        let mut r = Self {
            hud: None,
            root,
            stack: vec![],
        };

        r.stack.push(r.root.clone());
        r.root.borrow_mut().focus();

        r
    }

    fn render(&self) {
        if let Some(hud) = &self.hud {
            let head = self.stack.last().unwrap().clone();
            let _ = hud.text(&head.borrow().render());
        }
    }

    pub fn run(&mut self, input: GamepadState) {
        use wut::gamepad::Button as B;
        if input.hold.contains(B::L | B::R) {
            if self.hud.is_none() {
                self.hud = Some(notifications::dynamic("").show().unwrap());
                self.render();
            }

            if self
                .stack
                .last()
                .unwrap()
                .clone()
                .borrow_mut()
                .control(input, &mut self.stack)
            {
                self.render();
            }
        } else {
            self.hud = None;
        }
    }
}

// endregion
