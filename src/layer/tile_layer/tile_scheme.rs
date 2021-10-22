use crate::layer::tile_layer::TileIndex;
use crate::map::Envelope;
use crate::Point;
use sorted_vec::SortedVec;
use std::cmp::Ordering;

const LEVEL_SELECTION_TOLERANCE: f32 = 0.99;

pub struct TileSchema {
    tile_width: u32,
    tile_height: u32,
    origin: Point,
    envelope: Envelope,
    y_direction: TilesDirection,
    levels: SortedVec<TileLevel>,
}

enum TilesDirection {
    BottomToTop,
    TopToBottom,
}

impl TileSchema {
    pub fn get_indices(&self, bbox: &Envelope, resolution: f32) -> Vec<TileIndex> {
        let level = self.select_level(resolution);
        let width = level.resolution * self.tile_width as f32;
        let height = level.resolution * self.tile_height as f32;

        let x_min = ((bbox.x_min.max(self.envelope.x_min) - self.origin[0]) / width).floor() as i32;
        let x_max = ((bbox.x_max.min(self.envelope.x_max) - self.origin[0]) / width).ceil() as i32;
        let (y_min, y_max) = match &self.y_direction {
            TilesDirection::BottomToTop => {
                let y_min = ((bbox.y_min.max(self.envelope.y_min) - self.origin[1]) / height)
                    .floor() as i32;
                let y_max =
                    ((bbox.y_max.min(self.envelope.y_max) - self.origin[1]) / height).ceil() as i32;

                (y_min, y_max)
            }
            TilesDirection::TopToBottom => {
                let y_min_from_origin = self.origin[1] - bbox.y_max.min(self.envelope.y_max);
                let y_max_from_origin = self.origin[1] - bbox.y_min.max(self.envelope.y_min);

                (
                    (y_min_from_origin / height).floor() as i32,
                    (y_max_from_origin / height).ceil() as i32,
                )
            }
        };

        let mut tiles = vec![];
        for x in x_min..x_max {
            for y in y_min..y_max {
                tiles.push(TileIndex {
                    x,
                    y,
                    z: level.z_index,
                });
            }
        }

        tiles
    }

    fn select_level(&self, resolution: f32) -> &TileLevel {
        for index in 0..self.levels.len() {
            let level = &self.levels[index];
            if level.resolution * LEVEL_SELECTION_TOLERANCE < resolution {
                return level;
            }
        }

        self.levels.last().unwrap()
    }
}

impl Default for TileSchema {
    fn default() -> Self {
        let mut levels = vec![TileLevel {
            z_index: 0,
            resolution: 156543.03392800014,
        }];

        for i in 1..18 {
            levels.push(TileLevel {
                z_index: i as i32,
                resolution: levels[i - 1].resolution / 2.0,
            });
        }

        Self {
            tile_width: 256,
            tile_height: 256,
            origin: [-20037508.342787, 20037508.342787],
            envelope: Envelope::new(
                -20037508.342787,
                -20037508.342787,
                20037508.342787,
                20037508.342787,
            ),
            y_direction: TilesDirection::TopToBottom,
            levels: SortedVec::from(levels),
        }
    }
}

#[derive(Debug)]
pub struct TileLevel {
    z_index: i32,
    resolution: f32,
}

impl PartialEq<Self> for TileLevel {
    fn eq(&self, other: &Self) -> bool {
        self.resolution.eq(&other.resolution)
    }
}

impl Eq for TileLevel {}

impl PartialOrd<Self> for TileLevel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.resolution
            .partial_cmp(&other.resolution)
            .map(|x| x.reverse())
    }
}

impl Ord for TileLevel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels_are_in_correct_order() {
        let default_scheme = TileSchema::default();
        assert_eq!(0, default_scheme.levels[0].z_index);

        let levels = vec![
            TileLevel {
                z_index: 1,
                resolution: 1.0,
            },
            TileLevel {
                z_index: 2,
                resolution: 2.0,
            },
        ];
        let sorted = SortedVec::from(levels);
        assert_eq!(2.0, sorted[0].resolution);
    }

    #[test]
    fn select_level() {
        let default_schema = TileSchema::default();
        let resolution = default_schema.levels[0].resolution;
        assert_eq!(
            &default_schema.levels[0],
            default_schema.select_level(resolution)
        );
        assert_eq!(
            &default_schema.levels[0],
            default_schema.select_level(resolution + 1.0)
        );
        assert_eq!(
            &default_schema.levels[0],
            default_schema.select_level(resolution - 1.0)
        );
        assert_eq!(
            &default_schema.levels[0],
            default_schema.select_level(resolution * 2.0)
        );
        assert_eq!(
            &default_schema.levels[1],
            default_schema.select_level(resolution / 2.0)
        );
        assert_eq!(
            &default_schema.levels[1],
            default_schema.select_level(resolution / 1.5)
        );

        let last_level = default_schema.levels.last().unwrap();
        assert_eq!(
            last_level.resolution,
            default_schema
                .select_level(last_level.resolution / 2.0)
                .resolution
        );
        assert_eq!(
            last_level.resolution,
            default_schema
                .select_level(last_level.resolution / 4.0)
                .resolution
        );
    }

    fn get_simple_schema() -> TileSchema {
        TileSchema {
            tile_width: 10,
            tile_height: 10,
            origin: [0.0, 0.0],
            envelope: Envelope::new(0.0, 0.0, 10.0, 10.0),
            y_direction: TilesDirection::BottomToTop,
            levels: SortedVec::from(vec![
                TileLevel {
                    resolution: 1.0,
                    z_index: 0,
                },
                TileLevel {
                    resolution: 0.5,
                    z_index: 1,
                },
                TileLevel {
                    resolution: 0.25,
                    z_index: 2,
                },
            ]),
        }
    }

    #[test]
    fn get_indices_simple_schema() {
        let schema = get_simple_schema();
        let envelope = Envelope::new(0.0, 0.0, 10.0, 10.0);
        let indices = schema.get_indices(&envelope, 1.0);
        assert_eq!(1, indices.len());
        assert_eq!(TileIndex::new(0, 0, 0), indices[0]);

        let indices = schema.get_indices(&envelope, 0.5);
        assert_eq!(4, indices.len());
        for index in indices {
            assert_eq!(1, index.z);
            assert!(index.x >= 0 && index.x <= 1);
            assert!(index.y >= 0 && index.y <= 1);
        }

        let indices = schema.get_indices(&envelope, 0.25);
        assert_eq!(16, indices.len());
        for index in indices {
            assert_eq!(2, index.z);
            assert!(index.x >= 0 && index.x <= 3);
            assert!(index.y >= 0 && index.y <= 3);
        }
    }

    #[test]
    fn get_indices_partial_intersection() {
        let schema = get_simple_schema();
        let envelope = Envelope::new(2.0, 2.0, 8.0, 4.0);
        let indices = schema.get_indices(&envelope, 0.5);
        assert_eq!(2, indices.len());
        for index in indices {
            assert_eq!(1, index.z);
            assert!(index.x >= 0 && index.x <= 1);
            assert!(index.y >= 0 && index.y <= 0);
        }
    }

    #[test]
    fn get_indices_large_bbox() {
        let schema = get_simple_schema();
        let envelope = Envelope::new(-10.0, -10.0, 20.0, 20.0);
        let indices = schema.get_indices(&envelope, 0.5);
        assert_eq!(4, indices.len());
        for index in indices {
            assert!(index.x >= 0 && index.x <= 1);
            assert!(index.y >= 0 && index.y <= 1);
        }
    }

    #[test]
    fn get_indices_high_resolution() {
        let schema = get_simple_schema();
        let envelope = Envelope::new(2.0, 2.0, 3.0, 3.0);
        let indices = schema.get_indices(&envelope, 0.125);
        assert_eq!(4, indices.len());
        for index in indices {
            assert_eq!(2, index.z);
            assert!(index.x >= 0 && index.x <= 1, "Invalid x value {}", index.x);
            assert!(index.y >= 0 && index.y <= 1, "Invalid y value {}", index.y);
        }
    }

    #[test]
    fn get_indices_no_intersection() {
        let schema = get_simple_schema();
        let envelope = Envelope::new(-10.0, -10.0, -5.0, -5.0);
        let indices = schema.get_indices(&envelope, 0.5);
        assert_eq!(0, indices.len());
    }

    #[test]
    fn get_indices_top_to_bottom() {
        let mut schema = get_simple_schema();
        schema.origin = [0.0, 10.0];
        schema.y_direction = TilesDirection::TopToBottom;
        let envelope = Envelope::new(2.0, 2.0, 3.0, 3.0);
        let indices = schema.get_indices(&envelope, 0.125);
        assert_eq!(4, indices.len());
        for index in indices {
            assert_eq!(2, index.z);
            assert!(index.x >= 0 && index.x <= 1, "Invalid x value {}", index.x);
            assert!(index.y >= 2 && index.y <= 3, "Invalid y value {}", index.y);
        }
    }

    #[test]
    fn get_indices_different_tile_size() {
        let mut schema = get_simple_schema();
        schema.tile_width = 5;
        schema.tile_height = 3;
        let envelope = Envelope::new(2.0, 2.0, 7.0, 7.0);
        let indices = schema.get_indices(&envelope, 1.0);
        assert_eq!(6, indices.len());
        for index in indices {
            assert_eq!(0, index.z);
            assert!(index.x >= 0 && index.x <= 1, "Invalid x value {}", index.x);
            assert!(index.y >= 0 && index.y <= 2, "Invalid y value {}", index.y);
        }

        let indices = schema.get_indices(&envelope, 0.5);
        assert_eq!(12, indices.len());
        for index in indices {
            assert_eq!(1, index.z);
            assert!(index.x >= 0 && index.x <= 2, "Invalid x value {}", index.x);
            assert!(index.y >= 1 && index.y <= 4, "Invalid y value {}", index.y);
        }
    }

    #[test]
    fn get_indices_moved_origin() {
        let mut schema = get_simple_schema();
        schema.origin = [5.0, 5.0];
        let envelope = Envelope::new(2.5, 2.5, 7.5, 7.5);
        let indices = schema.get_indices(&envelope, 0.25);
        assert_eq!(4, indices.len());
        for index in indices {
            assert_eq!(2, index.z);
            assert!(index.x >= -1 && index.x <= 1, "Invalid x value {}", index.x);
            assert!(index.y >= -1 && index.y <= 1, "Invalid y value {}", index.y);
        }
    }
}
