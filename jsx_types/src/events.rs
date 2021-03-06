pub type EventHandler<'a, T> = 'a + FnMut(&T) -> ();
pub use web_sys::{
  Element,
  Document,
  Window,
  MouseEvent,
  KeyboardEvent,
  InputEvent,
  HtmlElement,
  EventTarget,
  Event,
  Node
};

pub type MouseEventHandler<'a> = EventHandler<'a, MouseEvent>;

pub type KeyboardEventHandler<'a> = EventHandler<'a, KeyboardEvent>;

pub type InputEventHandler<'a> = EventHandler<'a, InputEvent>;

pub struct EventHandlers<'a> {
  pub on_click: Option<Box<MouseEventHandler<'a>>>,
  pub on_mouse_over: Option<Box<MouseEventHandler<'a>>>,
  pub on_mouse_out: Option<Box<MouseEventHandler<'a>>>,
  pub on_input: Option<Box<InputEventHandler<'a>>>,
  pub on_keydown: Option<Box<KeyboardEventHandler<'a>>>,
}

impl<'a> EventHandlers<'a> {
  pub fn new() -> EventHandlers<'a> {
    EventHandlers {
      on_click: None,
      on_mouse_over: None,
      on_mouse_out: None,
      on_input: None,
      on_keydown: None,
    }
  }
}
