use std::cmp::max;
use std::{cell, collections::HashMap, path::PathBuf};

use macroquad::{
    color::{BLACK, BLANK, Color, WHITE},
    texture::Image,
};
// use macroquad::texture::Image;
use open_jsw_tiled::tiled::{
    layer::{Layer, LayerType},
    map::{Map, MapOrientation},
    property::{Property, PropertyVal, property_type},
    tileset::Tileset,
};

use crate::raw_game::{CellBehaviour, JswRawGame, JswRawRoom, ROOM_LAYOUT_SIZE};

use super::Converter;

const EMPTY_CELL_SPRITE: [u8; 8] = [0; 8];
const TRANSPARENT: Color = Color::new(0.0, 0.0, 0.0, 0.0);

pub struct RawToTiledConverter;

pub struct MapWithSpritesheet {
    pub map: Map,
    pub spritesheet: Image,
}

struct ConvertContext {
    rooms: HashMap<u8, RoomContext>,
    cell_sprites: SpriteSetContext,

    empty_cell_sprite: Image,
}

struct RoomContext {
    cells: HashMap<u8, CellContext>,
}

struct CellContext {
    bg_sprite_id: u32,
    fg_sprite_id: u32,
}

struct SpriteSetContext {
    sprites: HashMap<u32, Image>,
    next_sprite_id: u32,
}

impl Converter<JswRawGame, MapWithSpritesheet> for RawToTiledConverter {
    fn convert(&self, raw_game: &JswRawGame) -> MapWithSpritesheet {
        let mut context = ConvertContext::new();
        let mut map = Map::new(None, MapOrientation::Orthogonal, 32, 24, 8, 8);

        let room_layers = self.convert_rooms(&mut context, &mut map, &raw_game.rooms);

        // Create the spritesheet
        let spritesheet = self.create_spritesheet(&context, &raw_game.rooms);

        // Add a dummy tileset
        let tileset = Tileset::new(
            "tiles".to_string(),
            "spritesheet.png".to_string(),
            8 * 16,
            8 * 16,
            8,
            8,
            1,
        );

        map.layers = room_layers;
        map.tilesets.push(tileset);

        MapWithSpritesheet { map, spritesheet }
    }
}

impl RawToTiledConverter {
    fn convert_rooms(
        &self,
        context: &mut ConvertContext,
        map: &mut Map,
        rooms: &Vec<JswRawRoom>,
    ) -> Vec<Layer> {
        let mut layers = Vec::new();

        for room in rooms {
            layers.push(self.convert_room(context, map, room));
        }

        // Layers are stored in reverse order
        layers.reverse();

        layers
    }

    fn convert_room(
        &self,
        context: &mut ConvertContext,
        map: &mut Map,
        room: &JswRawRoom,
    ) -> Layer {
        let mut room_context = RoomContext::new();
        let mut room_layer = Layer::new(map, LayerType::Group, room.name.clone());
        let mut room_layer_layers = Vec::new();

        let mut bg_layer = Layer::new(map, LayerType::TileLayer, "Background 1".to_string());
        bg_layer.class = Some("bg".to_string());
        bg_layer.visible = true;

        let mut object_layer = Layer::new(map, LayerType::ObjectGroup, "Dynamic 1".to_string());
        object_layer.class = Some("dynamic".to_string());
        object_layer.visible = true;

        let mut fg_layer = Layer::new(map, LayerType::TileLayer, "Foreground 1".to_string());
        fg_layer.class = Some("fg".to_string());
        fg_layer.visible = true;

        // TODO - set the background colour from the first cell (air)
        // NEED TO IMPLEMENT PROPERTY SERIALIZATION
        // let mut bg_colour = BLACK;
        // if let Some(c) = room.cells.first().map(|c| c.paper) {
        //     bg_colour = c;
        // }
        // room_layer.properties.push(Property {
        //     typ: property_type::STRING.to_string(),
        //     name: "bg_color".to_string(),
        //     value: PropertyVal::String(color_to_string(bg_colour)),
        // });

        // Convert the cells to sprites
        let cell_sprites = &mut context.cell_sprites;
        for cell in &room.cells {
            // Skip air cells
            // if cell.behaviour == CellBehaviour::Air {
            //     continue;
            // }

            let bg_sprite_image = self.create_image_from_sprite_data(
                &EMPTY_CELL_SPRITE,
                8,
                8,
                TRANSPARENT,
                cell.paper,
            );
            let fg_sprite_image =
                self.create_image_from_sprite_data(&cell.sprite, 8, 8, cell.ink, TRANSPARENT);

            // See if the same image already exists, otherwise add it
            let mut find_sprite_id = |image: Image| -> u32 {
                let sprite_id = cell_sprites
                    .sprites
                    .iter()
                    .find(|(_, existing_image)| existing_image.bytes == image.bytes)
                    .map(|(id, _)| *id)
                    .unwrap_or_else(|| {
                        let sprite_id = cell_sprites.get_next_sprite_id();
                        cell_sprites.sprites.insert(sprite_id, image);
                        sprite_id
                    });
                sprite_id
            };

            let bg_sprite_id = find_sprite_id(bg_sprite_image);
            let fg_sprite_id = find_sprite_id(fg_sprite_image);

            let cell_context = CellContext {
                bg_sprite_id,
                fg_sprite_id,
            };
            room_context.cells.insert(cell.id, cell_context);
        }

        // Add some tiles to the static layer
        let cols = bg_layer.width.unwrap() as usize;
        let mut bg_data = bg_layer.get_tile_matrix();
        let mut fg_data = fg_layer.get_tile_matrix();
        for (i, cell_id) in room.layout.iter().enumerate().take(ROOM_LAYOUT_SIZE) {
            let col = i % cols;
            let row = i / cols;

            let cell = room.cells.iter().find(|c| c.id == *cell_id);

            // If the cell is not found, use the first cell (and if there is no first cell, panic:
            // TODO - don't panic, but return a Result<Error>.
            let cell = cell
                .or_else(|| room.cells.first())
                .unwrap_or_else(|| panic!("No cells found for room '{}'", room.name));

            if let Some(cell_context) = room_context.cells.get(&cell.id) {
                bg_data[row][col] = cell_context.bg_sprite_id;
                fg_data[row][col] = cell_context.fg_sprite_id;
            } else {
                bg_data[row][col] = 0;
                fg_data[row][col] = 0;
            };
        }

        // data[0][0] = 1;
        // data[20][10] = 1;

        room_layer_layers.push(bg_layer);
        room_layer_layers.push(object_layer);
        room_layer_layers.push(fg_layer);

        // Layers are stored in reverse order
        // room_layer_layers.reverse();
        room_layer.layers = Some(room_layer_layers);

        // Add the room to the context
        context.rooms.insert(room.room_no, room_context);

        room_layer
    }

    fn create_spritesheet(&self, context: &ConvertContext, rooms: &[JswRawRoom]) -> Image {
        // let mut sprites: HashMap<u32, Image> = HashMap::new();

        // let mut sprite_images: Vec<&Image> = Vec::new();
        // let context_rooms = &context.rooms;
        // for room in rooms {
        //     let room_context = context_rooms.get(&room.room_no).unwrap();
        //     for cell in &room.cells {
        //         let cell_context = room_context.cells.get(&cell.id).unwrap();
        //         sprite_images.push(cell_context.);
        //         // sprites.insert(cell_context.bg_sprite_id, cell.bg_sprite.clone());
        //         // sprites.insert(cell_context.fg_sprite_id, cell.fg_sprite.clone());
        //     }
        // }

        let sprite_images: Vec<&Image> = context.get_cell_sprites_array();
        let spritesheet = create_spritesheet(sprite_images);

        // Save to png
        let mut path = PathBuf::from("tmp");
        path.push("spritesheet.png");
        image::save_buffer(&path, &spritesheet.bytes, 128, 128, image::ColorType::Rgba8).unwrap();

        // Hack, write to pngs
        for (id, sprite) in context.cell_sprites.sprites.iter() {
            let mut path = PathBuf::from("tmp");
            path.push(format!("sprite_{}.png", id));

            // Save to png
            image::save_buffer(&path, &sprite.bytes, 8, 8, image::ColorType::Rgba8).unwrap();
        }

        // TODO build the sprites into a spritesheet

        spritesheet
    }

    fn create_image_from_sprite_data(
        &self,
        data: &[u8],
        width: usize,
        height: usize,
        fg: Color,
        bg: Color,
    ) -> Image {
        let mut image = Image::gen_image_color(width as u16, height as u16, BLANK);
        let mut colors: Vec<Color> = vec![BLANK; width * height];
        for (i, r) in data.iter().enumerate() {
            for c in 0..width {
                let bit = (r >> c) & 1;
                let mut color = bg;
                if bit == 1 {
                    color = fg;
                }
                colors[i * 8 + c] = color;
            }
        }
        image.update(&colors);
        image
    }
}

impl ConvertContext {
    fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            cell_sprites: SpriteSetContext::new(),

            empty_cell_sprite: Image::gen_image_color(8, 8, BLANK),
        }
    }

    fn get_cell_sprites_array(&self) -> Vec<&Image> {
        let mut sprites_array: Vec<&Image> = vec![];

        let key_max = self.cell_sprites.sprites.keys().max().unwrap_or(&0);

        for i in 1..*key_max {
            let sprite = self.cell_sprites.sprites.get(&i);

            if let Some(sprite) = sprite {
                sprites_array.push(sprite);
            } else {
                sprites_array.push(&self.empty_cell_sprite);
            }
        }

        sprites_array
    }
}

impl RoomContext {
    fn new() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }
}

impl SpriteSetContext {
    fn new() -> Self {
        Self {
            sprites: HashMap::new(),
            next_sprite_id: 1,
        }
    }

    fn get_next_sprite_id(&mut self) -> u32 {
        let sprite_id = self.next_sprite_id;
        self.next_sprite_id += 1;
        sprite_id
    }
}

fn color_to_string(color: Color) -> String {
    format!(
        "#{:02x}{:02x}{:02x}{:02x}",
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8,
        (color.a * 255.0) as u8
    )
}

fn create_spritesheet(images: Vec<&Image>) -> Image {
    let count = images.len() as u32;
    let max_width = images.iter().map(|img| img.width as u32).max().unwrap_or(1);
    let max_height = images
        .iter()
        .map(|img| img.height as u32)
        .max()
        .unwrap_or(1);

    let mut grid_size = 1;
    while grid_size * grid_size < count {
        grid_size *= 2;
    }

    let raw_width = grid_size * max_width;
    let raw_height = grid_size * max_height;

    // Make sure the sheet is square and size is a power of two
    let side = max(raw_width, raw_height).next_power_of_two();

    let mut sheet_bytes = vec![0u8; (side * side * 4) as usize];

    for (i, image) in images.iter().enumerate() {
        let col = (i as u32) % grid_size;
        let row = (i as u32) / grid_size;

        let x_offset = col * max_width;
        let y_offset = row * max_height;

        let image_data = image.get_image_data();

        for y in 0..image.height as u32 {
            for x in 0..image.width as u32 {
                let dst_index = (((y_offset + y) * side + (x_offset + x)) * 4) as usize;
                let src_index = (y * image.width as u32 + x) as usize;
                sheet_bytes[dst_index..dst_index + 4].copy_from_slice(&image_data[src_index]);
            }
        }
    }

    Image {
        bytes: sheet_bytes,
        width: side as u16,
        height: side as u16,
    }
}
