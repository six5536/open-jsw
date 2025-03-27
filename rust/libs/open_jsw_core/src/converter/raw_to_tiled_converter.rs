use std::collections::HashMap;

use macroquad::{color::Color, texture::Image};
// use macroquad::texture::Image;
use open_jsw_tiled::tiled::{
    layer::{Layer, LayerType},
    map::{Map, MapOrientation},
    tileset::Tileset,
};

use crate::{
    Error, Result,
    error::GameConversionError,
    image::{TRANSPARENT, create_image_from_sprite_data, create_spritesheet},
    raw_game::{JswRawGame, JswRawRoom, ROOM_LAYOUT_SIZE},
};

use super::Converter;

const CELL_WIDTH: usize = 8;
const CELL_HEIGHT: usize = 8;
const CELL_BYTES: usize = (CELL_WIDTH / 8) * CELL_HEIGHT;
const EMPTY_CELL_SPRITE: [u8; CELL_BYTES] = [0; CELL_BYTES];

pub struct RawToTiledConverter;

pub struct MapWithSpritesheet {
    pub map: Map,
    pub cell_spritesheet: Image,
    pub cell_sprites: HashMap<u32, Image>,
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
    fn convert(&self, raw_game: &JswRawGame) -> Result<MapWithSpritesheet> {
        let mut context = ConvertContext::new();
        let mut map = Map::new(
            None,
            MapOrientation::Orthogonal,
            32,
            24,
            CELL_WIDTH as u32,
            CELL_HEIGHT as u32,
        );

        let room_layers = self.convert_rooms(&mut context, &mut map, &raw_game.rooms)?;

        // Create the spritesheet
        let cell_spritesheet = self.create_cell_spritesheet(&context, &mut map)?;

        map.layers = room_layers;

        Ok(MapWithSpritesheet {
            map,
            cell_spritesheet,
            cell_sprites: context.cell_sprites.sprites,
        })
    }
}

impl RawToTiledConverter {
    fn convert_rooms(
        &self,
        context: &mut ConvertContext,
        map: &mut Map,
        rooms: &Vec<JswRawRoom>,
    ) -> Result<Vec<Layer>> {
        let mut layers = Vec::new();

        for room in rooms {
            layers.push(self.convert_room(context, map, room)?);
        }

        // Layers are stored in reverse order
        layers.reverse();

        Ok(layers)
    }

    fn convert_room(
        &self,
        context: &mut ConvertContext,
        map: &mut Map,
        room: &JswRawRoom,
    ) -> Result<Layer> {
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

            let bg_sprite_image = create_image_from_sprite_data(
                &EMPTY_CELL_SPRITE,
                CELL_WIDTH,
                CELL_HEIGHT,
                TRANSPARENT,
                cell.paper,
            )?;
            let fg_sprite_image = create_image_from_sprite_data(
                &cell.sprite,
                CELL_WIDTH,
                CELL_HEIGHT,
                cell.ink,
                TRANSPARENT,
            )?;

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

            // Find the cell for the cell_id
            // If the cell is not found, use the first cell (and if there is no first cell, raise an error):
            let cell = room
                .cells
                .iter()
                .find(|c| c.id == *cell_id)
                .or(room.cells.first());

            if let Some(cell) = cell {
                if let Some(cell_context) = room_context.cells.get(&cell.id) {
                    bg_data[row][col] = cell_context.bg_sprite_id;
                    fg_data[row][col] = cell_context.fg_sprite_id;
                } else {
                    bg_data[row][col] = 0;
                    fg_data[row][col] = 0;
                };
            } else {
                return Err(Error::GameConversionFailed {
                    mode: GameConversionError::RoomConversionFailed {
                        room: room.name.clone(),
                    },
                    message: "No cells found".to_string(),
                });
            }
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

        Ok(room_layer)
    }

    fn create_cell_spritesheet(&self, context: &ConvertContext, map: &mut Map) -> Result<Image> {
        let sprite_images: Vec<&Image> = context.get_cell_sprites_vec();
        let spritesheet = create_spritesheet(sprite_images);

        // Add the tileset for the spritesheet
        let tileset = Tileset::new(
            "cells".to_string(),
            "gfx/cells.png".to_string(),
            spritesheet.width as u32,
            spritesheet.height as u32,
            8,
            8,
            1,
        );
        map.tilesets.push(tileset);

        Ok(spritesheet)
    }
}

impl ConvertContext {
    fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            cell_sprites: SpriteSetContext::new(),

            empty_cell_sprite: Image::gen_image_color(
                CELL_WIDTH as u16,
                CELL_HEIGHT as u16,
                TRANSPARENT,
            ),
        }
    }

    fn get_cell_sprites_vec(&self) -> Vec<&Image> {
        let mut sprites_vec: Vec<&Image> = vec![];

        let key_max = self.cell_sprites.sprites.keys().max().unwrap_or(&0);

        for i in 1..=*key_max {
            let sprite = self.cell_sprites.sprites.get(&i);

            if let Some(sprite) = sprite {
                sprites_vec.push(sprite);
            } else {
                sprites_vec.push(&self.empty_cell_sprite);
            }
        }

        sprites_vec
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
