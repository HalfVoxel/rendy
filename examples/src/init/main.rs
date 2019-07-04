//!
//! Basic example initializes core type of the rendy - `Factory` and exits.
//!

#![cfg_attr(
    not(any(
        feature = "dx12",
        feature = "gl",
        feature = "metal",
        feature = "vulkan"
    )),
    allow(unused)
)]

use rendy::factory::{Config, Factory};

#[cfg(feature = "dx12")]
type Backend = rendy::dx12::Backend;

#[cfg(feature = "gl")]
type Backend = rendy::gl::Backend;

#[cfg(feature = "metal")]
type Backend = rendy::metal::Backend;

#[cfg(feature = "vulkan")]
type Backend = rendy::vulkan::Backend;

#[cfg(any(
    feature = "dx12",
    feature = "gl",
    feature = "metal",
    feature = "vulkan"
))]
fn main() {
    env_logger::Builder::from_default_env()
        .filter_module("init", log::LevelFilter::Trace)
        .init();

    let config: Config = Default::default();

    rendy_with_gl_backend! {
        let events_loop = rendy::gl::glutin::EventsLoop::new();
        rendy::gl::glutin::Context::new_headless(&events_loop, );
        let (factory, families): (Factory<Backend>, _) = rendy::factory::init_with_instance(rendy::gl::Headless, config).unwrap();    
    }

    let (factory, families): (Factory<Backend>, _) = rendy::factory::init(config).unwrap();
    drop(families);
    drop(factory);
}

#[cfg(not(any(
    feature = "dx12",
    feature = "gl",
    feature = "metal",
    feature = "vulkan"
)))]
fn main() {
    panic!("Specify feature: { dx12, gl, metal, vulkan }");
}
