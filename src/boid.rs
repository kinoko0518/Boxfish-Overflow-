use bevy::{math::VectorSpace, prelude::*};
use rand::Rng;
use std::f32::consts::PI;

const BOID_TEXTURE: &str = "embedded://boid/fish.png";
const DISTANT: f32 = 1100.0;

#[derive(Resource)]
pub struct BoidConfig {
    pub count: i32,
    // Range
    pub cohesion_radius: f32,
    pub separation_radius: f32,
    pub alignment_radius: f32,
    // Coefficients
    pub cohesion_coefficient: f32,
    pub separation_coefficient: f32,
    pub alignment_coefficient: f32,
    pub homing_coefficient: f32,
    // Behavior
    pub vision_angle: f32, // Radian
}

impl Default for BoidConfig {
    fn default() -> Self {
        Self {
            count: 150,
            cohesion_radius: 80.0,
            separation_radius: 40.0,
            alignment_radius: 80.0,
            cohesion_coefficient: 0.1,
            separation_coefficient: 45.0,
            alignment_coefficient: 25.0,
            homing_coefficient: 0.001,
            vision_angle: PI / 2.0,
        }
    }
}

#[derive(Component)]
pub struct Boid {
    pub speed: f32,
}

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoidConfig::default())
            .add_systems(Startup, boid_setup)
            .add_systems(Update, boid_system);
    }
}

pub fn boid_setup(mut commands: Commands, asset_server: Res<AssetServer>, config: Res<BoidConfig>) {
    let mut rng = rand::rng();

    for _ in 0..config.count {
        let x = rng.random_range(-300.0..300.0);
        let y = rng.random_range(-300.0..300.0);

        commands.spawn((
            Sprite {
                image: asset_server.load(BOID_TEXTURE),
                ..default()
            },
            Transform::from_xyz(x, y, 0.),
            Boid {
                speed: rng.random_range(20.0..70.0),
            },
        ));
    }
}

pub fn boid_system(
    mut boid_query: Query<(Entity, &mut Transform, &Boid)>,
    config: Res<BoidConfig>,
    time: Res<Time>,
) {
    let all_boids: Vec<(Entity, Vec2, Vec2)> = boid_query
        .iter()
        .map(|(entity, transform, _)| {
            let pos = transform.translation.xy();
            let dir = (transform.rotation * Vec3::Y).xy();
            (entity, pos, dir)
        })
        .collect();

    for (entity, mut transform, boid) in boid_query.iter_mut() {
        let own_pos = transform.translation.xy();
        let own_dir = (transform.rotation * Vec3::Y).xy();

        let neighbors: Vec<(Vec2, Vec2)> = all_boids
            .iter()
            .filter(|(e, _, _)| *e != entity)
            .map(|(_, pos, dir)| (*pos, *dir))
            .collect();

        if neighbors.is_empty() {
            continue;
        }

        let cohesion_vec = calculate_cohesion_vec(own_pos, &neighbors, config.cohesion_radius);
        let separation_vec =
            calculate_separation_vec(own_pos, &neighbors, config.separation_radius);
        let alignment_vec = calculate_alignment_vec(
            own_dir,
            &neighbors,
            own_pos,
            config.alignment_radius,
            config.vision_angle,
        );
        let homing = own_pos.distance(Vec2::ZERO) / DISTANT * config.homing_coefficient;

        let desired_direction = config.cohesion_coefficient * cohesion_vec
            + config.separation_coefficient * separation_vec
            + config.alignment_coefficient * alignment_vec
            + homing * (-transform.translation.xy());

        if desired_direction.length_squared() > 0.0001 {
            let normalized_dir = desired_direction.normalize();

            let new_pos = own_pos + normalized_dir * boid.speed * time.delta_secs();
            transform.translation.x = new_pos.x;
            transform.translation.y = new_pos.y;

            let angle = normalized_dir.y.atan2(normalized_dir.x);
            transform.rotation = Quat::from_rotation_z(angle - PI / 2.0);
        } else {
            let new_pos = own_pos + own_dir * boid.speed * time.delta_secs();
            transform.translation.x = new_pos.x;
            transform.translation.y = new_pos.y;
        }
    }
}

fn calculate_cohesion_vec(own_pos: Vec2, neighbors: &[(Vec2, Vec2)], radius: f32) -> Vec2 {
    let mut center = Vec2::ZERO;
    let mut count = 0;

    for (pos, _) in neighbors {
        if own_pos.distance_squared(*pos) < radius * radius {
            center += *pos;
            count += 1;
        }
    }

    if count > 0 {
        (center / count as f32) - own_pos
    } else {
        Vec2::ZERO
    }
}

fn calculate_separation_vec(own_pos: Vec2, neighbors: &[(Vec2, Vec2)], radius: f32) -> Vec2 {
    let mut force = Vec2::ZERO;

    for (pos, _) in neighbors {
        let distance_sq = own_pos.distance_squared(*pos);
        if distance_sq > 0.0 && distance_sq < radius * radius {
            force += (own_pos - *pos) / distance_sq;
        }
    }
    force
}

fn calculate_alignment_vec(
    own_dir: Vec2,
    neighbors: &[(Vec2, Vec2)],
    own_pos: Vec2,
    radius: f32,
    vision_angle: f32,
) -> Vec2 {
    let mut avg_dir = Vec2::ZERO;
    let mut count = 0;

    for (pos, dir) in neighbors {
        if own_pos.distance_squared(*pos) < radius * radius {
            let to_boid = (*pos - own_pos).normalize_or_zero();
            if own_dir.dot(to_boid) > vision_angle.cos() {
                avg_dir += *dir;
                count += 1;
            }
        }
    }

    if count > 0 {
        (avg_dir / count as f32).normalize_or_zero()
    } else {
        Vec2::ZERO
    }
}
