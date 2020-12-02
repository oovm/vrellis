mod draw_image;
mod draw_svg;
mod samples;
use image::ImageFormat;
use vrellis_core::{Vrellis, VrellisAlgorithm, VrellisShape};

#[test]
fn ready() {
    println!("it works!")
}



pub fn draw_circle(name:&str) {
    let mut ctx = Vrellis::default();
    ctx.inverted_color = false;
    ctx.convex_shape = VrellisShape::Circle;
    ctx.points = 200;
    ctx.algorithm = VrellisAlgorithm::AntiAliased;
    let mut state = ctx.render_path( format!("tests/{}.png",name)).unwrap();
    state.steps(1000);
    state.save_svg(format!("tests/draw_svg/{}-circle.svg",name)).unwrap();
    state.current_image.save_with_format(format!("tests/draw_image/{}-circle1.png",name), ImageFormat::Png).unwrap();
    state.current_composite_image.save_with_format(format!("tests/draw_image/{}-circle2.png",name), ImageFormat::Png).unwrap();
}

pub fn draw_square(name:&str) {
    let mut ctx = Vrellis::default();
    ctx.inverted_color = false;
    ctx.convex_shape = VrellisShape::Square;
    ctx.points = 200;
    ctx.algorithm = VrellisAlgorithm::AntiAliased;
    let mut state = ctx.render_path( format!("tests/{}.png",name)).unwrap();
    state.steps(1000);
    state.save_svg(format!("tests/draw_svg/{}-square.svg",name)).unwrap();
    state.current_image.save_with_format(format!("tests/draw_image/{}-square1.png",name), ImageFormat::Png).unwrap();
    state.current_composite_image.save_with_format(format!("tests/draw_image/{}-square2.png",name), ImageFormat::Png).unwrap();
}