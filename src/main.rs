sixtyfps::include_modules!();

fn main() {
    let window = ModdingToolWindow::new();

    window.on_extract(|| {
        println!("Extracting...");
        gm_data_win::main(gm_data_win::Args {
            extract_audio: true,
            extract_sprites: true,
            extract_fonts: true,
            extract_textures: true,
            .. Default::default()
        }, false);
        println!("Finished Extracting!");
    });

    window.on_inject(|| {
        println!("Injecting...");
        gm_data_win::main(gm_data_win::Args {
            mod_audio: true,
            mod_sprites: true,
            mod_textures: true,
            .. Default::default()
        }, false);
        println!("Finished Injecting!");
    });

    window.run();
}
