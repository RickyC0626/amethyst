//! A collection of abstractions for various input devices to be used with Amethyst.

#![doc(
    html_logo_url = "https://amethyst.rs/brand/logo-standard.svg",
    html_root_url = "https://docs.amethyst.rs/stable"
)]
#![deny(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    rust_2018_compatibility
)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::new_without_default,
    clippy::module_name_repetitions,
    clippy::pub_enum_variant_names
)]

use std::iter::Iterator;

pub use winit::event::{ElementState, VirtualKeyCode};

#[cfg(feature = "sdl_controller")]
pub use self::sdl_events_system::SdlEventsSystem;
pub use self::{
    axis::Axis,
    bindings::{BindingError, Bindings},
    bundle::{BindingsFileError, InputBundle},
    button::Button,
    controller::{ControllerAxis, ControllerButton, ControllerEvent},
    event::InputEvent,
    input_handler::{InputHandler, KeyboardModifiersState},
    mouse::MouseAxis,
    scroll_direction::ScrollDirection,
    system::InputSystem,
    util::{
        get_action_simple, get_input_axis_simple, get_key, get_mouse_button, is_close_requested,
        is_key_down, is_key_up, is_mouse_button_down,
    },
};

mod axis;
mod bindings;
mod bundle;
mod button;
mod controller;
mod event;
mod input_handler;
mod mouse;
mod scroll_direction;
mod system;
mod util;

#[cfg(feature = "sdl_controller")]
mod sdl_events_system;
