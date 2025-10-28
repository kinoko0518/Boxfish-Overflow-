use bevy::prelude::*;

const LOGIGATE_TILESET: &str = "embedded://tile/logical_gates.png";
const OUTLINE_TILESET: &str = "embedded://tile/aquarium.png";
const WALL_SPRITE: &str = "embedded://tile/wall.png";
const GOAL_SPRITE: &str = "embedded://tile/goal.png";

#[derive(Resource, Default)]
pub struct AquariumResource {
    pub tile_sprite: Handle<Image>,
    pub tile_layout: Handle<TextureAtlasLayout>,
    pub outline_sprite: Handle<Image>,
    pub outline_layout: Handle<TextureAtlasLayout>,
    pub wall_sprite: Handle<Image>,
    pub goal_sprite: Handle<Image>,
}

pub fn init_aquarium_resource(
    mut resource: ResMut<AquariumResource>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    resource.tile_sprite = asset_server.load(LOGIGATE_TILESET);
    resource.tile_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16),
        16,
        16,
        None,
        None,
    ));
    resource.outline_sprite = asset_server.load(OUTLINE_TILESET);
    resource.outline_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16),
        4,
        4,
        None,
        None,
    ));
    resource.wall_sprite = asset_server.load(WALL_SPRITE);
    resource.goal_sprite = asset_server.load(GOAL_SPRITE);
}
