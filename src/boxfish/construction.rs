use crate::boxfish::{BooleanImage, visual::PlayerImage};
use crate::prelude::*;
use bevy::prelude::*;

/// Called when the game just executed once.
///
/// Spawn the boxfish's head which not deleted during game.
pub fn spawn_boxfishs_head(mut commands: Commands, player_image: Res<PlayerImage>) {
    commands.spawn((
        player_image.index_to_sprite(2, 0),
        Transform::from_xyz(0., 0., PLAYER_LAYER),
        Head {
            is_expanding: false,
            history: Vec::new(),
        },
        Player,
        TileCoords {
            tile_pos: IVec2::new(0, 0),
        },
    ));
}

/// Apply the stage that just loaded to the boxfish.
pub fn update_player_to_just_loaded_stage(
    mut head_query: Query<(
        Entity,
        Option<&Children>,
        &mut Transform,
        &mut TileCoords,
        &mut Head,
    )>,
    mut construct_aquarium: EventReader<ConstructAquarium>,
    mut commands: Commands,
    player_image: Res<PlayerImage>,
    boolean_image: Res<BooleanImage>,
) {
    // Reading the new stage
    let aquarium = match construct_aquarium.read().next() {
        Some(aq) => aq,
        None => return,
    };
    // Getting the head of the boxfish
    let mut head = match head_query.single_mut() {
        Ok(h) => h,
        Err(_) => panic!("The head of the boxfish not found"),
    };
    // Updating coords
    head.3.tile_pos = aquarium.player_origin;
    head.2.translation =
        (aquarium.player_origin.as_vec2() * (TILE_SIZE as f32)).extend(PLAYER_LAYER);
    // Reset expansion
    head.4.is_expanding = false;

    // Delete old bits and a tail
    if let Some(children) = head.1 {
        for child in children {
            commands.entity(*child).despawn();
        }
    }
    // Generating new bits and a tail
    commands.entity(head.0).with_children(|parent| {
        let body_length = aquarium.player_defaultbits.len();
        let bit_transform = Transform::from_xyz(0., 0., PLAYER_LAYER);
        let tail_transform = Transform::from_xyz(-(TILE_SIZE as f32), 0., PLAYER_LAYER);

        for (iter, bit) in aquarium.player_defaultbits.iter().enumerate() {
            parent
                .spawn((
                    player_image.index_to_sprite(1, 0),
                    bit_transform.clone(),
                    Body,
                    BitIter { pos: iter },
                    Player,
                ))
                .with_child((
                    boolean_image.y_to_sprite(0),
                    bit_transform.clone(),
                    BoxfishRegister {
                        boolean: *bit,
                        history: Vec::new(),
                    },
                    BitIter { pos: iter },
                    Player,
                ));
        }
        // Spawn a tail
        parent.spawn((
            player_image.index_to_sprite(0, 0),
            tail_transform,
            Body,
            BitIter { pos: body_length },
            Tail,
            Player,
        ));
    });

    // Clear movement history
    for mut head in head_query {
        head.4.history = Vec::new();
    }
}
