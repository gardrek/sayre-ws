use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};

use sayre::vfc::Rgb;
use sayre::vfc::SCREEN_HEIGHT as HEIGHT;
use sayre::vfc::SCREEN_WIDTH as WIDTH;

fn main() {
    let mut vfc = sayre::main();

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X4,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let rgb_yel = Rgb::new(0xcc, 0xaa, 0x22);

    //~ let yellow = 0xcc * 0x10000 + 0xaa * 0x100 + 0x22;
    let yellow = rgb_yel.as_argb_u32();

    // Limit to max ~60 fps update rate
    //~ window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // Limit to max 64 fps cause why not
    window.limit_update_rate(Some(std::time::Duration::from_micros(16125)));
    //~ window.limit_update_rate(Some(std::time::Duration::from_micros(80625)));

    // Limit to max ~640 fps cause why not
    window.limit_update_rate(Some(std::time::Duration::from_micros(1612)));

    //~ panic!();

    let mut old_pixel = yellow;

    let mut start_time;

    let mut frametime_hist = std::collections::VecDeque::from(vec![]);

    for _ in 0..5 {
        frametime_hist.push_back(0);
    }


    while window.is_open() && !window.is_key_down(Key::Escape) {
        start_time = std::time::Instant::now();

        if window.is_key_pressed(Key::F9, KeyRepeat::No) {
            vfc.bg_layers[0].hidden = !vfc.bg_layers[0].hidden;
        }

        vfc.render_frame();

        sayre::render_to_argb_u32(&mut vfc.framebuffer, &mut buffer);

        //~ let oam_0 = &mut vfc.oam[sayre::vfc::OamIndex(0)];

        //~ oam_0.x += 1;

        {
            let obj = &mut vfc.oam[sayre::vfc::OamIndex(0)];
            obj.y = obj.y.wrapping_add(1)
        }

        {
            let layer = &mut vfc.bg_layers[0];
            layer.x = layer.x.wrapping_add(1)
        }

        {
            let pixel = buffer.iter_mut().next().unwrap();
            *pixel = !(old_pixel) & 0xffffff;
            old_pixel = (*pixel).clone();
        }

        // We unwrap here as we want this code to exit if it fails.
        // Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        let end_time = std::time::Instant::now();

        //~ let instant_fps = 1_000 / (end_time - start_time).as_millis();

        let instant_microsecs_per_frame = (end_time - start_time).as_micros();

        frametime_hist.pop_front().expect("fifo wrong length");

        frametime_hist.push_back(instant_microsecs_per_frame);

        let mut average = 0;
        for i in 0..5 {
            average += frametime_hist[i];
        }
        average /= 5;

        eprintln!("{instant_microsecs_per_frame:?}\t{average:?}");
        //~ eprintln!("{instant_fps:?}");
        //~ eprintln!("{instant_fps:?}\t {:?}");
    }
}
