use crate::layer::Layer;
use crate::map::MapPosition;
use crate::render_target::RenderTarget;

mod tile_scheme;

pub struct TileLayer {}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TileIndex {
    x: i32,
    y: i32,
    z: i32,
}

impl TileIndex {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl TileLayer {
    fn update_tile_list(&mut self, position: &MapPosition, screen_size: (u32, u32)) {
        let required_tiles = self.get_required_tiles(position, screen_size);
        let tile_store = self.get_tile_store();
        for tile in &required_tiles {
            if !tile_store.contains(tile) {
                tile_store.add_tile(tile);
            }
        }

        for tile in tile_store.tiles().filter(|t| !required_tiles.contains(t)) {
            tile_store.remove_if_not_needed(&tile);
        }
    }

    fn get_required_tiles(
        &self,
        position: &MapPosition,
        screen_size: (u32, u32),
    ) -> Vec<TileIndex> {
        todo!()
    }

    fn get_tile_store(&self) -> &mut TileStore {
        todo!()
    }

    fn render_current_tiles(&self, target: &RenderTarget, position: &MapPosition) {
        todo!();
    }
}

impl Layer for TileLayer {
    fn draw(&mut self, target: &RenderTarget, position: &MapPosition) {
        self.update_tile_list(position, target.get_dimensions());
        self.render_current_tiles(target, position);
    }
}

struct TileStore {}

impl TileStore {
    fn contains(&self, tile: &TileIndex) -> bool {
        todo!()
    }

    fn add_tile(&mut self, tile: &TileIndex) {
        todo!()
    }

    fn tiles(&self) -> impl Iterator<Item = TileIndex> {
        todo!();

        // to prevent compilation error until implemented
        vec![TileIndex { x: 0, y: 9, z: 0 }].into_iter()
    }

    fn remove_if_not_needed(&mut self, tile: &TileIndex) {
        todo!()
    }
}
