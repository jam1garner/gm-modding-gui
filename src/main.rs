sixtyfps::include_modules!();

use std::panic::catch_unwind;

struct Settings {
    audio: bool,
    sprites: bool,
    fonts: bool,
    textures: bool,
}

fn get_settings(window: &ModdingToolWindow) -> Settings {
    Settings {
        audio: window.get_audio(),
        sprites: window.get_sprites(),
        fonts: window.get_fonts(),
        textures: window.get_textures(),
    }
}

fn main() {
    let window = ModdingToolWindow::new();

    window.set_title_text(format!("GameMaker 2 Modding Tool {}", env!("CARGO_PKG_VERSION")).into());

    let win = window.clone_strong();
    window.on_extract(move || {
        println!("Extracting...");
        let Settings { audio, sprites, fonts, textures } = get_settings(&win);
        let _ = catch_unwind(|| {
            gm_data_win::main(gm_data_win::Args {
                extract_audio: audio,
                extract_sprites: sprites,
                extract_fonts: fonts,
                extract_textures: textures,
                .. Default::default()
            }, false);

            println!("Finished Extracting!");
        });
    });

    let win = window.clone_strong();
    window.on_inject(move || {
        println!("Injecting...");
        let Settings { audio, sprites, textures, fonts: _ } = get_settings(&win);
        let _ = catch_unwind(|| {
            if sprites | textures {
                println!("Injecting sprites...");
                gm_data_win::main(gm_data_win::Args {
                    mod_sprites: sprites,
                    mod_textures: textures,
                    .. Default::default()
                }, false);
            }

            if audio {
                println!("Injecting audio...");
                gm_data_win::main(gm_data_win::Args {
                    mod_audio: audio,
                    .. Default::default()
                }, false);
            }

            println!("Finished Injecting!");
        });
    });

    window.run();
}
