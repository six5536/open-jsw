use open_jsw_tiled::tiled::{
    layer::{Layer, LayerType},
    map::{Map, MapOrientation},
    tileset::Tileset,
};

use crate::raw_game::{JswRawGame, JswRawRoom, ROOM_LAYOUT_SIZE};

use super::Converter;

pub struct RawToTiledConverter;

impl Converter<JswRawGame, Map> for RawToTiledConverter {
    fn convert(&self, raw_game: &JswRawGame) -> Map {
        let mut map = Map::new(None, MapOrientation::Orthogonal, 32, 24, 32, 32);

        let room_layers = self.convert_rooms(&mut map, &raw_game.rooms);

        // Add a dummy tileset
        let tileset = Tileset::new(
            "tiles".to_string(),
            "tileset.png".to_string(),
            160,
            160,
            32,
            32,
            1,
        );

        map.layers = room_layers;
        map.tilesets.push(tileset);

        map
    }
}

impl RawToTiledConverter {
    fn convert_rooms(&self, map: &mut Map, rooms: &Vec<JswRawRoom>) -> Vec<Layer> {
        let mut layers = Vec::new();

        for room in rooms {
            layers.push(self.convert_room(map, room));
        }

        // Layers are stored in reverse order
        layers.reverse();

        layers
    }

    fn convert_room(&self, map: &mut Map, room: &JswRawRoom) -> Layer {
        let mut room_layer = Layer::new(map, LayerType::Group, room.name.clone());
        let mut room_layer_layers = Vec::new();

        let mut static_layer = Layer::new(map, LayerType::TileLayer, "Static 1".to_string());
        static_layer.class = Some("static".to_string());
        static_layer.visible = true;

        let mut object_layer = Layer::new(map, LayerType::ObjectGroup, "Dynamic 1".to_string());
        object_layer.class = Some("dynamic".to_string());
        object_layer.visible = true;

        // Add some tiles to the static layer
        let cols = static_layer.width.unwrap() as usize;
        let mut data = static_layer.get_tile_matrix();
        room.layout
            .iter()
            .enumerate()
            .take(ROOM_LAYOUT_SIZE)
            .for_each(|(i, &tile)| {
                let col = i % cols;
                let row = i / cols;
                let mut v = 0;
                if tile > 0 {
                    v = 1;
                }
                data[row][col] = v;
            });

        // data[0][0] = 1;
        // data[20][10] = 1;

        room_layer_layers.push(static_layer);
        room_layer_layers.push(object_layer);

        // Layers are stored in reverse order
        room_layer_layers.reverse();
        room_layer.layers = Some(room_layer_layers);

        room_layer
    }
}
