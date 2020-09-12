use mosaic_craft_core::{repack_all_theme, MosaicCraft};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let ctx = MosaicCraft::default();
    let out = ctx.render_path("tests/wolfram-wolf.png").unwrap();
    out.save_image("tests/wolfram-wolf-out.png").unwrap()
}

#[ignore]
#[test]
fn pack_all_theme() {
    match repack_all_theme("../mosaic-craft-themes", "../mosaic-craft-themes") {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}
