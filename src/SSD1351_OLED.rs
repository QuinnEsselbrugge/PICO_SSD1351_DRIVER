use alloc::{vec, vec::Vec};

use crate::{
    SSD1351_OLED_graphics::{SSD1351_Color, SSD1351_FBO},
    SSD1351_command::SSD1351_command,
    SSD1351_command::SSD1351_command::*,
    SSD1351_SPI::SPIDisplayInterface,
};

pub struct SSD1351_OLED {
    pub spi_controller: SPIDisplayInterface,
    pub height: u32,
    pub width: u32,
}

impl SSD1351_OLED {
    const DISPLAY_INIT_TIME: u32 = 200; // ms

    pub fn init(&mut self, delay: &mut cortex_m::delay::Delay, setup_default: bool) {
        self.spi_controller.reset_interface(delay);

        if setup_default == true {
            self.setup_ssd1351_default(delay);
        }
    }

    pub fn sleep_mode()
    {

    }

    pub fn send_command(&mut self, command: SSD1351_command, value: Option<Vec<u8>>) {
        if value.is_none() {
            self.spi_controller
                .transfer_command_vec(command.command_hex(), command.default_value_hex());
        } else {
            self.spi_controller
                .transfer_command_vec(command.command_hex(), value.unwrap());
        }
    }

    pub fn clear(&mut self, fbo: &mut SSD1351_FBO) {
        self.send_command(SSD1351_SET_COLUMN_ADDRESS, None);
        self.send_command(SSD1351_SET_ROW_ADDRESS, None);

        self.send_command(SSD1351_ENABLE_WRITE_TO_RAM, None);

        for i in 0..self.height * self.width {
            self.spi_controller.transfer(&[0x0, 0x0, 0x0]);
            fbo.pixels[i as usize] = SSD1351_Color {
                    red: 0,
                    green: 0,
                    blue: 0,
            };
        }
    }

    pub fn update(&mut self, fbo: &mut SSD1351_FBO) {
        self.send_command(SSD1351_SET_COLUMN_ADDRESS, None);
        self.send_command(SSD1351_SET_ROW_ADDRESS, None);

        // Enable writing pixel data to the ram until the next command
        self.send_command(SSD1351_ENABLE_WRITE_TO_RAM, None);

        for i in 0..self.height * self.width {
            self.spi_controller
                .transfer(&fbo.pixels[i as usize].get_color_data_6_bit());
            // fbo.pixels[i as usize] = SSD1351_Color {
            //     red: 0,
            //     green: 0,
            //     blue: 0,
            // }; // pixel is consumed
        }
    }

    pub fn update_x_y(&mut self, x: u8, y: u8, color: &SSD1351_Color) {
        self.send_command(SSD1351_SET_COLUMN_ADDRESS, Some(vec![x]));
        self.send_command(SSD1351_SET_ROW_ADDRESS, Some(vec![y]));

        self.send_command(SSD1351_ENABLE_WRITE_TO_RAM, None);

        self.spi_controller.transfer(&color.get_color_data_6_bit());
    }

    fn setup_ssd1351_default(&mut self, delay: &mut cortex_m::delay::Delay) {

        // None will always default to the standard value for the command
        self.send_command(SSD1351_COMMAND_LOCK_OLED_DRIVER_UNLOCK, None);
        self.send_command(SSD1351_COMMAND_LOCK_A2_B1_B3_BB_BE_C1_ACCESS, None);
        self.send_command(SSD1351_DISPLAY_OFF, None);
        self.send_command(SSD1351_DISPLAY_NORMAL_MODE, None);
        self.send_command(SSD1351_SET_COLUMN_ADDRESS, None);
        self.send_command(SSD1351_SET_ROW_ADDRESS, None);
        self.send_command(SSD1351_SET_FRONT_CLOCK_DIVIDER, None);
        self.send_command(SSD1351_SET_MUX_RATIO, None);
        self.send_command(SSD1351_SET_REMAP_AND_COLOR_DEPTH, None);
        self.send_command(SSD1351_DISPLAY_START_LINE, None);
        self.send_command(SSD1351_DISPLAY_OFFSET, None);
        self.send_command(SSD1351_SELECT_FUNCTION, None);
        self.send_command(SSD1351_SET_SEGMENT_LOW_VOLTAGE, None);
        self.send_command(SSD1351_SET_CONTRAST_FOR_COLOR, None);
        self.send_command(SSD1351_SET_MASTER_CONTRAST_CURRENT_CONTROL, None);
        self.send_command(SSD1351_SET_PHASE_RESET, None);
        self.send_command(SSD1351_DISPLAY_TOGGLE_ENHANCEMENT, None);
        self.send_command(SSD1351_SET_PRECHARGE_VOLTAGE, None);
        self.send_command(SSD1351_SET_SECOND_PRECHARGE_PERIOD, None);
        self.send_command(SSD1351_SET_COM_DESELECT_VOLTAGE, None);
        self.send_command(SSD1351_DISPLAY_RESET, None);

        //  delay 200 ms according to datasheet required for OLED screen to "warm" up
        delay.delay_ms(Self::DISPLAY_INIT_TIME);

        self.send_command(SSD1351_SLEEP_OFF, None);
    }
}
