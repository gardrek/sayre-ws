use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};

use sayre::vfc::Rgb;
use sayre::vfc::Vfc;
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

    let rgb_yel = Rgb(0xcc, 0xaa, 0x22);

    //~ let yellow = 0xcc * 0x10000 + 0xaa * 0x100 + 0x22;
    let yellow = rgb_yel.as_argb_u32();

    // Limit to max ~60 fps update rate
    //~ window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // Limit to max 64 fps cause why not
    window.limit_update_rate(Some(std::time::Duration::from_micros(16125)));

    //~ panic!();

    let mut old_pixel = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::F9, KeyRepeat::No) {
            vfc.bg_layers[0].hidden = !vfc.bg_layers[0].hidden;
        }

        vfc.render_frame();

        sayre::render_to_argb_u32(&mut vfc.framebuffer, &mut buffer);

        //~ let oam_0 = &mut vfc.oam[sayre::vfc::OamIndex(0)];

        //~ oam_0.x += 1;

        vfc.oam[sayre::vfc::OamIndex(0)].x += 1;

        vfc.bg_layers[0].x += 1;

        /*
        for (i, pixel) in buffer.iter_mut().enumerate() {
            let xi = i as usize % WIDTH;
            //~ let x = 0;
            let yi = i as usize / WIDTH;
            //~ let y = 0;

            let index = Vfc::get_fb_pixel_index(xi as u8, yi as u8);

            let rx = xi as u8;
            let ry = yi as u8;

            //~ let b = oam_0.bounding_box_contains_pixel(rx, ry);
            let b = false;
            let c = rx == oam_0.x || ry == oam_0.y || rx == oam_0.x + 7 || ry == oam_0.y + 7;

            *pixel = if b && c { yellow } else { vfc.framebuffer[index].as_argb_u32() };
            //~ let m = WIDTH.max(HEIGHT);
            //~ *pixel = ((x * 255 / m) ^ (y * 255 / m)) as u32;
        }
        */

        {
            let pixel = buffer.iter_mut().next().unwrap();
            *pixel = !(old_pixel) & 0xffffff;
            old_pixel = (*pixel).clone();
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
