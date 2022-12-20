extern crate sdl2;

//use std::collections::HashMap;
use rustc_hash::FxHashMap; // fast hashmap
use std::ffi::c_void;

use byte_slice_cast::AsMutSliceOf; // cast to a different type for slices u8->u32
                                   //use image::RgbaImage;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::surface::Surface;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl, TimerSubsystem};
//use sdl2::rect::{Point, Rect};
use sdl2::sys::{SDL_DisplayMode, SDL_GetCurrentDisplayMode};

use crate::point::Pixel;
use crate::vector::{Vec2, Vec3};

struct StreamingBuffer {
    color_buffer: Box<[u8]>,
    texture: Texture,
}

pub struct Display {
    sdl_context: Sdl,
    //video_subsystem: VideoSubsystem,
    timer: TimerSubsystem,
    w_width: u32,
    w_height: u32,
    scaling_factor_w: f32,
    scaling_factor_h: f32,
    //window: Window,
    canvas: Canvas<Window>,
    //texture_creator: TextureCreator<WindowContext>,
    //texture: Texture,
    //t_width: u32,
    //t_height: u32,
    //color_buffer: Box<[u8]>,
    sprites: FxHashMap<String, Texture>,
    //color_buffers: HashMap<String, Box<[u8]>>,
    //streaming_textures: HashMap<String, Texture>,
    streaming_buffers: FxHashMap<String, StreamingBuffer>,
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

        let w_width: u32 = 1920;
        let w_height: u32 = 1080;

        // Calculate scaling factor from screen size to a virtual 1920x1080
        let scaling_factor_w: f32 = dm.w as f32 / w_width as f32;
        let scaling_factor_h: f32 = dm.h as f32 / w_height as f32;

        //let w_width: u32 = dm.w as u32;
        //let w_height: u32 = dm.h as u32;

        println!("Virtual display w: {} h: {}", w_width, w_height);
        println!("Size factors w: {} h: {}", scaling_factor_w, scaling_factor_h);

        let mut window = video_subsystem
            .window("AkinoXmas 2022", dm.w as u32, dm.h as u32)
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
        //let surface_for_icon = Surface::from_data(
        //    buffer_for_icon.as_mut_slice(),
        //    image_from_file.width(),
        //    image_from_file.height(),
        //    image_from_file.width() * 4,
        //    PixelFormatEnum::ARGB8888,
        //)
        //.unwrap();
        let mut pixels = Self::load_icon_256_256_4();
        window.set_icon(
            Surface::from_data(
                pixels.as_mut_slice(), 
                256, 
                256, 
                256*4, 
                PixelFormatEnum::ARGB8888 
            )
            .unwrap()
        );
        
        let canvas = window
            .into_canvas()
            .accelerated()
            //.software()
            .present_vsync()
            //.target_texture()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        //let texture_creator = canvas.texture_creator();
        //println!("Texture formats: {:?}", canvas.info().texture_formats);

        //let t_width: u32 = w_width / 3;
        //let t_height: u32 = w_height / 3;
        //println!("t_w: {} t_h: {}", t_width, t_height);

        //let mut texture = texture_creator
        //    .create_texture_streaming(PixelFormatEnum::ARGB8888, t_width, t_height)
        //    .map_err(|e| e.to_string())
        //    .unwrap();
        //texture.set_blend_mode(sdl2::render::BlendMode::Blend);
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

        //let mut pixel_data: Vec<u8> = Vec::with_capacity((t_width * 4 * t_height) as usize);

        //for _offset in 0..(t_width * 4 * t_height) as usize {
        //    pixel_data.push(0);
        //}

        //let pixel_data = pixel_data.into_boxed_slice();

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
            scaling_factor_w,
            scaling_factor_h,
            //window: window,
            //canvas: canvas,
            canvas,
            //texture: texture,
            //texture,
            //t_width: t_width,
            //t_width,
            //t_height: t_height,
            //t_height,
            //color_buffer: pixel_data,
            sprites: FxHashMap::default(),
            //color_buffers: HashMap::new(),
            //streaming_textures: HashMap::new(),
            streaming_buffers: FxHashMap::default(),
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

    /* pub fn t_width(&self) -> u32 {
        self.t_width
    } */

    /* pub fn t_height(&self) -> u32 {
        self.t_height
    } */

    pub fn w_height(&self) -> u32 {
        self.w_height
    }

    pub fn w_width(&self) -> u32 {
        self.w_width
    }

    /* pub fn put_pixel_raw(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let offset = (y * self.t_width as usize * 4) + (x * 4);
        self.color_buffer[offset] = b as u8; // blue
        self.color_buffer[offset + 1] = g as u8; // green
        self.color_buffer[offset + 2] = r as u8; // red
        self.color_buffer[offset + 3] = 255_u8;
        // clippy warning: casting integer literal to `u8` is unnecessary
        //self.color_buffer[offset + 3] = 255 as u8; // ?? alpha but seems ignored
        //canvas.draw_point(Point::new(x as i32, y as i32))?;
    } */

    /* pub fn put_pixel(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8) {
        if x < 0 || x > (self.t_width - 1) as i32 || y < 0 || y > (self.t_height - 1) as i32 {
            return;
        }
        let color: u32 = u32::from_be_bytes([0xff, r, g, b]); //ARGB888
        let offset = ((y * self.t_width as i32) + (x)) as usize;
        let pixel_data_u32 = self.color_buffer.as_mut_slice_of::<u32>().unwrap();
        pixel_data_u32[offset] = color;
    } */

    // pub fn put_pixel(&mut self, name: &str, x: i32, y: i32, r: u8, g: u8, b: u8) {
    //     //if let Some(streaming_texture) = self.streaming_textures.get(name) {
    //     //    if let Some(src_buffer) = self.color_buffers.get_mut(name) {
    //     if let Some(streaming_buffer) = self.streaming_buffers.get_mut(name) {
    //         let width = streaming_buffer.texture.query().width;
    //         let height = streaming_buffer.texture.query().height;
    //         if x < 0 || x > (width - 1) as i32 || y < 0 || y > (height - 1) as i32 {
    //             return;
    //         }
    //         let color: u32 = u32::from_be_bytes([0xff, r, g, b]); //ARGB888
    //         let offset = ((y * width as i32) + (x)) as usize;
    //         let pixel_data_u32 = streaming_buffer
    //             .color_buffer
    //             .as_mut_slice_of::<u32>()
    //             .unwrap();
    //         pixel_data_u32[offset] = color;
    //         //}
    //     }
    // }

    // clippy warning: writing `&Vec` instead of `&[_]` involves a new object where a slice will do
    pub fn put_pixel_queue(&mut self, name: &str, pixel_queue: &[Pixel]) { // replaced "&Vec<Pixel>" with "&[Pixel]" ==> huge performance gain!!
        if let Some(streaming_buffer) = self.streaming_buffers.get_mut(name) {
            let width = streaming_buffer.texture.query().width;
            let height = streaming_buffer.texture.query().height;
            pixel_queue.iter().for_each(|pixel| {
                let x = pixel.x;
                let y = pixel.y;
                let a = pixel.a;
                let r = pixel.r;
                let g = pixel.g;
                let b = pixel.b;
                if x < 0 || x > (width - 1) as i32 || y < 0 || y > (height - 1) as i32 {
                    //()
                } else {
                    let color: u32 = u32::from_be_bytes([a, r, g, b]); //ARGB888
                    let offset = ((y * width as i32) + (x)) as usize;
                    let pixel_data_u32 = streaming_buffer
                        .color_buffer
                        .as_mut_slice_of::<u32>()
                        .unwrap();
                    pixel_data_u32[offset] = color;
                }
            });
        }
    }

    /* pub fn clear_color_buffer(&mut self, r: u8, g: u8, b: u8) {
        let color: u32 = u32::from_be_bytes([0x00, r, g, b]); //ARGB888
        let pixel_data_u32 = self.color_buffer.as_mut_slice_of::<u32>().unwrap();
        pixel_data_u32[..].fill(color);
    } */

    pub fn clear_streaming_buffer(&mut self, name: &str, r: u8, g: u8, b: u8) {
        if let Some(streaming_buffer) = self.streaming_buffers.get_mut(name) {
            let color: u32 = u32::from_be_bytes([0x00, r, g, b]); //ARGB888
            let pixel_data_u32 = streaming_buffer
                .color_buffer
                .as_mut_slice_of::<u32>()
                .unwrap();
            pixel_data_u32[..].fill(color);
        }
    }

    pub fn add_streaming_buffer(&mut self, name: &str, width: usize, height: usize) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::ARGB8888, width as u32, height as u32)
            .map_err(|e| e.to_string())
            .unwrap();
        texture.set_blend_mode(sdl2::render::BlendMode::Blend);

        let mut pixel_data: Vec<u8> = Vec::with_capacity(width * 4 * height);

        for _offset in 0..(width * 4 * height) {
            pixel_data.push(0);
        }

        let pixel_data = pixel_data.into_boxed_slice();

        self.streaming_buffers.insert(
            name.to_string(),
            StreamingBuffer {
                color_buffer: pixel_data,
                texture,
            },
        );
    }

    /*     pub fn add_color_buffer(&mut self, name: &str, width: usize, height: usize) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::ARGB8888, width as u32, height as u32)
            .map_err(|e| e.to_string())
            .unwrap();
        texture.set_blend_mode(sdl2::render::BlendMode::Blend);

        let mut pixel_data: Vec<u8> = Vec::with_capacity(width * 4 * height);

        for _offset in 0..(width * 4 * height) {
            pixel_data.push(0);
        }

        let pixel_data = pixel_data.into_boxed_slice();

        self.color_buffers.insert(name.to_string(), pixel_data);
        self.streaming_textures.insert(name.to_string(), texture);
    } */

    pub fn streaming_buffer_to_canvas(&mut self, name: &str) {
        /* if let Some(streaming_texture) = self.streaming_textures.get_mut(name) {
        if let Some(src_buffer) = self.color_buffers.get(name) { */
        if let Some(streaming_buffer) = self.streaming_buffers.get_mut(name) {
            streaming_buffer
                .texture
                //streaming_texture
                .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                    buffer.copy_from_slice(&streaming_buffer.color_buffer);
                })
                .unwrap();
            /*
            streaming_texture
                .update(None, src_buffer, width as usize * 4).unwrap();
            */
            //}
            self.canvas
                .copy(&streaming_buffer.texture, None, None)
                .unwrap();
        }
    }

    /* pub fn color_buffer_to_canvas(&mut self) {
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
    } */

    fn load_icon_256_256_4() -> Vec<u8> {
        let image_from_file = image::open("./assets/bola_roja.png")
            // clippy warning: use of `expect` followed by a function call
            //.expect(format!("Cannot read image file: {}", filename).as_str())
            .unwrap_or_else(|_| panic!("Cannot read icon file"))
            .into_rgba8();
        let mut buffer_for_icon = Vec::<u8>::new();
        for image_pixel in image_from_file.pixels() {
            buffer_for_icon.push(image_pixel.0[2]); // blue
            buffer_for_icon.push(image_pixel.0[1]); // green
            buffer_for_icon.push(image_pixel.0[0]); // red
            buffer_for_icon.push(image_pixel.0[3]); // alpha
        }
        //let surface_for_icon = Surface::from_data(
        //    buffer_for_icon.as_mut_slice(),
        //    image_from_file.width(),
        //    image_from_file.height(),
        //    image_from_file.width() * 4,
        //    PixelFormatEnum::ARGB8888,
        //)
        //.unwrap();
        buffer_for_icon
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
            PixelFormatEnum::ARGB8888,
        )
        .unwrap();
        let texture_creator = self.canvas.texture_creator();
        self.sprites.insert(
            name.to_string(),
            texture_creator
                .create_texture_from_surface(surface_for_sprite)
                .unwrap(),
        );
    }

    pub fn streaming_buffer_width(&self, name: &str) -> Result<u32, String> {
        //let width;
        if let Some(streaming_buffer) = self.streaming_buffers.get(name) {
            Ok(streaming_buffer.texture.query().width)
        } else {
            Err("No such streaming_buffer".to_string())
        }
    }

    pub fn streaming_buffer_height(&self, name: &str) -> Result<u32, String> {
        //let width;
        if let Some(streaming_buffer) = self.streaming_buffers.get(name) {
            Ok(streaming_buffer.texture.query().height)
        } else {
            Err("No such streaming_buffer".to_string())
        }
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
            self.canvas
                .copy(texture, None, Some(Rect::new(x, y, width, height)))
                .unwrap()
        }
    }

    pub fn put_sprite_centered(&mut self, name: &str, x: i32, y: i32, size_factor: f32, mod_color: Option<(u8,u8,u8)>) {
        if let Some(texture) = self.sprites.get_mut(name) {
            let width = (texture.query().width as f32 * size_factor) as i32;
            let height = (texture.query().height as f32 * size_factor) as i32;
            let cx = x - width / 2;
            let cy = y - height / 2;
            if let Some((r, g, b)) = mod_color {
                texture.set_color_mod(r, g, b);    
            } else {
                texture.set_color_mod(255_u8, 255_u8, 255_u8);
            }
            // transform width, height, cx and cy for the virtual 1920x1080 screen
            // to the real screen
            let width: i32 = (width as f32 * self.scaling_factor_w) as i32;
            let height: i32 = (height as f32 * self.scaling_factor_h) as i32;
            let cx: i32 = (cx as f32 * self.scaling_factor_w) as i32;
            let cy: i32 = (cy as f32 * self.scaling_factor_h) as i32;

            self.canvas
                .copy(
                    texture,
                    None,
                    Some(Rect::new(cx, cy, width as u32, height as u32)),
                )
                .unwrap()
        }
    }

    pub fn put_sprite_rect(&mut self, name: &str, x: i32, y: i32, rect: &Rect) {
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
            let dest = Rect::new(x, y, rect.width(), rect.height());
            self.canvas.copy(texture, Some(src), Some(dest)).unwrap()
        }
    }

    pub fn put_sprite_rect_rect(&mut self, name: &str, src_rect: &Rect, dst_rect: &Rect) {
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
            // transform width, height, x and y for the virtual 1920x1080 screen
            // to the real screen
            let width: u32 = (dst_rect.w as f32 * self.scaling_factor_w) as u32;
            let height: u32 = (dst_rect.h as f32 * self.scaling_factor_h) as u32;
            let x: i32 = (dst_rect.x as f32 * self.scaling_factor_w) as i32;
            let y: i32 = (dst_rect.y as f32 * self.scaling_factor_h) as i32;
            let dst_rect = Rect::new(x, y, width, height);

            self.canvas
                .copy(texture, Some(*src_rect), Some(dst_rect))
                .unwrap()
        }
    }

    pub fn put_sprite_ex(&mut self, name: &str, src_rect: &Rect, dst_rect: &Rect) {
        if let Some(texture) = self.sprites.get(name) {
            self.canvas
                .copy_ex(
                    texture,
                    Some(*src_rect),
                    Some(*dst_rect),
                    0_f64,
                    //Some(sdl2::rect::Point::new(center_x, center_y)),
                    None,
                    false,
                    false,
                )
                .unwrap();
        }
    }

    pub fn project(&self, v: &Vec3) -> Vec2 {
        let fov_factor: f32 = 640.0; // TODO: calculate FOV from color buffer size
        Vec2 {
            x: (fov_factor * v.x) / v.z,
            y: (fov_factor * -v.y) / v.z,
        }
    }

    // i = interval. Always change in +1 or -1 steps
    // d = displacement. Fractional increment, rounded to integer.
    fn interpolate_int(&self, i0: i32, d0: i32, i1: i32, d1: i32) -> Vec<(i32, i32)> {
        if i0 == i1 {
            return vec![(i0, d0)];
        }
        let distance = (i1 - i0).unsigned_abs(); // clippy change as usize to .unsigned_abs()
        let mut values: Vec<(i32, i32)> = Vec::with_capacity(distance.try_into().unwrap()); // clippy convert to usize and panic in case of error
        let mut a: f32 = (d1 as f32 - d0 as f32) / (i1 as f32 - i0 as f32);
        // clippy warning: unneeded late initialization
        let step: i32 = if i1 > i0 { 1 } else { -1 };
        if step == -1 {
            a = -a;
        } // change sign of a if we are going backwards
        let mut i = i0;
        let mut d = d0 as f32;
        loop {
            values.push((i, d as i32));

            d += a;
            i += step;

            if step == 1 && i > i1 {
                break;
            }
            if step == -1 && i < i1 {
                break;
            }
        }

        values
    }

    // pub fn line(&mut self, name: &str, x0: i32, y0: i32, x1: i32, y1: i32, r: u8, g: u8, b: u8) {
    //     let dx = x1 - x0;
    //     let dy = y1 - y0;

    //     if dx.abs() > dy.abs() {
    //         // horizontal-ish
    //         /* for (x, y) in self.interpolate_int(x0, y0, x1, y1).iter() {
    //             self.put_pixel(name, *x, *y, r, g, b);
    //         } */
    //         self.interpolate_int(x0, y0, x1, y1)
    //             .iter()
    //             .for_each(|(x, y)| self.put_pixel(name, *x, *y, r, g, b));
    //     } else {
    //         // vertical-ish
    //         /* for (y, x) in self.interpolate_int(y0, x0, y1, x1).iter() {
    //             self.put_pixel(name, *x, *y, r, g, b);
    //         } */
    //         self.interpolate_int(y0, x0, y1, x1)
    //             .iter()
    //             .for_each(|(y, x)| self.put_pixel(name, *x, *y, r, g, b));
    //     }
    // }

    #[allow(clippy::too_many_arguments)]
    pub fn line_queue(&self, queue: &mut Vec<Pixel>, x0: i32, y0: i32, x1: i32, y1: i32, r: u8, g: u8, b: u8) {
        let dx = x1 - x0;
        let dy = y1 - y0;

        if dx.abs() > dy.abs() {
            // horizontal-ish
            /* for (x, y) in self.interpolate_int(x0, y0, x1, y1).iter() {
                self.put_pixel(name, *x, *y, r, g, b);
            } */
            self.interpolate_int(x0, y0, x1, y1)
                .iter()
                .for_each(|(x, y)| 
                    //self.put_pixel(name, *x, *y, r, g, b)
                    queue.push(Pixel { 
                        x: *x, 
                        y: *y, 
                        a: 255_u8, 
                        r, 
                        g, 
                        b, })
                );
        } else {
            // vertical-ish
            /* for (y, x) in self.interpolate_int(y0, x0, y1, x1).iter() {
                self.put_pixel(name, *x, *y, r, g, b);
            } */
            self.interpolate_int(y0, x0, y1, x1)
                .iter()
                .for_each(|(y, x)| 
                    //self.put_pixel(name, *x, *y, r, g, b)
                    queue.push(Pixel { 
                        x: *x, 
                        y: *y, 
                        a: 255_u8, 
                        r, 
                        g, 
                        b, })
                );
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn line_horizontal_queue(&self, queue: &mut Vec<Pixel>, x0: i32, y0: i32, x1: i32, r: u8, g: u8, b: u8) {
        let mut x0 = x0;
        let mut x1 = x1;
        if x1 < x0 { (x0, x1) = (x1, x0); }
        for x in x0..=x1 {
            queue.push(Pixel { 
                x, 
                y: y0, 
                a: 255_u8, 
                r, 
                g, 
                b, })
        }
    }

    // pub fn filled_triangle(
    //     &mut self,
    //     name: &str,
    //     point0: &Vec2,
    //     point1: &Vec2,
    //     point2: &Vec2,
    //     r: u8,
    //     g: u8,
    //     b: u8,
    // ) {
    //     let mut v0 = *point0;
    //     let mut v1 = *point1;
    //     let mut v2 = *point2;

    //     // order the vertices by the y
    //     if v1.y < v0.y {
    //         (v0, v1) = (v1, v0);
    //     }
    //     if v2.y < v0.y {
    //         (v0, v2) = (v2, v0);
    //     }
    //     if v2.y < v1.y {
    //         (v1, v2) = (v2, v1);
    //     }

    //     // floats to integers
    //     let y0 = v0.y.round() as i32;
    //     let x0 = v0.x.round() as i32;
    //     let y1 = v1.y.round() as i32;
    //     let x1 = v1.x.round() as i32;
    //     let y2 = v2.y.round() as i32;
    //     let x2 = v2.x.round() as i32;

    //     // get lines of the edges of the triangles
    //     // we assume v0 to v2 is the longest edge (in the y scale)
    //     let edge0 = self.interpolate_int(y0, x0, y2, x2); //v0 to v2, longest
    //     let mut edge1 = self.interpolate_int(y0, x0, y1, x1); // v0 to v1
    //     let mut edge2 = self.interpolate_int(y1, x1, y2, x2); // v1 to v2

    //     // join the two short edges into one. Remove the last element of the first edge and then concatenate
    //     edge1.pop();
    //     edge1.append(&mut edge2);

    //     // at this point, edge0 is the (y, x) line from top to bottom vertices
    //     // edge1 is the (y, x) lines from top to midle to bottom vertices

    //     // now we zip the two edges to define the horizontal lines that builds the triangle
    //     /*
    //     let triangle_iter = edge0.iter().zip(edge1.iter());
    //     for ((y0,x0),(y1, x1)) in triangle_iter {
    //         self.line(name, *x0, *y0, *x1, *y1, r, g, b);
    //     } */
    //     edge0
    //         .iter()
    //         .zip(edge1.iter())
    //         .for_each(|((y0, x0), (y1, x1))| 
    //             self.line(name, *x0, *y0, *x1, *y1, r, g, b)
    //             //y0 and y1 are always equal
    //         );
    // }

    #[allow(clippy::too_many_arguments)]
    pub fn filled_triangle_queue(
        &self,
        queue: &mut Vec<Pixel>,
        point0: &Vec2,
        point1: &Vec2,
        point2: &Vec2,
        r: u8,
        g: u8,
        b: u8,
    ) {
        let mut v0 = *point0;
        let mut v1 = *point1;
        let mut v2 = *point2;

        // order the vertices by the y
        if v1.y < v0.y {
            (v0, v1) = (v1, v0);
        }
        if v2.y < v0.y {
            (v0, v2) = (v2, v0);
        }
        if v2.y < v1.y {
            (v1, v2) = (v2, v1);
        }

        // floats to integers
        let y0 = v0.y as i32;
        let x0 = v0.x as i32;
        let y1 = v1.y as i32;
        let x1 = v1.x as i32;
        let y2 = v2.y as i32;
        let x2 = v2.x as i32;

        // get lines of the edges of the triangles
        // we assume v0 to v2 is the longest edge (in the y scale)
        let edge0 = self.interpolate_int(y0, x0, y2, x2); //v0 to v2, longest
        let mut edge1 = self.interpolate_int(y0, x0, y1, x1); // v0 to v1
        let mut edge2 = self.interpolate_int(y1, x1, y2, x2); // v1 to v2

        // join the two short edges into one. Remove the last element of the first edge and then concatenate
        edge1.pop();
        edge1.append(&mut edge2);

        // at this point, edge0 is the (y, x) line from top to bottom vertices
        // edge1 is the (y, x) lines from top to midle to bottom vertices

        // now we zip the two edges to define the horizontal lines that builds the triangle
        /*
        let triangle_iter = edge0.iter().zip(edge1.iter());
        for ((y0,x0),(y1, x1)) in triangle_iter {
            self.line(name, *x0, *y0, *x1, *y1, r, g, b);
        } */
        edge0
            .iter()
            .zip(edge1.iter())
            .for_each(|((y0, x0), (_y1, x1))| 
                //self.line_queue(queue, *x0, *y0, *x1, *y1, r, g, b);
                //y0 and y1 are always equal
                self.line_horizontal_queue(queue, *x0, *y0, *x1, r, g, b)
            );
    }
}
