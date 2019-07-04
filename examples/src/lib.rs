// #![cfg_attr(
//     not(any(
//         feature = "dx12",
//         feature = "gl",
//         feature = "metal",
//         feature = "vulkan"
//     )),
//     allow(unused)
// )]

use rendy::{
    command::Families,
    factory::{Config, Factory},
    graph::Graph,
    hal::{self, format::AsFormat as _},
    wsi::Surface,
    util::*,
};

rendy_wasm32! {
    pub use hal::pso::ShaderStageFlags;
    pub use wasm_bindgen::prelude::*;
}

#[cfg(not(all(feature = "gl", target_arch = "wasm32")))]
pub use rendy::{
    shader::{ShaderKind, SourceLanguage, SourceShaderInfo},
    wsi::winit::{EventsLoop, WindowBuilder},
};

#[cfg(feature = "spirv-reflection")]
pub use rendy::shader::SpirvReflection;

#[cfg(not(feature = "spirv-reflection"))]
pub use rendy::mesh::AsVertex;

#[cfg(not(any(
    feature = "dx12",
    feature = "gl",
    feature = "metal",
    feature = "vulkan"
)))]
pub type Backend = rendy::empty::Backend;

#[cfg(feature = "dx12")]
pub type Backend = rendy::dx12::Backend;

#[cfg(feature = "gl")]
pub type Backend = rendy::gl::Backend;

#[cfg(feature = "metal")]
pub type Backend = rendy::metal::Backend;

#[cfg(feature = "vulkan")]
pub type Backend = rendy::vulkan::Backend;

#[cfg(any(
    feature = "dx12",
    feature = "gl",
    feature = "metal",
    feature = "vulkan"
))]
pub fn run<I, U, T>(init: I)
where
    I: FnOnce(&mut Factory<Backend>, &mut Families<Backend>, Surface<Backend>) -> (Graph<Backend, T>, T, U),
    U: FnMut(&mut Factory<Backend>, &mut Families<Backend>, &mut T) -> bool,
{
    rendy_not_wasm32! {
        env_logger::Builder::from_default_env()
            .filter_module("triangle", log::LevelFilter::Trace)
            .init();
    }

    rendy_wasm32! {
        console_log::init_with_level(log::Level::Trace);
    }

    let config: Config = Default::default();

    rendy_not_wasm32! {
        let window_builder = WindowBuilder::new().with_title("Rendy example");
        let mut events_loop = EventsLoop::new();
    }

    rendy_without_gl_backend!{
        let window = window_builder.build(&events_loop).unwrap();
        let (mut factory, mut families): (Factory<Backend>, _) = rendy::factory::init(config).unwrap();
        let surface = factory.create_surface(&window);
    }

    rendy_with_gl_backend!{
        rendy_not_wasm32! {
            let windowed_context = unsafe {
                let builder = rendy::gl::config_context(
                    rendy::gl::glutin::ContextBuilder::new(),
                    hal::format::Rgba8Srgb::SELF,
                    None,
                )
                .with_vsync(true);
                builder.build_windowed(window_builder, &events_loop)
                    .unwrap().make_current().unwrap()
            };
        }
        rendy_wasm32! {
            let window = { rendy::gl::Window };
        }
    }

    rendy_with_gl_backend!{
        rendy_wasm32! {
            let surface = rendy::gl::Surface::from_window(window);
        }

        rendy_not_wasm32! {
            let surface = rendy::gl::Surface::from_window(windowed_context);
        }
        let (mut factory, mut families) =
            rendy::factory::init_with_instance(surface.clone(), config).unwrap();
        let surface = unsafe { factory.wrap_surface(surface) };
    }

    let (mut graph, mut aux, mut update) = init(&mut factory, &mut families, surface);

    rendy_wasm32! {
        let mut frames = 0u64..1;
    }

    rendy_not_wasm32! {
        let mut frames = 0u64..;
        let started = std::time::Instant::now();
        let mut elapsed = started.elapsed();

        // kill switch
        // std::thread::spawn(move || {
        //     while started.elapsed() < std::time::Duration::new(60, 0) {
        //         std::thread::sleep(std::time::Duration::new(1, 0));
        //     }

        //     std::process::abort();
        // });
        
    }

    for _ in &mut frames {
        if !update(&mut factory, &mut families, &mut aux) {
            break;
        }

        factory.maintain(&mut families);
        graph.run(&mut factory, &mut families, &aux);

        rendy_not_wasm32!{
            events_loop.poll_events(|_| ());

            elapsed = started.elapsed();
            if elapsed >= std::time::Duration::new(5, 0) { break; }
        }
    }

    graph.dispose(&mut factory, &mut aux);

    rendy_not_wasm32!{
        let elapsed_ns = elapsed.as_secs() * 1_000_000_000 + elapsed.subsec_nanos() as u64;

        log::info!(
            "Elapsed: {:?}. Frames: {}. FPS: {}",
            elapsed,
            frames.start,
            frames.start * 1_000_000_000 / elapsed_ns
        );
    }
}

#[cfg(not(any(
    feature = "dx12",
    feature = "gl",
    feature = "metal",
    feature = "vulkan"
)))]
pub fn run<T>(_: impl FnOnce(&mut Factory<Backend>, &mut Families<Backend>, Surface<Backend>) -> T) {
    panic!("Specify graphics backend via feature: { dx12, gl, metal, vulkan }");
}