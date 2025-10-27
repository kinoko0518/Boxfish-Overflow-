use crate::boxfish::PlayerCollidedAnimation;
use crate::prelude::*;
use bevy::prelude::*;

// Resources
const BOXFISH_PATH: &str = "embedded://boxfish/boxfish.png";
const BOOLEAN_PATH: &str = "embedded://boxfish/0_to_1_to_0.png";

#[derive(Resource, Default)]
/// Registing player's sprite map and atlas layout.
pub struct PlayerImage {
    image: Handle<Image>,
    atlas_layout: Handle<TextureAtlasLayout>,
}

impl PlayerImage {
    pub fn index_to_sprite(&self, x: usize, y: usize) -> Sprite {
        Sprite::from_atlas_image(self.image.clone(), self.index_to_atlas(x, y))
    }
    /// Getting a texture atlas of player's sprite map from x and y.
    pub fn index_to_atlas(&self, x: usize, y: usize) -> TextureAtlas {
        TextureAtlas {
            layout: self.atlas_layout.clone(),
            index: x + y * 4,
        }
    }
}

#[derive(Resource, Default)]
pub struct BooleanImage {
    image: Handle<Image>,
    atlas_layout: Handle<TextureAtlasLayout>,
}

impl BooleanImage {
    pub fn y_to_sprite(&self, y: usize) -> Sprite {
        Sprite::from_atlas_image(self.image.clone(), self.y_to_atlas(y))
    }
    fn y_to_atlas(&self, y: usize) -> TextureAtlas {
        TextureAtlas {
            layout: self.atlas_layout.clone(),
            index: y,
        }
    }
    pub fn zero(&self) -> TextureAtlas {
        self.y_to_atlas(0)
    }
    pub fn one(&self) -> TextureAtlas {
        self.y_to_atlas(10)
    }
}

pub fn assets_setup(
    mut player_image: ResMut<PlayerImage>,
    mut boolean_image: ResMut<BooleanImage>,
    asset_server: Res<AssetServer>,
) {
    player_image.image = asset_server.load(BOXFISH_PATH);
    player_image.atlas_layout = asset_server.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16),
        4,
        4,
        None,
        None,
    ));
    boolean_image.image = asset_server.load(BOOLEAN_PATH);
    boolean_image.atlas_layout = asset_server.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16),
        1,
        20,
        None,
        None,
    ));
}

pub fn face_manager(
    mut query: Query<(&mut Sprite, Option<&PlayerCollidedAnimation>, &Head)>,
    player_image: Res<PlayerImage>,
) {
    if let Ok((mut sprite, col_anim, head)) = query.single_mut() {
        // Boxfish's head texture is at 2nd row.
        // So, by specifing line, the face of boxfish can be specified.
        let face_kind = match col_anim {
            Some(_) => 2, // Surprising face
            None => {
                if head.is_expanding {
                    1 // Expanding face
                } else {
                    0 // Neutral face
                }
            }
        };
        sprite.texture_atlas = Some(player_image.index_to_atlas(2, face_kind));
    }
}
