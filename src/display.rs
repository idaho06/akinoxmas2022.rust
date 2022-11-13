extern crate sdl2;

use std::collections::HashMap;
use std::ffi::c_void;

use byte_slice_cast::AsMutSliceOf;
use image::RgbaImage;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::{EventPump, Sdl, TimerSubsystem};
//use sdl2::rect::{Point, Rect};
use sdl2::sys::{SDL_DisplayMode, SDL_GetCurrentDisplayMode};

pub struct Display {
    sdl_context: Sdl,
    //video_subsystem: VideoSubsystem,
    timer: TimerSubsystem,
    //w_width: u32,
    //w_height: u32,
    //window: Window,
    canvas: Canvas<Window>,
    //texture_creator: TextureCreator<WindowContext>,
    texture: Texture,
    t_width: u32,
    t_height: u32,
    color_buffer: Box<[u8]>,
    sprites: HashMap<String, Texture>,
}

impl Display {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let timer = sdl_context.timer().unwrap();

        let mut dm = SDL_DisplayMode {
            format: 0,
            w: 0,
            h: 0,
            refresh_rate: 0,
            driverdata: 0 as *mut c_void,
        };
        unsafe {
            SDL_GetCurrentDisplayMode(0 as i32, &mut dm);
        }

        println!("w: {} h: {}", dm.w, dm.h);
        let w_width: u32 = dm.w as u32;
        let w_height: u32 = dm.h as u32;
        //let w_width: u32 = 640;
        //let w_height: u32 = 480;

        let window = video_subsystem
            .window("rust-sdl2 demo: Video", w_width, w_height)
            .position_centered()
            .opengl()
            .borderless()
            //.fullscreen()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let canvas = window
            .into_canvas()
            .accelerated()
            //.software()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        let texture_creator = canvas.texture_creator();
        println!("Texture formats: {:?}", canvas.info().texture_formats);

        let t_width: u32 = w_width / 3;
        let t_height: u32 = w_height / 3;
        println!("t_w: {} t_h: {}", t_width, t_height);

        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::ARGB8888, t_width, t_height)
            .map_err(|e| e.to_string())
            .unwrap();
        texture.set_blend_mode(sdl2::render::BlendMode::Blend);
        // Create a red-green gradient
        /* texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..256 {
                for x in 0..256 {
                    let offset = y * pitch + x * 4;
                    buffer[offset] = 0; // blue
                    buffer[offset + 1] = y as u8; // green
                    buffer[offset + 2] = x as u8; // red
                    buffer[offset + 3] = 0 as u8; // ?? alpha but seems ignored
                }
            }
        })?; */

        //let pixel_data: Vec<u8> = vec![0; 1920*4*1080 as usize];
        //let mut pixel_data: [u8; 1920*4*1080 as usize] = [0; 1920*4*1080 as usize];

        let mut pixel_data: Vec<u8> = Vec::with_capacity((t_width * 4 * t_height) as usize);

        for _offset in 0..(t_width * 4 * t_height) as usize {
            pixel_data.push(0);
        }

        let pixel_data = pixel_data.into_boxed_slice();

        //let mut pixel_data2 = RgbaImage::new(t_width, t_height);
        //let test = pixel_data2.into_raw().into_boxed_slice();

        Self {
            sdl_context: sdl_context,
            //video_subsystem: video_subsystem,
            timer: timer,
            //w_width: w_width,
            //w_height: w_height,
            //window: window,
            canvas: canvas,
            texture: texture,
            t_width: t_width,
            t_height: t_height,
            color_buffer: pixel_data,
            sprites: HashMap::new(),
        }
    }

    pub fn cls(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        //self.canvas.present();
    }

    pub fn present_canvas(&mut self) {
        self.canvas.present();
    }

    pub fn event_pump(&self) -> EventPump {
        self.sdl_context.event_pump().unwrap()
    }

    pub fn ticks(&self) -> u32 {
        self.timer.ticks()
    }

    pub fn t_width(&self) -> u32 {
        self.t_width
    }

    pub fn t_height(&self) -> u32 {
        self.t_height
    }

    pub fn put_pixel_raw(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let offset = (y * self.t_width as usize * 4) + (x * 4);
        self.color_buffer[offset] = b as u8; // blue
        self.color_buffer[offset + 1] = g as u8; // green
        self.color_buffer[offset + 2] = r as u8; // red
        self.color_buffer[offset + 3] = 255 as u8; // ?? alpha but seems ignored */
                                                   //canvas.draw_point(Point::new(x as i32, y as i32))?;
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8) {
        if x < 0 || x > (self.t_width - 1) as i32 || y < 0 || y > (self.t_height - 1) as i32 {
            return;
        }
        let color: u32 = u32::from_be_bytes([0xff, r, g, b]); //ARGB888
        let offset = ((y * self.t_width as i32) + (x)) as usize;
        let pixel_data_u32 = self.color_buffer.as_mut_slice_of::<u32>().unwrap();
        pixel_data_u32[offset] = color;
    }

    pub fn clear_color_buffer(&mut self, r: u8, g: u8, b: u8) {
        let color: u32 = u32::from_be_bytes([0x00, r, g, b]); //ARGB888
        let pixel_data_u32 = self.color_buffer.as_mut_slice_of::<u32>().unwrap();
        pixel_data_u32[..].fill(color);
    }

    pub fn color_buffer_to_canvas(&mut self) {
        self.texture
            .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                buffer.copy_from_slice(&self.color_buffer);
            })
            .unwrap();

        self.canvas.copy(&self.texture, None, None).unwrap();
    }

    pub fn add_sprite(&mut self, name: &str, filename: &str) {
        let image_from_file = image::open(filename)
            .expect(format!("Cannot read image file: {}", filename).as_str())
            .into_rgba8();
        let mut buffer_for_sprite = Vec::<u8>::new();
        for image_pixel in image_from_file.pixels() {
            
            
            
            buffer_for_sprite.push(image_pixel.0[2]); // blue
            buffer_for_sprite.push(image_pixel.0[1]); // green
            buffer_for_sprite.push(image_pixel.0[0]); // red
            buffer_for_sprite.push(image_pixel.0[3]); // alpha
        }
        let surface_for_sprite = Surface::from_data(
            buffer_for_sprite.as_mut_slice(), 
            image_from_file.width(), 
            image_from_file.height(), 
            image_from_file.width() * 4, 
            PixelFormatEnum::ARGB8888).unwrap();      
        let texture_creator = self.canvas.texture_creator();
        self.sprites.insert(
            name.to_string(), 
            texture_creator.create_texture_from_surface(surface_for_sprite).unwrap());
    }

    pub fn put_sprite(&mut self, name: &str, x: i32, y: i32, size_factor: f32) { 
        match self.sprites.get(name) {
            Some(texture) => {
                let width = (texture.query().width as f32 * size_factor) as u32;
                let height = (texture.query().height as f32 * size_factor) as u32;
                self.canvas.copy(texture, None, Some(Rect::new(x,y,width,height))).unwrap()
            },
            None => (),
        }
    }
}
