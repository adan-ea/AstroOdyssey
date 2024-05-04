use bevy::prelude::*;

use crate::sim::{
    base::events::MinerSpawnEvent,
    droids::{
        components::{DroidState, Robot},
        generate_random_nearby_position, random_name,
    },
    resources::iron::components::Iron
};


use super::{
    components::{Miner, MinerAction, MinerParent},
    MINER_ENERGY, MINER_INVENTORY_CAPACITY, MINER_IRON_COST, MINER_SPEED, MINER_SPRITE_PATH,
};

const NAME: [&str; 17] = [
    "Milo", "Miles", "Morgan", "Max", "Mason", "Matthew", "Michael", "Mark", "Maverick", "Maddox",
    "Malachi", "Mateo", "Martin", "Marshall", "Mario", "Maurice", "Mauricio",
];

pub fn spawn_miner_parent(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        MinerParent,
        Name::new("Miners"),
    ));
}

pub fn spawn_miner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    parent: Query<Entity, With<MinerParent>>,
    mut miner_spawn_er: EventReader<MinerSpawnEvent>,
) {
    let parent = parent.single();
    for miner_spawn in miner_spawn_er.read() {
        let spawn_pos = generate_random_nearby_position(miner_spawn.spawn_pos);

        commands.entity(parent).with_children(|commands| {
            let name = random_name(NAME);
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 10.0),
                    texture: asset_server.load(MINER_SPRITE_PATH),
                    ..default()
                },
                Miner {
                    inventory_capacity: MINER_INVENTORY_CAPACITY,
                    miner_action: MinerAction::Null,
                },
                Robot {
                    name: name.clone(),
                    energy: MINER_ENERGY,
                    speed: MINER_SPEED,
                    iron_cost: MINER_IRON_COST,
                    destination: Vec2::new(spawn_pos.x, spawn_pos.y),
                    droid_state: DroidState::Idle,
                },
                Name::new(name),
            ));
        });
    }
}

pub fn miner_behavior_system(
    mut query: Query<(&mut Miner, &mut Robot, &Transform)>,
    //resource_query: Query<(&dyn Resource, &Transform), With<Iron>>,
) {
    for (mut miner, mut robot, miner_transform) in query.iter_mut() {
        match miner.miner_action {
            MinerAction::Mine => {
                // Mine
                //robot.destination = resource_transform.translation.truncate();
                robot.droid_state = DroidState::Working;
                miner.inventory_capacity += 1; // add iron to inventory
                // Fin the more nearby ressource
                /*if let Some((_, resource_transform)) = resource_query.iter()
                    .filter(|(_, transform)| transform.translation.distance(miner_transform.translation) < 50.0)
                    .min_by_key(|(_, transform)| transform.translation.distance(miner_transform.translation) as u32)
                {
                    // Mine
                    robot.destination = resource_transform.translation.truncate();
                    robot.droid_state = DroidState::Working;
                    miner.inventory_capacity += 1; // add iron to inventory
                } else {
                    miner.miner_action = MinerAction::DropOff;
                }*/
            },
            MinerAction::DropOff => {
                let base_position = Vec2::new(0.0, 0.0);
                robot.destination = base_position;
                if robot.destination == miner_transform.translation.truncate() {
                    // drop off iron
                    miner.inventory_capacity = 0;
                    robot.droid_state = DroidState::Idle;
                    miner.miner_action = MinerAction::Mine;
                }
            },
            _ => {},
        }
    }
}
