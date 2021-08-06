sixtyfps::include_modules!();

use std::panic::catch_unwind;
use std::fmt;

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

enum SupportedGame {
    SamuraiGunn2,
    RivalsOfAether,
    Unknown
}

impl SupportedGame {
    fn game_id(&self) -> Option<&str> {
        match self {
            SupportedGame::RivalsOfAether => Some("383980"),
            SupportedGame::SamuraiGunn2 => Some("1397790"),
            SupportedGame::Unknown => None,
        }
    }

    fn is_recognized(&self) -> bool {
        !matches!(self, SupportedGame::Unknown)
    }
}

impl fmt::Display for SupportedGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            SupportedGame::SamuraiGunn2 => "Samurai Gunn 2",
            SupportedGame::RivalsOfAether => "Rivals of Aether",
            SupportedGame::Unknown => "Unknown"
        })
    }
}

fn detected_game() -> SupportedGame {
    let current_dir = std::env::current_dir().unwrap();
    let folder_name = current_dir.file_name().unwrap().to_string_lossy();
    match &*folder_name {
        "Samurai Gunn 2" => SupportedGame::SamuraiGunn2,
        "Rivals of Aether" => SupportedGame::RivalsOfAether,
        _ => SupportedGame::Unknown,
    }
}

fn main() {
    let window = ModdingToolWindow::new();

    window.set_title_text(format!("GameMaker 2 Modding Tool {}", env!("CARGO_PKG_VERSION")).into());

    let game = detected_game();
    window.set_game_detected(format!("Game Detected: {}", game).into());

    window.set_allow_install(game.is_recognized());

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

    window.on_uninstall(move || {
        if let Some(game_id) = game.game_id() {
            open::that_in_background(format!("steam://validate/{}", game_id));
        } else {
            println!("The current game is unrecognized and thus cannot uninstall mods.");
        }
    });

    window.run();
}
