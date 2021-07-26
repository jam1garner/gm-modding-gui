sixtyfps::include_modules!();

use std::panic::catch_unwind;

fn main() {
    let window = ModdingToolWindow::new();

    window.on_extract(|| {
        println!("Extracting...");
        let _ = catch_unwind(|| {
            gm_data_win::main(gm_data_win::Args {
                extract_audio: true,
                extract_sprites: true,
                extract_fonts: true,
                extract_textures: true,
                .. Default::default()
            }, false);

            println!("Finished Extracting!");
        });
    });

    window.on_inject(|| {
        println!("Injecting...");
        let _ = catch_unwind(|| {
            println!("Injecting sprites...");
            gm_data_win::main(gm_data_win::Args {
                mod_sprites: true,
                mod_textures: true,
                .. Default::default()
            }, false);

            println!("Injecting audio...");
            gm_data_win::main(gm_data_win::Args {
                mod_audio: true,
                .. Default::default()
            }, false);

            println!("Finished Injecting!");
        });
    });

    window.run();
}
