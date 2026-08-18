/// https://resource.heltec.cn/download/Wireless_Paper/E-Ink%20Datasheet/E-INK%20V1.0(DEPG0213BNS800F41-2.0)%20.pdf#page=15
pub enum Commands {
    DriverOutputControl {
        height: u32,    // A: height-1, not sure what B does (appears to always be 0)
    },
    DataEntryMode(data_entry_mode::DataSequence),
    SoftReset,
    TempSensorSelect(temp_sensor_select::TempSensor),
    /// start the screen update sequence
    MasterActivation,
    /// TODO
    DisplayUpdateControl1{
        red_option: display_update_control1::RamOption,
        bw_option: display_update_control1::RamOption,
        mode: display_update_control1::Mode,
    },
    DisplayUpdateControl2(display_update_control2::DisplayMode),
    /// after this command, data will be written into BW RAM until a command change
    StartBwUpdate,
    /// after this command, data will be written into RED RAM until a command change
    StartRedUpdate,
    BorderWaveForm{vbd: border_wave_form::Vbd, vbd_level: border_wave_form::VbdLevel, lut: border_wave_form::Lut},
}
impl Commands {
    pub fn id(&self) -> u8 {
        return match self {
            Self::DriverOutputControl { .. } => 0x01,
            Self::DataEntryMode(_) => 0x11,
            Self::SoftReset => 0x12,
            Self::TempSensorSelect(_) => 0x18,
            Self::MasterActivation => 0x20,
            Self::DisplayUpdateControl1{..} => 0x21,
            Self::DisplayUpdateControl2(_) => 0x22,
            Self::StartBwUpdate => 0x24,
            Self::StartRedUpdate => 0x26,
            Self::BorderWaveForm {..} => 0x3C,
        };
    }
}



// specialized properties for commands
//------------------------------------------------------------------------------

/// https://resource.heltec.cn/download/Wireless_Paper/E-Ink%20Datasheet/E-INK%20V1.0(DEPG0213BNS800F41-2.0)%20.pdf#page=16
mod data_entry_mode {
    pub enum DataSequence {
        DecYDecX = 0b00,
        DecYIncX = 0b01,
        IncYDecX = 0b10,
        IncYIncX = 0b11,
    }
}

/// https://resource.heltec.cn/download/Wireless_Paper/E-Ink%20Datasheet/E-INK%20V1.0(DEPG0213BNS800F41-2.0)%20.pdf#page=18
mod temp_sensor_select {
    pub enum TempSensor {
        // reset default
        External = 0x48,
        Internal = 0x80,
    }
}

/// https://resource.heltec.cn/download/Wireless_Paper/E-Ink%20Datasheet/E-INK%20V1.0(DEPG0213BNS800F41-2.0)%20.pdf#page=18
mod display_update_control1 {
    pub enum RamOption {
        Normal = 0b0000,
        Bypass = 0b0100,
        Invert = 0b1000,
    }

    pub enum Mode {
        /// Available Source from S0 to S175
        Mode0 = 0b0,
        /// Available Source from S8 to S167
        Mode1 = 0b1,
    }
}

/// https://resource.heltec.cn/download/Wireless_Paper/E-Ink%20Datasheet/E-INK%20V1.0(DEPG0213BNS800F41-2.0)%20.pdf#page=19
mod display_update_control2 {
    pub enum DisplayMode {
        Mode2 = 0xF7,
    }
}

/// https://resource.heltec.cn/download/Wireless_Paper/E-Ink%20Datasheet/E-INK%20V1.0(DEPG0213BNS800F41-2.0)%20.pdf#page=22
mod border_wave_form {
    pub enum Vbd {
        GsTransition = 0b00,
        FixLevel = 0b01,
        Vcom     = 0b10,
        /// reset default
        HiZ      = 0b11,
    }

    pub enum VbdLevel {
        /// reset default
        Vss = 0b00,
        Vsh1 = 0b01,
        Vsl = 0b10,
        Vsh2 = 0b11,
    }

    pub enum Lut {
        Lut0 = 0b00,
        Lut1 = 0b01,
        Lut2 = 0b10,
        Lut3 = 0b11,
    }

}
