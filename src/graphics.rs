extern crate sdl2;
use sdl2::Sdl;
use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::Canvas;

struct Display {
    gfx: [[bool; 32];64],
    canvas: sdl2::render::Canvas,
}

impl Display {
    fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("CHIP-8",64,32)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas()
            .build()
            .unwrap();
        Display {
            gfx: [[false;32];64],
            canvas: canvas,
        }
    }
    fn clear_screen(&mut self) {
        self.gfx = [[false;32];64];
        self.draw();
    }
    fn draw(&mut self) {
        for row in 0..31 {
            for column in 0..63 {
                let point = Point::new(row,column);
                if self.gfx[row][column] {
                    canvas.set_draw_color(Color::RGB(255,255,255));
                }
                else {
                    canvas.set_draw_color(Color::RGB(0,0,0));
                }
                canvas.draw_point(point);
            }
        }
        canvas.clear();
        canvas.present();
    }
}
