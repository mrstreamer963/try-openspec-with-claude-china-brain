use crate::components::*;

pub struct GameMap {
    pub tiles: Vec<Vec<Tile>>,
    pub size: u16,
}

impl GameMap {
    pub fn new(size: u16) -> Self {
        let mut tiles = Vec::with_capacity(size as usize);
        for y in 0..size {
            let mut row = Vec::with_capacity(size as usize);
            for x in 0..size {
                row.push(Tile { kind: Self::generate_tile(x, y, size), building: None });
            }
            tiles.push(row);
        }
        GameMap { tiles, size }
    }

    fn generate_tile(x: u16, y: u16, size: u16) -> TileKind {
        let h = (x as u32).wrapping_mul(0x9e3779b9)
            .wrapping_add(y as u32)
            .wrapping_mul(0x9e3779b9);
        let r = h % 100;
        let center = size as f32 / 2.0;
        let dx = (x as f32 - center).abs();
        let dy = (y as f32 - center).abs();
        if dx < 5.0 && dy < 5.0 { return TileKind::Grass; }
        if x < 2 || y < 2 || x >= size - 2 || y >= size - 2 {
            if r < 60 { return TileKind::Water; }
        }
        match r {
            0..=14 => TileKind::Water,
            15..=39 => TileKind::Sand,
            _ => TileKind::Grass,
        }
    }

    pub fn is_walkable(&self, x: f32, y: f32) -> bool {
        let gx = x.floor() as u16;
        let gy = y.floor() as u16;
        if gx >= self.size || gy >= self.size { return false; }
        self.tiles[gy as usize][gx as usize].kind != TileKind::Water
    }

    pub fn get_tile(&self, x: u16, y: u16) -> Option<&Tile> {
        if x >= self.size || y >= self.size { return None; }
        Some(&self.tiles[y as usize][x as usize])
    }

    pub fn get_tile_mut(&mut self, x: u16, y: u16) -> Option<&mut Tile> {
        if x >= self.size || y >= self.size { return None; }
        Some(&mut self.tiles[y as usize][x as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_size() {
        let map = GameMap::new(30);
        assert_eq!(map.tiles.len(), 30);
        assert_eq!(map.tiles[0].len(), 30);
    }

    #[test]
    fn test_center_walkable() {
        let map = GameMap::new(30);
        assert!(map.is_walkable(15.0, 15.0));
    }

    #[test]
    fn test_water_not_walkable() {
        let map = GameMap::new(30);
        for y in 0..30u16 {
            for x in 0..30u16 {
                let tile = map.get_tile(x, y).unwrap();
                assert_eq!(map.is_walkable(x as f32, y as f32), tile.kind != TileKind::Water);
            }
        }
    }
}