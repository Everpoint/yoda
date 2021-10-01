pub mod control;
pub mod map;
pub mod layer;
pub mod render_target;
pub mod symbol;
pub mod event;

#[macro_use]
extern crate glium;

extern crate nalgebra as na;

pub type Point = [f32; 2];
pub type Color = [f32; 4];
pub type Point3 = [f32; 3];