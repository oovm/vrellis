use crate::VrellisCanvas;
use image::DynamicImage;
use itertools::Itertools;

impl VrellisCanvas {
    pub fn draw_svg(&self) -> String {
        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 {} {}">{}</svg>"#,
            self.current_image.width(),
            self.current_image.height(),
            format!(
                r#"<polyline points={:?} style="fill:none;stroke:black;stroke-width:1"/>"#,
                self.take_points(self.path.len())
            )
        )
    }
    pub fn draw_svg_steps(&self) -> Vec<String> {
        let mut out = Vec::with_capacity(self.path.len());
        for i in 1..=self.path.len() {
            out.push(format!(
                r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 {} {}">{}</svg>"#,
                self.current_image.width(),
                self.current_image.height(),
                format!(r#"<polyline points={:?} style="fill:none;stroke:black;stroke-width:1"/>"#, self.take_points(i))
            ))
        }
        return out;
    }

    fn take_points(&self, n: usize) -> String {
        self.path
            .iter()
            .take(n)
            .map(|i| unsafe { self.points.get_unchecked(*i as usize) })
            .map(|p| format!("{},{}", p.x, p.y))
            .collect_vec()
            .join(" ")
    }
}

impl VrellisCanvas {
    pub fn draw_image(&self) -> DynamicImage {
        unimplemented!()
    }
}

impl VrellisCanvas {
    pub fn draw_canvas(&self) {
        unimplemented!()
    }
}
