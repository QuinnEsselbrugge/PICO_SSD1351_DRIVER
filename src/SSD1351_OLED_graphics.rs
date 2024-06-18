#![no_std]

// color enum

use micromath::F32Ext;

use alloc::{vec::Vec};

use crate::ssd1351_driver::SSD1351_font::FONT11X18;

#[derive(Clone)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

pub struct QGI_Image_Info {
    pub image_width: u8,
    pub image_height: u8,
    pub start_position_x: u8,
    pub start_position_y: u8,
    pub nr_colours: u16,
}

#[derive(Clone)]
pub struct SSD1351_Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl SSD1351_Color {
    pub fn get_color_data_6_bit(&self) -> [u8; 3] {
        let red: u8 = self.red >> 2;
        let green: u8 = self.green >> 2;
        let blue: u8 = self.blue >> 2;

        return [red, green, blue];
    }
}

pub struct SSD1351_FBO {
    pub width: i16,
    pub height: i16,
    pub pixels: Vec<SSD1351_Color>,
}

impl SSD1351_FBO {
    pub fn draw_pixel(&mut self, x: i16, y: i16, color: &SSD1351_Color) {
        
        // Saturating sub ensures value stays above 0
        let index: i16 = ((x.saturating_sub(0)) * self.width) + (y.saturating_sub(0));

        // Because of rusts nature I have to manually create my own copy of the color value to comply with ownership

        if let Some(value) = self.pixels.get_mut(index as usize) {
            *value = color.clone();
        } else {
            return;
        }
    }

    pub fn draw_circle(&mut self, origin: Point, radius: i16, color: &SSD1351_Color) {
        // Adapted version of bresenhams line algorithm for circles
        let mut x = -radius;
        let mut y = 0;
        let mut err = 2 - 2 * radius;
        let mut r_d = radius;

        while x < 0 {
            self.draw_pixel(origin.x - x, origin.y + y, color);
            self.draw_pixel(origin.x - y, origin.y - x, color);
            self.draw_pixel(origin.x + x, origin.y - y, color);
            self.draw_pixel(origin.x + y, origin.y + x, color);

            r_d = err;

            if r_d <= y {
                y += 1;

                err += y * 2 + 1;
            }

            if (r_d > x || err > y) {
                x += 1;
                err += x * 2 + 1;
            }
        }
    }

    pub fn draw_circle_fill(&mut self, origin: Point, radius: i16, color: &SSD1351_Color) {
        let y = -radius;
        let x = -radius;

        let radius_sqr: i16 = radius * radius;

        for y in y..=radius {
            for x in x..=radius {
                if x * x + y * y <= radius_sqr {
                    self.draw_pixel(origin.x + x, origin.y + y, color)
                }
            }
        }
    }

    pub fn draw_rect(&mut self, origin: Point, width: i16, height: i16, color: &SSD1351_Color) {
        self.draw_fast_h_line(
            Point {
                x: origin.x,
                y: origin.y,
            },
            width,
            color,
        );
        self.draw_fast_h_line(
            Point {
                x: origin.x,
                y: origin.y + height - 1,
            },
            width,
            color,
        );

        self.draw_fast_v_line(
            Point {
                x: origin.x,
                y: origin.y,
            },
            height,
            color,
        );
        self.draw_fast_v_line(
            Point {
                x: origin.x + width - 1,
                y: origin.y,
            },
            height,
            color,
        );
    }

    pub fn draw_rect_fill(
        &mut self,
        origin: Point,
        width: i16,
        height: i16,
        color: &SSD1351_Color,
    ) {
        for i in 0..height {
            self.draw_fast_h_line(
                Point {
                    x: origin.x,
                    y: origin.y + i,
                },
                width,
                color,
            );
        }
    }

    pub fn draw_fast_v_line(&mut self, start: Point, length: i16, color: &SSD1351_Color) {
        for i in 0..length {
            self.draw_pixel(start.x, start.y + i, color)
        }
    }

    pub fn draw_fast_h_line(&mut self, start: Point, length: i16, color: &SSD1351_Color) {
        for i in 0..length {
            self.draw_pixel(start.x + i, start.y, color)
        }
    }

    pub fn draw_line(&mut self, start: Point, end: Point, color: &SSD1351_Color) {
        // Implement Bresenhams line algorithm using octagonal for both directions with integer math (Fast!)

        let mut mut_start: Point = start.clone();
        let diff_x = (end.x - start.x).abs();
        let select_x = if start.x < end.x { 1 } else { -1 };

        let diff_y = -(end.y - start.y).abs();
        let select_y = if start.y < end.y { 1 } else { -1 };

        let mut error = diff_x + diff_y;

        loop {
            self.draw_pixel(mut_start.x, mut_start.y, color);

            if mut_start.x == end.x && mut_start.y == end.y {
                break; // Line done, perfect line
            }

            // This can be done because we are doing pure integer math

            /*
                "All of the derivation for the algorithm is done. One performance issue is the 1/2 factor in the initial value of D.
                Since all of this is about the sign of the accumulated difference, then everything can be multiplied by 2 with no consequence."
            */

            let error_sup = 2 * error;

            if error_sup >= diff_y {
                if mut_start.x == end.x {
                    break;
                }

                error = error + diff_y;
                mut_start.x = mut_start.x + select_x;
            }

            if error_sup <= diff_x {
                if start.y == end.y {
                    break;
                }

                error = error + diff_x;
                mut_start.y = mut_start.y + select_y;
            }
        }
    }

    pub fn draw_char(
        &mut self,
        origin: Point,
        char: char,
        font: &FONT11X18,
        color: &SSD1351_Color,
    ) {
        // bits for characters are read left to right apparently. that took like 1.5 hours woops
        let char_data: [u16; 18] = FONT11X18::get_char(char as u8 + 1);

        for y in 0..font.height {
            for x in 0..font.width {
                if (char_data[y as usize] >> (font.width - x) + 4) & 1 != 0 {
                    self.draw_pixel(origin.x + x as i16, origin.y + y as i16, color)
                }
            }
        }
    }

    pub fn draw_string(
        &mut self,
        origin: Point,
        string: &str,
        font: &FONT11X18,
        color: &SSD1351_Color,
    ) {
        let mut index = 0;

        for char in string.chars() {
            let point: Point = Point {
                x: origin.x + (font.width as i16 * index),
                y: origin.y,
            };
            self.draw_char(point, char, font, color);

            index += 1;
        }
    }

    fn get_qgi_image_information(&self, image_vector: &Vec<u16>) -> QGI_Image_Info {
        // 0 -> ImageSize : 2 * 8
        // 1 -> StartPosition  : 2 * 8
        // 2 -> NrColours : 1 * 16

        let image_width: u8 = ((image_vector[0] >> 8) & 0xFF) as u8;
        let image_height: u8 = (image_vector[0] & 0xFF) as u8;

        let start_position_x: u8 = ((image_vector[1] >> 8) & 0xFF) as u8;
        let start_position_y: u8 = (image_vector[1] & 0xFF) as u8;

        let nr_colours: u16 = image_vector[2];

        return QGI_Image_Info {
            image_width,
            image_height,
            start_position_x,
            start_position_y,
            nr_colours,
        };
    }

    fn get_qgi_colour_pallete(
        &self,
        image_vector: &Vec<u16>,
        image_info: &QGI_Image_Info,
    ) -> Vec<SSD1351_Color> {
        let mut out_vec: Vec<SSD1351_Color> = Vec::new();
        let info_offset = 3; // nr info fields
        let start_pallete_offset = image_info.nr_colours / 2 * 3;

        for i in (info_offset..start_pallete_offset + info_offset).step_by(3) {
            let red_first: u8 = ((image_vector[i as usize] >> 8) & 0xFF) as u8;
            let green_first: u8 = (image_vector[i as usize] & 0xFF) as u8;
            let blue_first: u8 = ((image_vector[i as usize + 1] >> 8) & 0xFF) as u8;

            let red_second: u8 = (image_vector[i as usize + 1] & 0xFF) as u8;
            let green_second: u8 = ((image_vector[i as usize + 2] >> 8) & 0xFF) as u8;
            let blue_second: u8 = (image_vector[i as usize + 2] & 0xFF) as u8;

            out_vec.push(SSD1351_Color {
                red: red_first,
                green: green_first,
                blue: blue_first,
            });
            out_vec.push(SSD1351_Color {
                red: red_second,
                green: green_second,
                blue: blue_second,
            });
        }

        return out_vec;
    }

    pub fn draw_qgi_image(&mut self, image_vector: Vec<u16>) {
        let image_info: QGI_Image_Info = self.get_qgi_image_information(&image_vector);
        let colour_pallete: Vec<SSD1351_Color> =
            self.get_qgi_colour_pallete(&image_vector, &image_info);

        let data_block_offset = (image_info.nr_colours / 2 * 3) + 3; // nr info fields

        let mut x: i16 = image_info.start_position_x as i16;
        let mut y: i16 = image_info.start_position_y as i16;

        for i in data_block_offset..image_vector.len() as u16 {
            let colour_index: u8 = ((image_vector[i as usize] >> 8) & 0xFF) as u8;
            let nr_times: u8 = (image_vector[i as usize] & 0xFF) as u8;
            let colour = &colour_pallete[colour_index as usize];

            for _j in 0..nr_times {
                self.draw_pixel(x, y, &colour);

                if x == image_info.image_width as i16 {
                    y += 1;
                    x = 0;
                }

                x += 1;
            }
        }
    }

    pub fn release_pixels(&mut self) {
        self.pixels.clear();
    }

    // Returns an approximation for the distance between two points
    fn calculate_points_distance(&self, start: &Point, end: &Point) -> f32 {
        // distance: d=√((x2 – x1)² + (y2 – y1)²)

        let distance: f32 = ((end.x - start.x).pow(2) + (end.y - start.y).pow(2)).into();
        let distance = distance.sqrt();

        return distance;
    }
}
