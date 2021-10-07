use crate::layer::Layer;
use crate::control::{ControlState, MapControl, MapControlSettings};
use crate::render_target::RenderTarget;
use crate::event::{MapEvent, ClickEvent, HandlerStore, EventState, TypedHandlerStore, EventListener};
use crate::Point;
use std::rc::{Weak, Rc};
use std::cell::RefCell;

pub struct Map {
    layers: Vec<Rc<RefCell<dyn Layer>>>,
    position: MapPosition,
    animation: Option<MapAnimation>,
    control_state: ControlState,
    handler_store: HandlerStore,
}

impl Map {
    pub fn new() -> Self {
        Self {
            layers: vec![],
            position: MapPosition::default(),
            animation: None,
            control_state: ControlState::default(),
            handler_store: HandlerStore::default(),
        }
    }

    pub fn draw(&mut self, target: &mut RenderTarget) {
        let (x, y) = target.get_dimensions();
        self.position.set_screen_size(x, y);

        for layer in &mut self.layers {
            layer.borrow_mut().draw(target, &self.position);
        }
    }

    pub fn animate_to(&mut self, position: MapPosition, duration: u64) {
        todo!()
    }

    pub fn add_layer(&mut self, layer: Rc<RefCell<dyn Layer>>) {
        self.layers.push(layer);
    }

    fn animation_frame(&mut self) {
        todo!()
    }

    pub fn position(&self) -> &MapPosition {
        &self.position
    }

    pub fn set_center(&mut self, x: f32, y: f32) {
        self.position.set_center(x, y);
    }

    pub fn set_resolution(&mut self, resolution: f32) {
        self.position.set_resolution(resolution);
    }

    pub fn control(&mut self) -> MapControl {
        MapControl { map: self, settings: MapControlSettings::default() }
    }

    pub fn control_state(&self) -> &ControlState {
        &self.control_state
    }

    pub fn control_state_mut(&mut self) -> &mut ControlState {
        &mut self.control_state
    }

    pub fn trigger(&mut self, event: &MapEvent) {
        self.handle_event(event);
    }

    fn handle_event(&mut self, event: &MapEvent) {
        match event {
            MapEvent::Click(e) => self.trigger_event(*e, &self),
            MapEvent::DoubleClick(e) => {},
            MapEvent::Drag { dx, dy } => self.handle_drag(*dx, *dy),
            MapEvent::RightButtonDrag {dx, dy, cursor_position} => self.handle_right_button_drag(*dx, *dy, *cursor_position),
            MapEvent::MiddleButtonDrag {dx, dy} => self.handle_middle_button_drag(*dx, *dy),
            MapEvent::Zoom {delta, cursor_position} => self.handle_zoom(*delta, *cursor_position),
        }
    }

    fn handle_drag(&mut self, dx: i32, dy: i32) {
        self.position.translate_px(dx, dy);
    }

    fn handle_right_button_drag(&mut self, dx: i32, dy: i32, cursor_position: [i32; 2]) {
        let center = self.position.center();
        let position = self.position.get_map_position(&cursor_position);
        let v1 = [center[0] - position[0], center[1] - position[1]];
        let dx = dx as f32 * self.position.resolution();
        let dy = dy as f32 * self.position.resolution();

        let v2 = [v1[0] + dx, v1[1] + dy];
        let v1_len = (v1[0] * v1[0] + v1[1] * v1[1]).sqrt();
        let v2_len = (v2[0] * v2[0] + v2[1] * v2[1]).sqrt();
        if v1_len == 0. || v2_len == 0. {
            return;
        }

        let prod = (v1[0] * v2[0] + v1[1] * v2[1]) / v1_len / v2_len;
        if prod > 1. {
            return;
        }

        let mut angle = prod.acos();
        if v1[0] * v2[1] - v1[1] * v2[0] > 0.0 {
            angle = -angle;
        }

        self.position.rotate(0.0, angle);
    }

    fn handle_middle_button_drag(&mut self, dx: i32, dy: i32) {
        const ANGLE_STEP: f32 = 0.005;
        self.position.rotate(dy as f32 * ANGLE_STEP, 0.0);
    }

    fn handle_zoom(&mut self, delta: f32, cursor_position: [i32; 2]) {
        self.position.zoom(delta, cursor_position);
    }
}

impl<E> EventListener<E> for Map
    where E: Copy,
          HandlerStore: TypedHandlerStore<E>
{
    fn handler_store(&self) -> &HandlerStore {
        &self.handler_store
    }

    fn handler_store_mut(&mut self) -> &mut HandlerStore {
        &mut self.handler_store
    }
}

#[derive(Debug)]
pub struct MapPosition {
    screen_scale: na::Matrix4<f32>,
    scale: na::Matrix4<f32>,
    translate: na::Matrix4<f32>,
    rotation_x: f32,
    rotation_z: f32,
}

impl MapPosition {
    fn set_center(&mut self, x: f32, y: f32) {
        self.translate = na::Matrix4::new_translation(&na::Vector3::new(-x, -y, 0.0));
    }

    fn set_resolution(&mut self, resolution: f32) {
        self.scale = na::Matrix4::new_scaling(1.0 / resolution);
    }

    fn width_px(&self) -> f32 {
        2.0 / self.screen_scale[(0, 0)]
    }

    fn height_px(&self) -> f32 {
        2.0 / self.screen_scale[(1, 1)]
    }

    fn translate(&mut self, dx: f32, dy: f32) {
        todo!()
    }

    fn translate_px(&mut self, dx: i32, dy: i32) {
        let translate_px = na::Vector4::new(dx as f32, dy as f32, 0.0, 0.0);
        let rotated = self.inverse_rotation() * translate_px;
        let scaled = self.inverse_scale() * rotated;
        let translation = na::Matrix4::new_translation(&scaled.remove_fixed_rows::<1>(3));
        self.translate = self.translate * translation;
    }

    fn rotate(&mut self, x: f32, z: f32) {
        const MIN_X_ANGLE: f32 = 0.0;
        const MAX_X_ANGLE: f32 = 3.0 * std::f32::consts::FRAC_PI_8;
        self.rotation_x = (self.rotation_x + x).max(MIN_X_ANGLE).min(MAX_X_ANGLE);
        self.rotation_z += z;
    }

    fn rotation(&self) -> na::Matrix4<f32> {
        let x = na::Matrix4::from_axis_angle(&na::Unit::new_normalize(na::Vector3::new(1.0, 0.0, 0.0)), self.rotation_x);
        let z = na::Matrix4::from_axis_angle(&na::Unit::new_normalize(na::Vector3::new(0.0, 0.0, 1.0)), self.rotation_z);
        x * z
    }

    fn inverse_rotation(&self) -> na::Matrix4<f32> {
        let x = na::Matrix4::from_axis_angle(&na::Unit::new_normalize(na::Vector3::new(1.0, 0.0, 0.0)), -self.rotation_x);
        let z = na::Matrix4::from_axis_angle(&na::Unit::new_normalize(na::Vector3::new(0.0, 0.0, 1.0)), -self.rotation_z);
        z * x
    }

    fn zoom(&mut self, delta: f32, center_px: [i32; 2]) {
        let zoom_c = self.get_map_position(&center_px);
        let map_c = self.center();

        let dx = map_c[0] - zoom_c[0];
        let dy = map_c[1] - zoom_c[1];

        let dx_scaled = dx * (1.0 - delta);
        let dy_scaled = dy * (1.0 - delta);

        self.translate[(0, 3)] = -(map_c[0] + dx_scaled);
        self.translate[(1, 3)] = -(map_c[1] + dy_scaled);

        let transformation = na::Matrix4::new_nonuniform_scaling(&na::Vector3::new(delta, delta, delta));
        self.scale = self.scale * transformation;
    }

    pub fn matrix(&self) -> na::Matrix4<f32> {
        self.screen_scale * self.scale * self.rotation() * self.translate
    }

    fn set_screen_size(&mut self, width: u32, height: u32) {
        self.screen_scale = na::Matrix4::new_nonuniform_scaling(&na::Vector3::new(2.0 / width as f32, 2.0 / height as f32, 2.0 / width as f32));
    }

    fn inverse_scale(&self) -> na::Matrix4<f32> {
        na::Matrix4::new_nonuniform_scaling(&na::Vector3::new(1.0 / self.scale[(0, 0)], 1.0 / self.scale[(1, 1)], 1.0 / self.scale[(2, 2)]))
    }

    pub fn center(&self) -> Point {
        [-self.translate[(0, 3)], -self.translate[(1, 3)]]
    }

    pub fn resolution(&self) -> f32 {
        1.0 / self.scale[(0, 0)]
    }

    fn inverse_translation(&self) -> na::Matrix4<f32> {
        na::Matrix4::new_translation(&na::Vector3::new(-self.translate[(0, 3)], -self.translate[(1, 3)], 0.0))
    }

    fn half_screen_translation(&self) -> na::Matrix4<f32> {
        na::Matrix4::new_nonuniform_scaling(&na::Vector3::new(1.0, -1.0, 1.0)) *
            na::Matrix4::new_translation(&na::Vector3::new(-self.width_px() / 2.0, - self.height_px() / 2.0, 0.0))
    }

    fn inverse_screen_transformation(&self) -> na::Matrix4<f32> {
        self.inverse_translation() * self.inverse_scale() * self.inverse_rotation() * self.half_screen_translation()
    }

    pub fn get_map_position(&self, px_position: &[i32; 2]) -> Point {
        let point = na::Vector4::new(px_position[0] as f32, px_position[1] as f32, 0.0, 1.0);
        let transformed = self.inverse_screen_transformation() * point;
        [transformed[0], transformed[1]]
    }
}

impl Default for MapPosition {
    fn default() -> Self {
        use num_traits::identities::One;

        Self {
            screen_scale: na::Matrix4::one(),
            scale: na::Matrix4::one(),
            translate: na::Matrix4::one(),
            rotation_x: 0.0,
            rotation_z: 0.0,
        }
    }
}

struct MapAnimation {
    from: MapPosition,
    to: MapPosition,
    duration: u64,
    start: u64,
}
