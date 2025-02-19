use minifb::{Key, Window as ClientWindow, WindowOptions};

use crate::{Mandelbrot, MandelbrotWindow};

pub struct App {
    client: ClientWindow,
    mandelbrot_window: MandelbrotWindow,
    window_width: usize,
    widow_height: usize,
    window_buf: Vec<u32>,
    mandelbrot: Mandelbrot,
}

impl App {
    pub fn new(window_width: usize, window_height: usize, mandelbrot: Mandelbrot) -> Self {
        let mut mb_window = MandelbrotWindow::new();

        mb_window.translate(-1., 0.);

        let client = ClientWindow::new(
            "Mandelbrot",
            window_width,
            window_height,
            WindowOptions::default(),
        )
        .unwrap();

        // a 32 bit color
        let window_buf: Vec<u32> = vec![0x00FFFFFF; window_width * window_height];

        Self {
            client,
            mandelbrot,
            mandelbrot_window: mb_window,
            window_buf,
        }
    }

    pub fn run(mut self) {
        while self.client.is_open() && !self.client.is_key_down(Key::Escape) {
            self.mandelbrot_window
                .render(&mut self.window_buf, &self.mandelbrot);

            self.client
                .update_with_buffer(
                    &self.window_buf,
                    self.mandelbrot_window.window_width,
                    self.mandelbrot_window.window_height,
                )
                .unwrap();

            self.mandelbrot_window.zoom(1. / 1.01);
        }
    }
}
