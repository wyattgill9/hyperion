use flecs_ecs::core::World;
use hyperion_clap::{MinecraftCommand, hyperion_command::CommandRegistry};

use crate::command::{
    class::ClassCommand, fly::FlyCommand, raycast::RaycastCommand, replace::ReplaceCommand,
    speed::SpeedCommand, xp::XpCommand,
};

mod class;
mod fly;
mod raycast;
mod replace;
mod speed;
mod xp;

pub fn register(registry: &mut CommandRegistry, world: &World) {
    SpeedCommand::register(registry, world);
    FlyCommand::register(registry, world);
    ClassCommand::register(registry, world);
    XpCommand::register(registry, world);
    ReplaceCommand::register(registry, world);
    RaycastCommand::register(registry, world);
}
