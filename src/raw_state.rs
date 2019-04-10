use bitfield::bitfield;
use bitflags::bitflags;

bitflags! {
    struct GamecubeDpadState: u8 {
        const NEUTRAL = 0;
        const LEFT = 1 << 0;
        const RIGHT = 1 << 1;
        const DOWN = 1 << 2;
        const UP = 1 << 3;

        const UP_RIGHT = Self::UP.bits | Self::RIGHT.bits;
        const UP_LEFT = Self::UP.bits | Self::LEFT.bits;
        const DOWN_RIGHT = Self::DOWN.bits | Self::RIGHT.bits;
        const DOWN_LEFT = Self::DOWN.bits | Self::LEFT.bits;
    }
}

bitfield! {
    pub struct RawGamecubeInputState(u64);
    impl Debug;

    btn_a;
    btn_b;
    btn_x;
    btn_y;
    btn_start;
    high0;
    err_latch;
    err_stat;

    dpad_left;
    dpad_right;
    dpad_down;
    dpad_up;
    btn_z;
    btn_r;
    btn_l;
    high1;

    u8, into u8, x_axis: 16, 23;
    u8, into u8, x_axis: 24, 30;
    u8, into u8, c_x_axis: 31, 38;
    u8, into u8, c_y_axis: 39, 46;
    u8, into u8, r_analog: 47, 54;
    u8, into u8, l_analog: 55, 64;
}
