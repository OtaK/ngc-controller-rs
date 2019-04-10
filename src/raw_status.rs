use bitfield::bitfield;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u16)]
pub enum GamecubeDeviceId {
    N64Controller = 0x0500,
    N64Microphone = 0x0001,
    N64Keyboard = 0x0002,
    N64Mouse = 0x0200,
    GBA = 0x0004,
    GBAAlt = 0x0800,
    GCStandardController = 0x0900,
    GCWavebirdReceiver = 0xe960,
    GCWavebird = 0xe9a0,
    GCWavebirdAlt1 = 0xa800,
    GCWavebirdAlt2 = 0xebb0,
    GCKeyboard = 0x0820,
    GCSteeringWheel = 0x0800,
    Unknown = 0x0000,
}

impl From<u16> for GamecubeDeviceId {
    fn from(value: u16) -> Self {
        match value {
            0x0500 => GamecubeDeviceId::N64Controller,
            0x0001 => GamecubeDeviceId::N64Microphone,
            0x0002 => GamecubeDeviceId::N64Keyboard,
            0x0200 => GamecubeDeviceId::N64Mouse,
            0x0004 => GamecubeDeviceId::GBA,
            0x0800 => GamecubeDeviceId::GBAAlt,
            0x0900 => GamecubeDeviceId::GCStandardController,
            0xe960 => GamecubeDeviceId::GCWavebirdReceiver,
            0xe9a0 => GamecubeDeviceId::GCWavebird,
            0xa800 => GamecubeDeviceId::GCWavebirdAlt1,
            0xebb0 => GamecubeDeviceId::GCWavebirdAlt2,
            0x0820 => GamecubeDeviceId::GCKeyboard,
            0x0800 => GamecubeDeviceId::GCSteeringWheel,
            _ => GamecubeDeviceId::Unknown
        }
    }
}

bitfield! {
    pub struct RawGamecubeDeviceStatus(u16);
    impl Debug;

    u16, device_id: 0, 16;
    u8, state0, _: 17, 20; // ignored
    rumble;
    u8, state1, _: 22, 32; // ignored
}
