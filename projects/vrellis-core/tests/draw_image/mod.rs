use image::ImageFormat;
use vrellis_core::{Vrellis, VrellisShape};

#[test]
fn draw() {
    let mut ctx = Vrellis::default();
    ctx.inverted_color = false;
    ctx.convex_shape = VrellisShape::Square;
    let mut state = ctx.render_path("tests/wolfram-wolf.png").unwrap();
    let mut out = None;
    for _ in 0..100 {
        match state.next() {
            None => break,
            Some(img) => out = Some(img),
        }
    }
    println!("{:?}", state);
    out.unwrap().save_with_format("tests/draw_image/wolfram-wolf.png", ImageFormat::Png).unwrap()
}
