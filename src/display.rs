extern crate sdl2;

use std::collections::HashMap;
use std::ffi::c_void;

use byte_slice_cast::AsMutSliceOf; // cast to a different type for slices u8->u32
//use image::RgbaImage;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::{EventPump, Sdl, TimerSubsystem};
//use sdl2::rect::{Point, Rect};
use sdl2::sys::{SDL_DisplayMode, SDL_GetCurrentDisplayMode};

use crate::vector::{Vec3, Vec2};

pub struct Display {
    sdl_context: Sdl,
    //video_subsystem: VideoSubsystem,
    timer: TimerSubsystem,
    w_width: u32,
    w_height: u32,
    //window: Window,
    canvas: Canvas<Window>,
    //texture_creator: TextureCreator<WindowContext>,
    texture: Texture,
    t_width: u32,
    t_height: u32,
    color_buffer: Box<[u8]>,
    sprites: HashMap<String, Texture>,
}

//clippy warning: you should consider adding a `Default` implementation for `Display`
impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
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
            // clippy warning: `0 as *mut _` detected
            //driverdata: 0 as *mut c_void,
            driverdata: std::ptr::null_mut::<c_void>(),
        };
        unsafe {
            // clippy warning: casting integer literal to `i32` is unnecessary
            //SDL_GetCurrentDisplayMode(0 as i32, &mut dm);
            SDL_GetCurrentDisplayMode(0_i32, &mut dm);
        }

        println!("Current display w: {} h: {}", dm.w, dm.h);
        //let w_width: u32 = dm.w as u32;
        //let w_height: u32 = dm.h as u32;
        
        let w_width: u32 = 1920;
        let w_height: u32 = 1080;
        println!("Forcing display w: {} h: {}", w_width, w_height);

        let window = video_subsystem
            .window("AkinoXmas 2022", w_width, w_height)
            .position_centered()
            //.vulkan()
            //.opengl()
            .borderless()
            //.fullscreen()
            //.resizable()
            //.maximized()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        //window.set_icon(icon); // This needs a surface. TODO: Create a load_icon function returning a surface using image crate instead of SDL_image

        let canvas = window
            .into_canvas()
            .accelerated()
            //.software()
            .present_vsync()
            //.target_texture()
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
            // clippy warning: redundant field names in struct initialization
            //sdl_context: sdl_context,
            sdl_context,
            //video_subsystem: video_subsystem,
            //timer: timer,
            timer,
            //w_width: w_width,
            w_width,
            //w_height: w_height,
            w_height,
            //window: window,
            //canvas: canvas,
            canvas,
            //texture: texture,
            texture,
            //t_width: t_width,
            t_width,
            //t_height: t_height,
            t_height,
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

    pub fn w_height(&self) -> u32 {
        self.w_height
    }

    pub fn w_width(&self) -> u32 {
        self.w_width
    }


    pub fn put_pixel_raw(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let offset = (y * self.t_width as usize * 4) + (x * 4);
        self.color_buffer[offset] = b as u8; // blue
        self.color_buffer[offset + 1] = g as u8; // green
        self.color_buffer[offset + 2] = r as u8; // red
        self.color_buffer[offset + 3] = 255_u8;
        // clippy warning: casting integer literal to `u8` is unnecessary
        //self.color_buffer[offset + 3] = 255 as u8; // ?? alpha but seems ignored */
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
        
        /* 
        self.texture
            .update(None, &self.color_buffer, self.t_width as usize * 4).unwrap();
        */
        self.canvas.copy(&self.texture, None, None).unwrap();
    }

    pub fn add_sprite(&mut self, name: &str, filename: &str) {
        let image_from_file = image::open(filename)
            // clippy warning: use of `expect` followed by a function call
            //.expect(format!("Cannot read image file: {}", filename).as_str())
            .unwrap_or_else(|_| panic!("Cannot read image file: {}", filename))
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
        // clippy warning: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
        /* 
        match self.sprites.get(name) {
            Some(texture) => {
                let width = (texture.query().width as f32 * size_factor) as u32;
                let height = (texture.query().height as f32 * size_factor) as u32;
                self.canvas.copy(texture, None, Some(Rect::new(x,y,width,height))).unwrap()
            },
            None => (),
        }
        */
        if let Some(texture) = self.sprites.get(name) {
            let width = (texture.query().width as f32 * size_factor) as u32;
            let height = (texture.query().height as f32 * size_factor) as u32;
            self.canvas.copy(texture, None, Some(Rect::new(x,y,width,height))).unwrap()
        }
    }

    pub fn put_sprite_centered(&mut self, name: &str, x: i32, y: i32, size_factor: f32) {
        if let Some(texture) = self.sprites.get(name) {
            let width = (texture.query().width as f32 * size_factor).round() as i32;
            let height = (texture.query().height as f32 * size_factor).round() as i32;
            let cx = x - width / 2;
            let cy = y - height / 2;
            self.canvas.copy(texture, None, Some(Rect::new(cx,cy,width as u32,height as u32))).unwrap()
        }
    }

    pub fn put_sprite_rect(&mut self, name: &str, x: i32, y: i32, rect: &Rect){
        // clippy warning: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
        /* 
        match self.sprites.get(name) {
            Some(texture) => {
                let src = *rect;
                let dest = Rect::new(x,y,rect.width(),rect.height());
                self.canvas.copy(texture, Some(src), Some(dest)).unwrap()
            },
            None => (),
        }
        */
        if let Some(texture) = self.sprites.get(name) {
            let src = *rect;
            let dest = Rect::new(x,y,rect.width(),rect.height());
            self.canvas.copy(texture, Some(src), Some(dest)).unwrap()
        }
    }

    pub fn put_sprite_rect_rect(&mut self, name: &str, src_rect: &Rect, dst_rect: &Rect){
        // clippy warning: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
        /* 
        match self.sprites.get(name) {
            Some(texture) => {
                self.canvas.copy(texture, Some(*src_rect), Some(*dst_rect)).unwrap()
            },
            None => (),
        }
        */
        if let Some(texture) = self.sprites.get(name) {
            self.canvas.copy(texture, Some(*src_rect), Some(*dst_rect)).unwrap()
        }
    }

    pub fn put_sprite_ex(&mut self, name: &str, src_rect: &Rect, dst_rect: &Rect){
        if let Some(texture) = self.sprites.get(name) {
            self.canvas.copy_ex(
                texture, 
                Some(*src_rect), Some(*dst_rect), 
                0_f64, 
                //Some(sdl2::rect::Point::new(center_x, center_y)), 
                None,
                false, 
                false).unwrap();
        }
    }


    pub fn project(&self, v: &Vec3) -> Vec2 {
        let fov_factor: f32 = 640.0; // TODO: calculate FOV from color buffer size
        Vec2 { 
            x: (fov_factor * v.x) / v.z , 
            y: (fov_factor * -v.y) / v.z ,
        }
    }
}
