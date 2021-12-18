use gfx::{
    format::{Depth, Srgba8},
    Device,
};
use gfx_glyph::{ab_glyph::*, *};
use glutin::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};
use old_school_gfx_glutin_ext::*;
use std::env;
use glyph_brush::ab_glyph::{FontRef, point, PxScale, Rect};
use glyph_brush::{Extra, FontId};

/// creates a window and centers the cat fact with the cat image
pub fn cat_fax_window(fact: &str, num: usize) {

    // linux stuff
    if cfg!(target_os = "linux") {
        // winit wayland is currently still wip
        if env::var("WINIT_UNIX_BACKEND").is_err() {
            env::set_var("WINIT_UNIX_BACKEND", "x11");
        }
        // disables vsync sometimes on x11
        if env::var("vblank_mode").is_err() {
            env::set_var("vblank_mode", "0");
        }
    }

    // the font and calculating the width of the window

    // px_scale is the scale of the font for calculations
    let px_scale = PxScale::from(50.0);
    // the font from a file
    let font: &[u8] = include_bytes!("../assets/fonts/Roboto-Regular.ttf");
    let font = FontRef::try_from_slice(font).expect("failed to build font");
    // scaled_font for calculations of size
    let scaled_font = font.as_scaled(px_scale);
    // the total room to be allocated bordering the longer display string
    //   (half of this number is the space between the left and right borders)
    let padding = 24;
    // the width of the fact string + the padding
    //   this expects that this will be the longer string, but can be changed by later code.
    let mut width = padding;
    // the padding of the fact string, can be changed by following code
    let mut fact_padding = padding as f32 / 2.0;
    // loop through the chars and get the advance of the glyph to add to the string's length
    for c in fact.chars() {
        let advance = scaled_font.h_advance(font.glyph_id(c)).round() as u32;
        width += advance;
    }
    // the height of each character
    let font_height = scaled_font.ascent() + scaled_font.descent();

    // the title - "Cat Fax #..."
    let num_txt = format!("Cat Fax #{}", num);
    // the width of the number string
    let mut num_width = 0;
    // loop through the chars of the number string to get it's width using the same method as before
    for c in num_txt.chars() {
        let advance = scaled_font.h_advance(font.glyph_id(c)).round() as u32;
        num_width += advance;
    }

    // the padding added to the left and right if it is the longer string
    let mut num_padding = ((width as f32) - (num_width as f32)) / 2.0;

    // if the number string is longer than the fact string
    if num_width > (width - padding) {
        // set the padding for the fact to center the text
        fact_padding = ((num_width as f32) - (width as f32)) / 2.0;
        // add padding to the number text to make sure its not pressed up against the sides
        num_padding = padding as f32 / 2.0;
        // set the window width to the required size
        width = num_width + padding;
    }

    // creating the window

    // create the event loop
    let event_loop = glutin::event_loop::EventLoop::new();
    // the background color
    let clear_color = [(255.0 / 255.0), (224.0 / 255.0), (90.0 / 255.0), 1.0];
    // build the window with set settings
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("Cat Fax") // the title of the window
        .with_resizable(false) // set the window to not be resizable
        .with_always_on_top(true) // make sure the window stays on top
        .with_decorations(true) // the top bar with close button
        .with_inner_size(glutin::dpi::PhysicalSize::new(width, 200)); // the size of the window

    // Create the context and related variables
    let (window_ctx, mut device,
        mut factory, mut main_color,
        mut main_depth) =
        glutin::ContextBuilder::new()
            .with_gfx_color_depth::<Srgba8, Depth>()
            .build_windowed(window_builder, &event_loop).expect("failed to build context")
            .init_gfx::<Srgba8, Depth>();

    // create the brush for displaying text
    let mut glyph_brush = gfx_glyph::GlyphBrushBuilder::using_font(font.clone())
        .initial_cache_size((1024, 1024))
        .build(factory.clone());

    // create the encoder
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    // [removed] for displaying FPS in the title
    //let mut loop_helper = spin_sleep::LoopHelper::builder().build_with_target_rate(250.0);

    // calculating bounds of the window
    let (width, height, ..) = main_color.get_dimensions();
    let (width, height) = (f32::from(width), f32::from(height));

    // define the text color
    let color = [0.1, 0.1, 0.1, 1.0];

    // creating the fact text to display
    let fact_text: Vec<_> = gfx_glyph::Layout::default().calculate_glyphs(
        // the fonts to use
        &[font.clone()],
        // the geometry of the window and its position
        &gfx_glyph::SectionGeometry {
            // the position of the text in the window
            screen_position: (fact_padding, (height * 0.65) - (font_height / 2.0)),
            // the bounds of the window
            bounds: (width, height),
        },
        &[gfx_glyph::SectionText {
            // the text to display
            text: fact,
            // the scale of the text
            scale: px_scale,
            // the font to use from the fonts
            font_id: FontId(0),
        }],
    );

    let number_text: Vec<_> = gfx_glyph::Layout::default().calculate_glyphs(
        // the fonts to use
        &[font],
        // the geometry of the window and its position
        &gfx_glyph::SectionGeometry {
            // the position of the text in the window
            screen_position: (num_padding, (height * 0.25) - font_height / 2.0),
            // the bounds of the window
            bounds: (width, height),
        },
        &[gfx_glyph::SectionText {
            // the text to display
            text: num_txt.as_str(),
            // the scale of the text
            scale: px_scale,
            // the id of the font to use
            font_id: FontId(0),
        }],
    );

    // running the event loop for the window to function
    event_loop.run(move |event, _, control_flow| {
        // set the control_flow to Poll
        *control_flow = ControlFlow::Poll;

        // handle the event
        match event {
            Event::WindowEvent { event, .. } => match event {
                // add the escape key as a way to exit the program
                WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                }
                // or if the close button is pressed, exit as well
                | WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                // if the window is resized (somehow), update as needed.
                // this is here just in case I decide to re-implement window resizing.
                WindowEvent::Resized(size) => {
                    window_ctx.resize(size);
                    window_ctx.update_gfx(&mut main_color, &mut main_depth);
                }
                _ => (),
            },
            // if there are no more events to handle for the time being
            Event::MainEventsCleared => {
                // clear the screen with the background color
                encoder.clear(&main_color, clear_color);

                // queue the fact text
                glyph_brush.queue_pre_positioned(
                    // the text
                    fact_text.clone(),
                    // the color
                    vec![Extra { color, z: 0.0 }],
                    // bounds of the text
                    Rect {
                        min: point(0.0, 0.0),
                        max: point(width, height),
                    },
                );

                // queue the number text
                glyph_brush.queue_pre_positioned(
                    // the text
                    number_text.clone(),
                    // the color
                    vec![Extra { color, z: 0.0 }],
                    // the bounds of the text
                    Rect {
                        min: point(0.0, 0.0),
                        max: point(width, height),
                    },
                );

                // use the brush to draw the text
                glyph_brush
                    .use_queue()
                    .draw(&mut encoder, &main_color)
                    .unwrap();

                // switch buffers and clean up
                encoder.flush(&mut device);
                window_ctx.swap_buffers().unwrap();
                device.cleanup();

                // [removed] display the framerate in the title and refresh the loop_helper
                // if let Some(rate) = loop_helper.report_rate() {
                //     window_ctx
                //         .window()
                //         .set_title(&format!("{} - {:.0} FPS", title, rate));
                // }

                //loop_helper.loop_sleep();
                //loop_helper.loop_start();
            }
            _ => (),
        }
    });
}