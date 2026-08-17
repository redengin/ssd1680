
/// https://resource.heltec.cn/download/Wireless_Paper/E-Ink%20Datasheet/E-INK%20V1.0(DEPG0213BNS800F41-2.0)%20.pdf#page=15
pub enum Commands {
    /// start the screen update sequence
    MasterActivation,
    /// 
    DisplayUpdateControl2(DisplayMode),

}
impl Commands {
    pub fn id(&self) -> u8
    {
        return match self {
            Self::MasterActivation => 0x20,
            Self::DisplayUpdateControl2(_) => 0x22,


        }
    }
}


/// https://resource.heltec.cn/download/Wireless_Paper/E-Ink%20Datasheet/E-INK%20V1.0(DEPG0213BNS800F41-2.0)%20.pdf#page=19
pub enum DisplayMode {
    Mode2 = 0xF7,
}