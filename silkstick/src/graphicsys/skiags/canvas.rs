use skia_safe::Canvas;
use crate::graphicsys::{ GrahicsCanvas, Paint };

pub struct SkiaCanvas<'a> {
    canvas: &'a Canvas,
    current_paint: Option<SkiaPaint>,
}

struct SkiaPaint {
    skia_paint: skia_safe::Paint,
}

impl SkiaPaint {
    fn new() -> Self {
        SkiaPaint {
            skia_paint: skia_safe::Paint::default(),
        }
    }
}

impl Paint for SkiaPaint {
    fn set_color(&mut self, color: crate::graphicsys::color::Color) {
        let rgba8 = color.to_rgba8();
        let skia_color = skia_safe::Color::from_argb(rgba8.3, rgba8.0, rgba8.1, rgba8.2);
        self.skia_paint.set_color(skia_color);
    }
}

impl <'a> GrahicsCanvas<'a> for SkiaCanvas<'a> {
    fn alter_paint(&mut self, alter_fn: fn(&mut dyn Paint) -> ()) {
        if let Some(ref mut current_paint) = self.current_paint {
            let paint_ref: &mut dyn Paint = current_paint;
            alter_fn(paint_ref);
        } else {
            panic!("Attempt to recursively alter paint!");
        }
    }

    fn enter_section(&mut self, pos: (f32, f32), size: (f32, f32), clip: bool, section_func: fn(&mut dyn GrahicsCanvas)) {
        let new_paint = SkiaPaint::new();
        let old_paint = std::mem::replace(&mut self.current_paint, Some(new_paint));
        self.canvas.save();
        if clip {
            self.canvas.clip_rect(
                skia_safe::Rect::from_xywh(pos.0, pos.1, size.0, size.1),
                skia_safe::ClipOp::Intersect,
                true);
        }
        self.canvas.translate((pos.0, pos.1));
        section_func(self);
        self.canvas.restore();
        self.current_paint = old_paint;
    }

    fn draw_text(&mut self, _pos: (f32, f32), _text: &str) {
        todo!()
    }

    fn draw_rect(&mut self, pos: (f32, f32), size: (f32, f32)) {
        let rect = skia_safe::Rect::from_xywh(pos.0, pos.1, size.0, size.1);
        if let Some(ref mut current_paint) = self.current_paint {
            self.canvas.draw_rect(rect, &current_paint.skia_paint);
        } else {
            panic!("Attempt to use paint while altering it!");
        }
    }

    fn draw_elipse(&mut self, pos: (f32, f32), size: (f32, f32)) {
        let rect = skia_safe::Rect::from_xywh(pos.0, pos.1, size.0, size.1);
        if let Some(ref mut current_paint) = self.current_paint {
            self.canvas.draw_oval(&rect, &current_paint.skia_paint);
        } else {
            panic!("Attempt to use paint while altering it!");
        }
    }

    fn draw_line(&mut self, start: (f32, f32), mov: (f32, f32)) {
        if let Some(ref mut current_paint) = self.current_paint {
            self.canvas.draw_line(start, (start.0 + mov.0, start.1 + mov.1), &current_paint.skia_paint);
        } else {
            panic!("Attempt to use paint while altering it!");
        }
    }
}

pub fn create_canvas<'a>(surface_canvas: &'a Canvas) -> SkiaCanvas<'a> {
   SkiaCanvas {
        canvas: surface_canvas,
        current_paint: Some(SkiaPaint::new()),
    }
}