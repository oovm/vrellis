use image::ImageFormat;
use vrellis_core::{Vrellis, VrellisAlgorithm, VrellisShape};

#[test]
fn draw() {
    let mut ctx = Vrellis::default();
    ctx.inverted_color = false;
    ctx.convex_shape = VrellisShape::Square;
    ctx.points = 200;
    ctx.algorithm = VrellisAlgorithm::AntiAliased;
    let mut state = ctx.render_path("tests/图层 1 拷贝.png").unwrap();
    state.steps(100);
    println!("{:?}", state);
    println!("{:?}", state.draw_svg_steps().len());
    state.save_svg("tests/draw_image/wolfram-wolf1.svg").unwrap();
    state.current_image.save_with_format("tests/draw_image/wolfram-wolf1.png", ImageFormat::Png).unwrap();
    state.current_composite_image.save_with_format("tests/draw_image/wolfram-wolf2.png", ImageFormat::Png).unwrap();
}
