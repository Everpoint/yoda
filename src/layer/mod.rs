use crate::map::MapPosition;
use crate::render_target::RenderTarget;

mod static_layer;
pub use static_layer::StaticLayer;

mod tile_layer;

pub trait Layer {
    fn draw(&mut self, target: &RenderTarget, position: &MapPosition);
    fn feature_at_point(
        &self,
        _target: &RenderTarget,
        _screen_position: [i32; 2],
        _map_position: &MapPosition,
    ) -> Option<usize> {
        None
    }
}
