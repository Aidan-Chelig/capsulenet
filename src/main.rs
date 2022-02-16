use std::{collections::HashMap, net::IpAddr, net::Ipv4Addr, net::SocketAddr, time::Duration};
use bevy::{app::ScheduleRunnerSettings, prelude::*, render::RenderPlugin};
use bevy_networking_turbulence::{NetworkEvent, NetworkResource, NetworkingPlugin};

fn main() {
    // this app loops forever at 60 fps
    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        .add_plugin(NetworkingPlugin::default())
        .add_startup_system(setup_server)
        .add_system(counter)
        .run();
}

fn counter(mut state: Local<CounterState>) {
    if state.count % 60 == 0 {
        println!("{}", state.count);
    }
    state.count += 1;
}

#[derive(Default)]
struct CounterState {
    count: u32,
}

fn setup_server(mut net: ResMut<NetworkResource>) {
    let ip_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port = 9001;
    let socket_address = SocketAddr::new(ip_address, 9001);
    net.listen(socket_address, None, None);
    println!("Listening on: {ip_address}:{port}");
}
