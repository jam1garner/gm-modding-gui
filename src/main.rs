sixtyfps::include_modules!();

use std::panic::catch_unwind;
use std::path::Path;
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
    Unknown(String),
}

impl SupportedGame {
    fn game_id(&self) -> Option<&str> {
        match self {
            SupportedGame::RivalsOfAether => Some("383980"),
            SupportedGame::SamuraiGunn2 => Some("1397790"),
            _ => None,
        }
    }

    fn is_recognized_steam(&self) -> bool {
        !matches!(self, SupportedGame::Unknown(_))
    }
}

impl fmt::Display for SupportedGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let temp;
        write!(f, "{}", match self {
            SupportedGame::SamuraiGunn2 => "Samurai Gunn 2",
            SupportedGame::RivalsOfAether => "Rivals of Aether",
            SupportedGame::Unknown(name) => { temp = format!("{} (Game ID Unknown)", name); &temp },
        })
    }
}

enum DetectGameError {
    DataWinFailRead,
    DataWinFailParse,
    NoDataWin,
}

impl fmt::Display for DetectGameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "None ({})", match self {
            DetectGameError::DataWinFailParse => "failed to parse data.win",
            DetectGameError::DataWinFailRead => "data.win could not be read",
            DetectGameError::NoDataWin => "no data.win",
        })
    }
}

fn detected_game() -> Result<SupportedGame, DetectGameError> {
    let game_name = if Path::new("data.win").exists() {
        if let Ok(data_win) = std::fs::read("data.win") {
            let data_win_file = gm_data_win::take_data_win_file(&data_win);

            macro_rules! get_section {
                ($section_kind:ident) => {
                    data_win_file
                        .iter()
                        .filter_map(|section| {
                            if let gm_data_win::file_structs::Section::$section_kind(section) = section {
                                Some(section)
                            } else {
                                None
                            }
                        })
                        .next()
                        .ok_or(DetectGameError::DataWinFailParse)
                }
            }
            let gen8 = get_section!(Gen8)?;
            let strg = get_section!(Strg)?;

            strg.get(gen8.game_name_offset)
                .ok_or(DetectGameError::DataWinFailParse)
                .map(|name| name.clone())
        } else {
            Err(DetectGameError::DataWinFailRead)
        }
    } else {
        Err(DetectGameError::NoDataWin)
    }?;

    Ok(match &*game_name {
        "Samurai Gunn 2" => SupportedGame::SamuraiGunn2,
        "Rivals of Aether" => SupportedGame::RivalsOfAether,
        _ => SupportedGame::Unknown(game_name),
    })
}

fn main() {
    let window = ModdingToolWindow::new();

    window.set_title_text(format!("GameMaker 2 Modding Tool {}", env!("CARGO_PKG_VERSION")).into());

    let game = detected_game();
    window.set_game_detected(
        format!(
            "Detected Game: {}",
            game.as_ref().map(ToString::to_string).unwrap_or_else(ToString::to_string)
        ).into()
    );

    window.set_allow_install(game.as_ref().map(SupportedGame::is_recognized_steam).unwrap_or(false));

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
        if let Some(game_id) = game.as_ref().ok().map(SupportedGame::game_id).flatten() {
            open::that_in_background(format!("steam://validate/{}", game_id));
        } else {
            println!("The current game is unrecognized and thus cannot uninstall mods.");
        }
    });

    window.run();
}
