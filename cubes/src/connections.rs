#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Connection {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

pub const fn c(x: i8, y: i8, z: i8) -> Connection {
    Connection { x, y, z }
}

pub const FULL_CUBE: &[Connection] = &[
    c(0, 0, -1), // bottom
    c(0, 0, 1),  // top
    c(1, 0, 0),  // right
    c(-1, 0, 0), // left
    c(0, 1, 0),  // front
    c(0, -1, 0), // back
];

pub const OPEN_CUBE: &[Connection] = &[
    c(0, 0, -1),
    c(1, 0, 0),
    c(0, 1, 0),
    c(-1, 0, 0),
    c(0, -1, 0),
];

pub const HALF_CUBE: &[Connection] = &[c(0, 0, -1), c(0, 1, 0), c(-1, 0, 0), c(0, -1, 0)];

pub const THIRD_CUBE: &[Connection] = &[c(0, 0, -1), c(-1, 0, 0), c(0, -1, 0)];

pub const ONE_CONNECTION: &[Connection] = &[c(0, 0, -1)];

pub const ROD: &[Connection] = &[c(0, 0, -1), c(0, 0, 5)];

pub const ROD_SHORT: &[Connection] = &[c(0, 0, -1), c(0, 0, 3)];

pub const ROD_LONG: &[Connection] = &[c(0, 0, -1), c(0, 0, 9)];

pub const ROD_ARC: &[Connection] = &[c(0, 0, -1), c(5, 0, 4)];

pub const ROD_ARC_SHORT: &[Connection] = &[c(0, 0, -1), c(3, 0, 2)];

pub const ROD_DIAGONAL_2D: &[Connection] = &[c(0, 0, -1), c(4, 0, 5)];

pub const ROD_DIAGONAL_2D_SHORT: &[Connection] = &[c(0, 0, -1), c(2, 0, 3)];

pub const ROD_DIAGONAL_3D: &[Connection] = &[c(0, 0, -1), c(4, -4, 5)];

pub const ROD_PLUS: &[Connection] = &[c(0, 0, -1), c(0, 0, 5), c(3, 0, 2), c(-3, 0, 2)];

pub const ROD_FRAME: &[Connection] = &[c(0, 0, -1), c(4, 0, 5), c(5, 0, 0), c(-1, 0, 4)];

pub const ROD_CROSS: &[Connection] = &[c(0, 0, -1), c(4, 0, -1), c(0, 0, 5), c(4, 0, 5)];

pub const T3_PROP: &[Connection] = &[c(0, 0, -1), c(0, 2, -1), c(0, -2, -1)];

pub const T4_PROP: &[Connection] = &[
    c(0, 0, -1),
    c(0, 2, -1),
    c(0, -2, -1),
    c(2, 0, -1),
    c(-2, 0, -1),
];

pub const T5_LASER: &[Connection] = &[c(0, 0, -1), c(4, -10, -1), c(-4, -10, -1)];

pub const T2_MECH_LEG: &[Connection] = &[
    c(0, 0, -1),
    c(0, 2, -1),
    c(0, -2, -1),
    c(0, 3, 0),
    c(0, -3, 0),
    c(1, 0, 0),
    c(-1, 0, 0),
    c(1, 2, 0),
    c(-1, 2, 0),
    c(1, -2, 0),
    c(-1, -2, 0),
];

pub const T3_MECH_LEG: &[Connection] = &[
    c(0, 0, -1),
    c(0, 2, -1),
    c(0, -2, -1),
    c(0, 3, 0),
    c(0, -3, 0),
    c(3, 0, 0),
    c(-3, 0, 0),
    c(1, 2, 0),
    c(-1, 2, 0),
    c(1, -2, 0),
    c(-1, -2, 0),
    c(2, 1, 0),
    c(-2, 1, 0),
    c(2, -1, 0),
    c(-2, -1, 0),
];

pub const T4_MECH_LEG: &[Connection] = &[
    // R1
    c(0, 0, -1),
    c(2, 0, -1),
    c(-2, 0, -1),
    c(0, 2, -1),
    c(0, -2, -1),
    c(2, 2, -1),
    c(-2, -2, -1),
    c(2, -2, -1),
    c(-2, 2, -1),
    // R2
    c(3, 0, 0),
    c(3, 2, 0),
    c(3, -2, 0),
    c(-3, 0, 0),
    c(-3, 2, 0),
    c(-3, -2, 0),
    c(0, 3, 0),
    c(2, 3, 0),
    c(-2, 3, 0),
    c(0, -3, 0),
    c(2, -3, 0),
    c(-2, -3, 0),
];

pub const T5_MECH_LEG: &[Connection] = &[
    // R1
    c(0, 0, -1),
    c(2, 0, -1),
    c(-2, 0, -1),
    c(0, 2, -1),
    c(0, -2, -1),
    c(2, 2, -1),
    c(-2, -2, -1),
    c(2, -2, -1),
    c(-2, 2, -1),
    // R2
    c(4, 0, -1),
    c(-4, 0, -1),
    c(0, 4, -1),
    c(0, -4, -1),
    c(4, 2, -1),
    c(-4, 2, -1),
    c(4, -2, -1),
    c(-4, -2, -1),
    c(2, 4, -1),
    c(-2, 4, -1),
    c(2, -4, -1),
    c(-2, -4, -1),
    c(4, 4, -1),
    c(4, -4, -1),
    c(-4, 4, -1),
    c(-4, -4, -1),
    // R3
    c(5, 0, 0),
    c(5, 2, 0),
    c(5, -2, 0),
    c(5, 4, 0),
    c(5, -4, 0),
    c(-5, 0, 0),
    c(-5, 2, 0),
    c(-5, -2, 0),
    c(-5, 4, 0),
    c(-5, -4, 0),
    c(0, 5, 0),
    c(2, 5, 0),
    c(-2, 5, 0),
    c(4, 5, 0),
    c(-4, 5, 0),
    c(0, -5, 0),
    c(2, -5, 0),
    c(-2, -5, 0),
    c(4, -5, 0),
    c(-4, -5, 0),
];

pub const TX_T5_B_LEFT: &[Connection] = &[
    c(0, 0, -1),
    c(-6, 0, -1),
    c(5, 0, -6),
    c(0, 3, -6),
    c(-6, 3, -6),
];

pub const TX_T5_B_RIGHT: &[Connection] = &[
    c(0, 0, -1),
    c(6, 0, -1),
    c(-5, 0, -6),
    c(0, 3, -6),
    c(6, 3, -6),
];

pub const TX_T5_C: &[Connection] = &[
    c(0, 0, -1),
    c(0, 4, -1),
    c(12, 4, -1),
    c(12, 0, -1),
    c(-12, 4, -1),
    c(-12, 0, -1),
];

pub const T3_WING: &[Connection] = &[c(0, 0, -1), c(0, -2, -1)];

pub const T4_WING: &[Connection] = &[c(0, 2, -1), c(0, 0, -1), c(0, -2, -1)];

pub const T5_WING: &[Connection] = &[c(0, 2, -1), c(0, 0, -1), c(0, -2, -1), c(0, -4, -1)];

pub const T4_RUDDER: &[Connection] = &[c(0, 0, -1), c(0, -2, -1), c(0, -4, -1)];

pub const T5_RUDDER: &[Connection] = &[c(0, 0, -1), c(0, -2, -1), c(0, -4, -1), c(0, -6, -1)];

pub const T5_SEEKER: &[Connection] = &[
    c(0, 0, -1),
    c(0, 4, -1),
    c(0, -4, -1),
    c(4, 0, -1),
    c(-4, 0, -1),
];

pub const T5_AEROFLAK: &[Connection] = &[
    c(0, 0, -1),
    c(0, 10, -1),
    c(0, -10, -1),
    c(10, 0, -1),
    c(-10, 0, -1),
];

pub const T5_CHAINGUN: &[Connection] = &[
    c(0, 0, -1),
    c(2, 2, -1),
    c(2, -2, -1),
    c(-2, 2, -1),
    c(-2, -2, -1),
];

pub const POWER: &[Connection] = &[
    c(0, 0, -1),
    c(2, 2, -1),
    c(2, -2, -1),
    c(-2, 2, -1),
    c(-2, -2, -1),
    c(0, -3, 2),
];

pub const DISC: &[Connection] = &[
    c(0, 0, -1),
    c(0, 0, 5),
    c(0, 3, 2),
    c(0, -3, 2),
    c(3, 0, 2),
    c(-3, 0, 2),
];

pub const BLINK: &[Connection] = &[
    c(0, 0, -1),
    c(2, 0, -1),
    c(-2, 0, -1),
    c(2, 0, 5),
    c(0, 0, 5),
    c(-2, 0, 5),
];

pub const GHOST: &[Connection] = &[c(0, 0, -1), c(0, 0, 5), c(3, 0, 2), c(-3, 0, 2)];

pub const T5_HOVER: &[Connection] = &[
    c(0, 0, -1),
    c(2, 2, -1),
    c(-2, 2, -1),
    c(4, 0, -1),
    c(-4, 0, -1),
];

pub const T5_WHEEL: &[Connection] = &[
    c(0, 0, -1),
    c(0, -3, -2),
    c(4, -3, -2),
    c(-4, -3, -2),
    c(4, 0, -1),
    c(-4, 0, -1),
];

pub const T1_TANK_TRACK: &[Connection] = &[c(0, 0, -1), c(0, 0, 5), c(3, 0, 2)];

pub const T2_TANK_TRACK: &[Connection] = &[c(0, 0, -1), c(0, 0, 7), c(3, 0, 4)];

pub const T3_TANK_TRACK: &[Connection] = &[c(0, 0, -1), c(0, 0, 7), c(3, 0, 4), c(3, 0, 2)];

pub const T4_TANK_TRACK: &[Connection] = &[
    c(0, 0, -1),
    c(0, 0, 9),
    c(3, 0, 4),
    c(3, 4, 4),
    c(0, 4, -1),
    c(0, 4, 9),
];

pub const T5_TANK_TRACK: &[Connection] = &[
    c(0, 0, -1),
    c(0, 4, -1),
    c(0, -4, -1),
    c(0, 0, 9),
    c(0, 4, 9),
    c(0, -4, 9),
    c(3, 0, 4),
    c(3, 4, 4),
    c(3, -4, 4),
];

pub const STRUT_DIAGONAL_3D_LEFT: &[Connection] = &[c(0, 0, -1), c(4, -4, 5)];

pub const STRUT_DIAGONAL_3D_RIGHT: &[Connection] = &[c(0, 0, -1), c(-4, -4, 5)];

pub const STRUT_DIAGONAL_3D_LEFT_SHORT: &[Connection] = &[c(0, 0, -1), c(2, -2, 3)];

pub const STRUT_DIAGONAL_3D_RIGHT_SHORT: &[Connection] = &[c(0, 0, -1), c(-2, -2, 3)];
