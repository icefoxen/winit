#![cfg(feature = "serde")]

extern crate serde;
extern crate winit;

use serde::{Deserialize, Serialize};
use winit::dpi::{PhysicalPosition, PhysicalPosition, PhysicalSize, PhysicalSize};
use winit::{ControlFlow, MouseCursor};
use winit::{
    ElementState, KeyboardInput, ModifiersState, MouseButton, MouseScrollDelta, TouchPhase,
    VirtualKeyCode,
};

fn needs_serde<S: Serialize + Deserialize<'static>>() {}

#[test]
fn root_serde() {
    needs_serde::<ControlFlow>();
    needs_serde::<MouseCursor>();
}

#[test]
fn events_serde() {
    needs_serde::<KeyboardInput>();
    needs_serde::<TouchPhase>();
    needs_serde::<ElementState>();
    needs_serde::<MouseButton>();
    needs_serde::<MouseScrollDelta>();
    needs_serde::<VirtualKeyCode>();
    needs_serde::<ModifiersState>();
}

#[test]
fn dpi_serde() {
    needs_serde::<PhysicalPosition>();
    needs_serde::<PhysicalPosition>();
    needs_serde::<PhysicalSize>();
    needs_serde::<PhysicalSize>();
}
