use bevy::prelude::*; // Bevy 0.7
use mun_runtime::Runtime;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MunPlugin)
        .run();
}

struct MunPlugin;

// This timer will be used to reload mod.munlib
struct MunTimer(Timer);

// This timer will be used to define the print interval
struct PrintTimer(Timer);

impl Plugin for MunPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_mun_runtime.exclusive_system())
            // Set timer to track at 2 second intervals
            .insert_resource(MunTimer(Timer::from_seconds(1.0, true)))
            // Set timer to track at 1 second intervals
            .insert_resource(PrintTimer(Timer::from_seconds(1.0, true)))
            .add_system(reload_munlib)
            .add_system(print_from_mun);
    }
}

fn setup_mun_runtime(world: &mut World) {
    let builder = Runtime::builder("target/mod.munlib");
    let runtime: mun_runtime::Runtime =
        unsafe { builder.finish() }.expect("Failed to spawn Runtime");

    // Mun Runtime does-not implement the Send trait so it must be inserted
    // as a non-send Resource

    //world.insert_non_send(runtime); // Bevy 0.6
    world.insert_non_send_resource(runtime); // Bevy 0.7
}

fn print_from_mun(mun: NonSendMut<Runtime>, time: Res<Time>, mut timer: ResMut<PrintTimer>) {
    let result: usize = mun.invoke("mun_func", ()).unwrap();
    if timer.0.tick(time.delta()).just_finished() {
        println!("Printing value from `mun_func`: {}", result);
    }
}

fn reload_munlib(time: Res<Time>, mut timer: ResMut<MunTimer>, mut mun: NonSendMut<Runtime>) {
    if timer.0.tick(time.delta()).just_finished() {
        unsafe { mun.update() };
    }
}
