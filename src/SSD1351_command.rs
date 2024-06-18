use alloc::{vec, vec::{Vec}};

pub enum SSD1351_command
{
    SSD1351_COMMAND_LOCK_OLED_DRIVER_LOCK,
    SSD1351_COMMAND_LOCK_OLED_DRIVER_UNLOCK,
    SSD1351_COMMAND_LOCK_A2_B1_B3_BB_BE_C1_ACCESS,
    SSD1351_COMMAND_LOCK_A2_B1_B3_BB_BE_C1_NO_ACCESS,
    SSD1351_DISPLAY_OFF,
    SSD1351_DISPLAY_RESET,
    SSD1351_DISPLAY_NORMAL_MODE,
    SSD1351_DISPLAY_START_LINE,
    SSD1351_DISPLAY_OFFSET,
    SSD1351_DISPLAY_TOGGLE_ENHANCEMENT,
    SSD1351_DISPLAY_NO_ENHANCEMENT,
    SSD1351_SELECT_FUNCTION,
    SSD1351_SLEEP_OFF,
    SSD1351_ENABLE_WRITE_TO_RAM,
    SSD1351_SET_COLUMN_ADDRESS,
    SSD1351_SET_ROW_ADDRESS,
    SSD1351_SET_FRONT_CLOCK_DIVIDER,
    SSD1351_SET_MUX_RATIO,
    SSD1351_SET_REMAP_AND_COLOR_DEPTH,
    SSD1351_SET_GPIO,
    SSD1351_SET_SEGMENT_LOW_VOLTAGE,
    SSD1351_SET_PRECHARGE_VOLTAGE,
    SSD1351_SET_SECOND_PRECHARGE_PERIOD,
    SSD1351_SET_COM_DESELECT_VOLTAGE,
    SSD1351_SET_CONTRAST_FOR_COLOR,
    SSD1351_SET_PHASE_RESET,
    SSD1351_SET_MASTER_CONTRAST_CURRENT_CONTROL
}

impl SSD1351_command
{
    // Getter method to retrieve the field's value
    pub fn command_hex(&self) -> u8 {
        match self
        {
            SSD1351_command::SSD1351_COMMAND_LOCK_OLED_DRIVER_LOCK => 0xfd,
            SSD1351_command::SSD1351_COMMAND_LOCK_OLED_DRIVER_UNLOCK => 0xfd,
            SSD1351_command::SSD1351_COMMAND_LOCK_A2_B1_B3_BB_BE_C1_ACCESS => 0xfd,
            SSD1351_command::SSD1351_COMMAND_LOCK_A2_B1_B3_BB_BE_C1_NO_ACCESS => 0xfd,
            SSD1351_command::SSD1351_DISPLAY_OFF => 0xae,
            SSD1351_command::SSD1351_DISPLAY_RESET => 0xa6,
            SSD1351_command::SSD1351_DISPLAY_NORMAL_MODE => 0xa4,
            SSD1351_command::SSD1351_DISPLAY_START_LINE => 0xa1,
            SSD1351_command::SSD1351_DISPLAY_OFFSET => 0xa2,
            SSD1351_command::SSD1351_DISPLAY_TOGGLE_ENHANCEMENT => 0xB2,
            SSD1351_command::SSD1351_DISPLAY_NO_ENHANCEMENT => 0xB2,
            SSD1351_command::SSD1351_SELECT_FUNCTION => 0xAB,
            SSD1351_command::SSD1351_SLEEP_OFF => 0xAF,
            SSD1351_command::SSD1351_ENABLE_WRITE_TO_RAM => 0x5C,
            SSD1351_command::SSD1351_SET_COLUMN_ADDRESS => 0x15,
            SSD1351_command::SSD1351_SET_ROW_ADDRESS => 0x75,
            SSD1351_command::SSD1351_SET_FRONT_CLOCK_DIVIDER => 0xB3,
            SSD1351_command::SSD1351_SET_MUX_RATIO => 0xCA,
            SSD1351_command::SSD1351_SET_REMAP_AND_COLOR_DEPTH => 0xA0,
            SSD1351_command::SSD1351_SET_GPIO => 0xB5,
            SSD1351_command::SSD1351_SET_SEGMENT_LOW_VOLTAGE => 0xB4,
            SSD1351_command::SSD1351_SET_PRECHARGE_VOLTAGE => 0xBB,
            SSD1351_command::SSD1351_SET_SECOND_PRECHARGE_PERIOD => 0xB6,
            SSD1351_command::SSD1351_SET_COM_DESELECT_VOLTAGE => 0xBE,
            SSD1351_command::SSD1351_SET_CONTRAST_FOR_COLOR => 0xC1,
            SSD1351_command::SSD1351_SET_PHASE_RESET => 0xB1,
            SSD1351_command::SSD1351_SET_MASTER_CONTRAST_CURRENT_CONTROL => 0xC7
        }
    }
    
    pub fn default_value_hex(&self) -> Vec<u8>
    {
        match self
        {
            SSD1351_command::SSD1351_COMMAND_LOCK_OLED_DRIVER_LOCK => vec![0x16],
            SSD1351_command::SSD1351_COMMAND_LOCK_OLED_DRIVER_UNLOCK => vec![0x12],
            SSD1351_command::SSD1351_COMMAND_LOCK_A2_B1_B3_BB_BE_C1_ACCESS => vec![0xB1],
            SSD1351_command::SSD1351_COMMAND_LOCK_A2_B1_B3_BB_BE_C1_NO_ACCESS => vec![0xB0],
            SSD1351_command::SSD1351_DISPLAY_OFF => vec![],
            SSD1351_command::SSD1351_DISPLAY_RESET =>  vec![],
            SSD1351_command::SSD1351_DISPLAY_NORMAL_MODE => vec![],
            SSD1351_command::SSD1351_DISPLAY_START_LINE => vec![0],
            SSD1351_command::SSD1351_DISPLAY_OFFSET => vec![0],
            SSD1351_command::SSD1351_DISPLAY_TOGGLE_ENHANCEMENT => vec![0xA4, 0, 0],
            SSD1351_command::SSD1351_DISPLAY_NO_ENHANCEMENT => vec![0, 0, 0],
            SSD1351_command::SSD1351_SELECT_FUNCTION => vec![0x0],
            SSD1351_command::SSD1351_SLEEP_OFF => vec![],
            SSD1351_command::SSD1351_ENABLE_WRITE_TO_RAM => vec![],
            SSD1351_command::SSD1351_SET_COLUMN_ADDRESS => vec![0, 0x7f],
            SSD1351_command::SSD1351_SET_ROW_ADDRESS => vec![0, 0x7f],
            SSD1351_command::SSD1351_SET_FRONT_CLOCK_DIVIDER => vec![0xF1],
            SSD1351_command::SSD1351_SET_MUX_RATIO => vec![0x7F],
            SSD1351_command::SSD1351_SET_REMAP_AND_COLOR_DEPTH => vec![0xA4],
            SSD1351_command::SSD1351_SET_GPIO => vec![0],
            SSD1351_command::SSD1351_SET_SEGMENT_LOW_VOLTAGE => vec![0xA0, 0xB5, 0x55],
            SSD1351_command::SSD1351_SET_PRECHARGE_VOLTAGE => vec![0x17],
            SSD1351_command::SSD1351_SET_SECOND_PRECHARGE_PERIOD => vec![0x01],
            SSD1351_command::SSD1351_SET_COM_DESELECT_VOLTAGE => vec![0x05],
            SSD1351_command::SSD1351_SET_CONTRAST_FOR_COLOR => vec![0xC8, 0x80, 0xC8],
            SSD1351_command::SSD1351_SET_PHASE_RESET => vec![0x32],
            SSD1351_command::SSD1351_SET_MASTER_CONTRAST_CURRENT_CONTROL => vec![0x0F],
        }
    }
}



// Default: 0 -> 127
// SSD1351_SET_COLUMN_ADDRESS
// SSD1351_SET_ROW_ADDRESS

// Default: first 4 bits = divider = 2; last 4 bits = frequency = 15 (read Right to Left)
// SSD1351_SET_FRONT_CLOCK_DIVIDER

// Default: max mux val
// SSD1351_SET_MUX_RATIO

// Default: Horizontal increment, addres 0 map to SEG0, swapped color sequence, scan from COM[mux-1] to COM0, enable com split odd even, 262k color, 16-bit format 2
// SSD1351_SET_REMAP_AND_COLOR_DEPTH

// Default: 160, 181, 85
// SSD1351_SET_SEGMENT_LOW_VOLTAGE

// Default: Reset (17h)
// SSD1351_SET_PRECHARGE_VOLTAGE

// Default: 1 DCLKS
// SSD1351_SET_SECOND_PRECHARGE_PERIOD

// Default: Reset (05h)
// SSD1351_SET_COM_DESELECT_VOLTAGE

// Default: 200, 128, 192
// SSD1351_SET_CONTRAST_FOR_COLOR

// Default: first phase, 4 bits = 6 dclks, second phase, 3 bits = 2 DCKLS
// SSD1351_SET_PHASE_RESET

// Default: Enable Internal VDD regulator
// SSD1351_SET_FUNCTION_SELECT

// Default: No change. (No reduction)
// SSD1351_SET_MASTER_CONTRAST_CURRENT_CONTROL

