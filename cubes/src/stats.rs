use super::idents::Ident::*;
use super::Cube;
use crate::connections;

#[derive(Debug, PartialEq, Clone)]
pub struct Stats {
    pub tier: u32,
    pub cpu: u32,
    pub health: u32,
    pub mass: f32,
}

const MEDIUM_CUBE: Stats = Stats {
    tier: 1,
    cpu: 1,
    health: 2100,
    mass: 10.0,
};

const HEAVY_CUBE: Stats = Stats {
    tier: 2,
    cpu: 1,
    health: 2800,
    mass: 70.0,
};

const LIGHT_CUBE: Stats = Stats {
    tier: 2,
    cpu: 1,
    health: 1500,
    mass: 0.8,
};

const TX_CUBE: Stats = Stats {
    tier: 5,
    cpu: 3,
    health: 4500,
    mass: 2.5,
};

const ROD: Stats = Stats {
    tier: 2,
    cpu: 3,
    health: 5400,
    mass: 3.0,
};

const ROD_SHORT: Stats = Stats {
    tier: 2,
    cpu: 2,
    health: 3600,
    mass: 2.0,
};

const ROD_LONG: Stats = Stats {
    tier: 2,
    cpu: 5,
    health: 9000,
    mass: 5.0,
};

const STRUT: Stats = Stats {
    tier: 3,
    cpu: 9,
    health: 27000,
    mass: 90.0,
};

const STRUT_SHORT: Stats = Stats {
    tier: 3,
    cpu: 6,
    health: 18000,
    mass: 90.0,
};

const STRUT_LONG: Stats = Stats {
    tier: 3,
    cpu: 15,
    health: 45000,
    mass: 150.0,
};

const STRUT_SLICE_SHORT: Stats = Stats {
    tier: 3,
    cpu: 8,
    health: 24000,
    mass: 80.0,
};

const STRUT_RAMP: Stats = Stats {
    tier: 3,
    cpu: 12,
    health: 36000,
    mass: 120.0,
};

const STRUT_SKEWED_PLUS: Stats = Stats {
    tier: 3,
    cpu: 27,
    health: 64800,
    mass: 270.0,
};

const T0_WHEEL: Stats = Stats {
    tier: 0,
    cpu: 10,
    health: 17500,
    mass: 260.0,
};

const T1_WHEEL: Stats = Stats {
    tier: 1,
    cpu: 20,
    health: 37600,
    mass: 520.0,
};

const T2_WHEEL: Stats = Stats {
    tier: 2,
    cpu: 30,
    health: 60300,
    mass: 780.0,
};

const T3_WHEEL: Stats = Stats {
    tier: 3,
    cpu: 40,
    health: 85600,
    mass: 1040.0,
};

const T4_WHEEL: Stats = Stats {
    tier: 4,
    cpu: 50,
    health: 113500,
    mass: 1300.0,
};

const T5_WHEEL: Stats = Stats {
    tier: 5,
    cpu: 100,
    health: 240000,
    mass: 2600.0,
};

const T5_MECH_LEG: Stats = Stats {
    tier: 5,
    cpu: 205,
    health: 533000,
    mass: 8200.0,
};

const T4_SPRINTER_LEG: Stats = Stats {
    tier: 4,
    cpu: 98,
    health: 186200,
    mass: 3430.0,
};

const T3_INSECT_LEG: Stats = Stats {
    tier: 3,
    cpu: 34,
    health: 91800,
    mass: 102.0,
};

const T4_INSECT_LEG: Stats = Stats {
    tier: 4,
    cpu: 44,
    health: 132000,
    mass: 132.0,
};

const T3_SKI: Stats = Stats {
    tier: 3,
    cpu: 20,
    health: 52000,
    mass: 12.5,
};

const T5_HOVER: Stats = Stats {
    tier: 5,
    cpu: 100,
    health: 180000,
    mass: 500.0,
};

const T4_WING: Stats = Stats {
    tier: 4,
    cpu: 26,
    health: 23400,
    mass: 26.0,
};

const T5_WING: Stats = Stats {
    tier: 5,
    cpu: 28,
    health: 28000,
    mass: 28.0,
};

const T4_RUDDER: Stats = Stats {
    tier: 4,
    cpu: 23,
    health: 18400,
    mass: 23.0,
};

const T5_RUDDER: Stats = Stats {
    tier: 5,
    cpu: 25,
    health: 22500,
    mass: 25.0,
};

const T5_THRUSTER: Stats = Stats {
    tier: 5,
    cpu: 18,
    health: 19800,
    mass: 14.0,
};

const T0_LASER: Stats = Stats {
    tier: 0,
    cpu: 10,
    health: 10000,
    mass: 20.0,
};

const T1_LASER: Stats = Stats {
    tier: 1,
    cpu: 14,
    health: 14000,
    mass: 28.0,
};

const T2_LASER: Stats = Stats {
    tier: 2,
    cpu: 18,
    health: 18000,
    mass: 36.0,
};

const T3_LASER: Stats = Stats {
    tier: 3,
    cpu: 22,
    health: 22000,
    mass: 44.0,
};

const T4_LASER: Stats = Stats {
    tier: 4,
    cpu: 26,
    health: 31200,
    mass: 52.0,
};

const T5_LASER: Stats = Stats {
    tier: 5,
    cpu: 250,
    health: 250000,
    mass: 500.0,
};

const T4_PLASMA: Stats = Stats {
    tier: 4,
    cpu: 30,
    health: 36000,
    mass: 60.0,
};

const T5_PLASMA: Stats = Stats {
    tier: 5,
    cpu: 300,
    health: 300000,
    mass: 600.0,
};

const T5_RAIL: Stats = Stats {
    tier: 5,
    cpu: 260,
    health: 260000,
    mass: 520.0,
};

const T5_SEEKER: Stats = Stats {
    tier: 5,
    cpu: 270,
    health: 270000,
    mass: 550.0,
};

const T5_CHAIN: Stats = Stats {
    tier: 5,
    cpu: 280,
    health: 392000,
    mass: 650.0,
};

const T5_ION: Stats = Stats {
    tier: 5,
    cpu: 240,
    health: 240000,
    mass: 490.0,
};

const T5_MORTAR: Stats = Stats {
    tier: 5,
    cpu: 290,
    health: 290000,
    mass: 580.0,
};

const T2_EP_A: Stats = Stats {
    tier: 2,
    cpu: 22,
    health: 66000,
    mass: 22.0,
};

const T2_EP_B: Stats = Stats {
    tier: 2,
    cpu: 24,
    health: 72000,
    mass: 24.0,
};

const T2_EP_C: Stats = Stats {
    tier: 2,
    cpu: 28,
    health: 84000,
    mass: 28.0,
};

const T3_EP_A: Stats = Stats {
    tier: 3,
    cpu: 32,
    health: 96000,
    mass: 32.0,
};

const T3_EP_B: Stats = Stats {
    tier: 3,
    cpu: 36,
    health: 108000,
    mass: 36.0,
};

const T3_EP_C: Stats = Stats {
    tier: 3,
    cpu: 40,
    health: 120000,
    mass: 40.0,
};

const T4_EP_A: Stats = Stats {
    tier: 4,
    cpu: 44,
    health: 114400,
    mass: 44.0,
};

const T4_EP_B: Stats = Stats {
    tier: 4,
    cpu: 48,
    health: 124800,
    mass: 48.0,
};

const T4_EP_C: Stats = Stats {
    tier: 4,
    cpu: 52,
    health: 135200,
    mass: 52.0,
};

const T5_EP_A: Stats = Stats {
    tier: 5,
    cpu: 56,
    health: 145600,
    mass: 56.0,
};

const T5_EP_B: Stats = Stats {
    tier: 5,
    cpu: 130,
    health: 208000,
    mass: 130.0,
};

const COSMETIC_1CPU: Stats = Stats {
    tier: 1,
    cpu: 1,
    health: 1,
    mass: 0.01,
};

const COSMETIC_3CPU: Stats = Stats {
    tier: 1,
    cpu: 3,
    health: 1,
    mass: 0.01,
};

#[allow(non_snake_case)]
const fn RC14_CUBE(tier: u32) -> Stats {
    Stats {
        tier,
        cpu: 1,
        health: 2000,
        mass: 10.0,
    }
}

#[allow(non_snake_case)]
const fn RC14_WHEEL(tier: u32) -> Stats {
    Stats {
        tier,
        cpu: tier * 10,
        health: tier * 200,
        mass: 10.0,
    }
}

pub static CUBES: [Cube; 493] = [
    Cube {
        id: 227205318,
        idents: &[Medium, Cube],
        stats: MEDIUM_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 227917916,
        idents: &[Medium, Edge],
        stats: MEDIUM_CUBE,
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 1931676396,
        idents: &[Medium, Corner],
        stats: MEDIUM_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 3559488176,
        idents: &[Medium, Inner],
        stats: MEDIUM_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 447126572,
        idents: &[Medium, Edge, Round],
        stats: MEDIUM_CUBE,
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 2589418111,
        idents: &[Medium, Corner, Round],
        stats: MEDIUM_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 1837286858,
        idents: &[Medium, Inner, Round],
        stats: MEDIUM_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 1538778047,
        idents: &[Medium, Edge, Slope],
        stats: MEDIUM_CUBE,
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 557016830,
        idents: &[Medium, Corner, Slope],
        stats: MEDIUM_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 495400745,
        idents: &[Medium, Inner, Slope],
        stats: MEDIUM_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 1352234106,
        idents: &[Medium, Cone],
        stats: MEDIUM_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 2686602224,
        idents: &[Medium, Pyramid],
        stats: MEDIUM_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 123901970,
        idents: &[Heavy, Cube],
        stats: HEAVY_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 1225928721,
        idents: &[Heavy, Edge],
        stats: HEAVY_CUBE,
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 222074329,
        idents: &[Heavy, Corner],
        stats: HEAVY_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 2577871627,
        idents: &[Heavy, Inner],
        stats: HEAVY_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 368051733,
        idents: &[Heavy, Edge, Round],
        stats: HEAVY_CUBE,
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 2148353926,
        idents: &[Heavy, Corner, Round],
        stats: HEAVY_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 665761850,
        idents: &[Heavy, Inner, Round],
        stats: HEAVY_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 1334508924,
        idents: &[Heavy, Edge, Slope],
        stats: HEAVY_CUBE,
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 3665280239,
        idents: &[Heavy, Corner, Slope],
        stats: HEAVY_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 2111062867,
        idents: &[Heavy, Inner, Slope],
        stats: HEAVY_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 3886136035,
        idents: &[Heavy, Cone],
        stats: HEAVY_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 4184274974,
        idents: &[Heavy, Pyramid],
        stats: HEAVY_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 1972224393,
        idents: &[Light, Cube],
        stats: LIGHT_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 4157111053,
        idents: &[Light, Edge],
        stats: LIGHT_CUBE,
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 1347255791,
        idents: &[Light, Corner],
        stats: LIGHT_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 1941682825,
        idents: &[Light, Inner],
        stats: LIGHT_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 1993583577,
        idents: &[Light, Edge, Round],
        stats: LIGHT_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 582052348,
        idents: &[Light, Corner, Round],
        stats: LIGHT_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 1578378954,
        idents: &[Light, Inner, Round],
        stats: LIGHT_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 3093572869,
        idents: &[Light, Edge, Slope],
        stats: LIGHT_CUBE,
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 3959877408,
        idents: &[Light, Corner, Slope],
        stats: LIGHT_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 2426642454,
        idents: &[Light, Inner, Slope],
        stats: LIGHT_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 1422041599,
        idents: &[Light, Cone],
        stats: LIGHT_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 885415789,
        idents: &[Light, Pyramid],
        stats: LIGHT_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 1576857358,
        idents: &[Compact, Cube],
        stats: TX_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 327960333,
        idents: &[Compact, Edge],
        stats: TX_CUBE,
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 126353766,
        idents: &[Compact, Corner],
        stats: TX_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 2378018821,
        idents: &[Compact, Inner],
        stats: TX_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 676446887,
        idents: &[Compact, Edge, Round],
        stats: TX_CUBE,
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 1728084091,
        idents: &[Compact, Corner, Round],
        stats: TX_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 925214026,
        idents: &[Compact, Inner, Round],
        stats: TX_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 1915435470,
        idents: &[Compact, Edge, Slope],
        stats: TX_CUBE,
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 1881278783,
        idents: &[Compact, Corner, Slope],
        stats: TX_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 1834966563,
        idents: &[Compact, Inner, Slope],
        stats: TX_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 1979640496,
        idents: &[Compact, Cone],
        stats: TX_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 979123471,
        idents: &[Compact, Pyramid],
        stats: TX_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 2392661891,
        idents: &[Medium, Cube, Carbon6, Letters],
        stats: MEDIUM_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 3367514907,
        idents: &[Medium, Cube, Carbon6, Logo],
        stats: MEDIUM_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 2077855120,
        idents: &[Medium, Cube, Cardboard],
        stats: MEDIUM_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 348383505,
        idents: &[Medium, Edge, Cardboard],
        stats: MEDIUM_CUBE,
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 150161008,
        idents: &[Neon, Cube],
        stats: MEDIUM_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 1183051379,
        idents: &[Neon, Edge],
        stats: MEDIUM_CUBE,
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 2333317284,
        idents: &[Neon, Corner],
        stats: MEDIUM_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 1616217456,
        idents: &[Neon, Inner],
        stats: MEDIUM_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 2623263862,
        idents: &[Neon, Edge, Round],
        stats: MEDIUM_CUBE,
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 3195590950,
        idents: &[Neon, Corner, Round],
        stats: MEDIUM_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 1774712137,
        idents: &[Neon, Inner, Round],
        stats: MEDIUM_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 3324063519,
        idents: &[Neon, Edge, Slope],
        stats: MEDIUM_CUBE,
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 3825345103,
        idents: &[Neon, Corner, Slope],
        stats: MEDIUM_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 868027936,
        idents: &[Neon, Inner, Slope],
        stats: MEDIUM_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 552682446,
        idents: &[Neon, Cone],
        stats: MEDIUM_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 3447434946,
        idents: &[Neon, Pyramid],
        stats: MEDIUM_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 3327812742,
        idents: &[Glass, Cube],
        stats: MEDIUM_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 445857183,
        idents: &[Glass, Inner],
        stats: MEDIUM_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 3066776961,
        idents: &[Glass, Inner, Round],
        stats: MEDIUM_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 4235300269,
        idents: &[Glass, Edge],
        stats: MEDIUM_CUBE,
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 3629216992,
        idents: &[Glass, Edge, Round],
        stats: MEDIUM_CUBE,
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 1094997616,
        idents: &[Glass, Corner],
        stats: MEDIUM_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 2380494106,
        idents: &[Glass, Corner, Round],
        stats: MEDIUM_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 183067052,
        idents: &[Retro, Cube],
        stats: MEDIUM_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 1150929327,
        idents: &[Retro, Edge],
        stats: MEDIUM_CUBE,
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 1387792676,
        idents: &[Retro, Corner],
        stats: MEDIUM_CUBE,
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 3985049715,
        idents: &[Retro, Inner],
        stats: MEDIUM_CUBE,
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 3226650954,
        idents: &[Helium],
        stats: Stats {
            tier: 3,
            cpu: 1,
            health: 5000,
            mass: 1.0,
        },
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 1818686721,
        idents: &[Rod],
        stats: ROD,
        connections: connections::ROD,
    },
    Cube {
        id: 1312806203,
        idents: &[Rod, Short],
        stats: ROD_SHORT,
        connections: connections::ROD_SHORT,
    },
    Cube {
        id: 3489647384,
        idents: &[Rod, Long],
        stats: ROD_LONG,
        connections: connections::ROD_LONG,
    },
    Cube {
        id: 2454932271,
        idents: &[Rod, Arc],
        stats: ROD,
        connections: connections::ROD_ARC,
    },
    Cube {
        id: 3568811832,
        idents: &[Rod, Arc, Short],
        stats: ROD_SHORT,
        connections: connections::ROD_ARC_SHORT,
    },
    Cube {
        id: 394503911,
        idents: &[Rod, Diagonal, TwoD],
        stats: Stats {
            tier: 2,
            cpu: 3,
            health: 5400,
            mass: 4.0,
        },
        connections: connections::ROD_DIAGONAL_2D,
    },
    Cube {
        id: 1316425218,
        idents: &[Rod, Diagonal, TwoD, Short],
        stats: ROD_SHORT,
        connections: connections::ROD_DIAGONAL_2D_SHORT,
    },
    Cube {
        id: 3705632066,
        idents: &[Rod, Diagonal, ThreeD],
        stats: ROD_LONG,
        connections: connections::ROD_DIAGONAL_3D,
    },
    Cube {
        id: 1750720388,
        idents: &[Rod, Plus],
        stats: ROD_LONG,
        connections: connections::ROD_PLUS,
    },
    Cube {
        id: 651695911,
        idents: &[Rod, Frame],
        stats: Stats {
            tier: 2,
            cpu: 8,
            health: 14400,
            mass: 8.0,
        },
        connections: connections::ROD_FRAME,
    },
    Cube {
        id: 4265819694,
        idents: &[Rod, Cross],
        stats: ROD_LONG,
        connections: connections::ROD_CROSS,
    },
    Cube {
        id: 3950939011,
        idents: &[Rod, Spring],
        stats: ROD,
        connections: connections::ROD,
    },
    Cube {
        id: 2828600518,
        idents: &[Rod, Spring, Short],
        stats: ROD_SHORT,
        connections: connections::ROD_SHORT,
    },
    Cube {
        id: 1051732041,
        idents: &[Rod, Spring, Long],
        stats: ROD_LONG,
        connections: connections::ROD_LONG,
    },
    Cube {
        id: 4262490755,
        idents: &[Strut],
        stats: STRUT,
        connections: connections::ROD,
    },
    Cube {
        id: 1648206355,
        idents: &[Strut, Short],
        stats: STRUT_SHORT,
        connections: connections::ROD_SHORT,
    },
    Cube {
        id: 3084940372,
        idents: &[Strut, Long],
        stats: STRUT_LONG,
        connections: connections::ROD_LONG,
    },
    Cube {
        id: 743433251,
        idents: &[Strut, Arc],
        stats: STRUT,
        connections: connections::ROD_ARC,
    },
    Cube {
        id: 4206005137,
        idents: &[Strut, Arc, Short],
        stats: STRUT_SHORT,
        connections: connections::ROD_ARC_SHORT,
    },
    Cube {
        id: 2126360617,
        idents: &[Strut, Slice],
        stats: STRUT,
        connections: connections::ROD_ARC,
    },
    Cube {
        id: 2608938070,
        idents: &[Strut, Slice, Short],
        stats: STRUT_SLICE_SHORT,
        connections: connections::ROD_ARC_SHORT,
    },
    Cube {
        id: 3999885169,
        idents: &[Strut, Ramp],
        stats: STRUT_RAMP,
        connections: connections::ROD_DIAGONAL_2D,
    },
    Cube {
        id: 3903776284,
        idents: &[Strut, Ramp, Short],
        stats: STRUT,
        connections: connections::ROD_DIAGONAL_2D_SHORT,
    },
    Cube {
        id: 14694903,
        idents: &[Strut, Diagonal, TwoD],
        stats: STRUT_RAMP,
        connections: connections::ROD_DIAGONAL_2D,
    },
    Cube {
        id: 3562021587,
        idents: &[Strut, Diagonal, TwoD, Short],
        stats: STRUT,
        connections: connections::ROD_DIAGONAL_2D_SHORT,
    },
    Cube {
        id: 2028802540,
        idents: &[Strut, Diagonal, ThreeD, Left],
        stats: STRUT_LONG,
        connections: connections::STRUT_DIAGONAL_3D_LEFT,
    },
    Cube {
        id: 2002864041,
        idents: &[Strut, Diagonal, ThreeD, Right],
        stats: STRUT_LONG,
        connections: connections::STRUT_DIAGONAL_3D_RIGHT,
    },
    Cube {
        id: 1319987665,
        idents: &[Strut, Diagonal, ThreeD, Left, Short],
        stats: STRUT_SLICE_SHORT,
        connections: connections::STRUT_DIAGONAL_3D_LEFT_SHORT,
    },
    Cube {
        id: 3923036903,
        idents: &[Strut, Diagonal, ThreeD, Right, Short],
        stats: STRUT_SLICE_SHORT,
        connections: connections::STRUT_DIAGONAL_3D_RIGHT_SHORT,
    },
    Cube {
        id: 511164535,
        idents: &[Strut, Plus],
        stats: Stats {
            tier: 3,
            cpu: 9,
            health: 21600,
            mass: 90.0,
        },
        connections: connections::ROD_PLUS,
    },
    Cube {
        id: 1521447199,
        idents: &[Strut, Skewed, Plus],
        stats: STRUT_SKEWED_PLUS,
        connections: connections::ROD_FRAME,
    },
    Cube {
        id: 787409845,
        idents: &[Strut, Cross],
        stats: STRUT_SKEWED_PLUS,
        connections: connections::ROD_CROSS,
    },
    Cube {
        id: 1810876016,
        idents: &[Wheel],
        stats: T0_WHEEL,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 932552373,
        idents: &[Wheel, Steering],
        stats: T0_WHEEL,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 4172855773,
        idents: &[Wheel],
        stats: T1_WHEEL,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 169839823,
        idents: &[Wheel, Steering],
        stats: T1_WHEEL,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2924196438,
        idents: &[Wheel],
        stats: T2_WHEEL,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2958657062,
        idents: &[Wheel, Steering],
        stats: T2_WHEEL,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3028949761,
        idents: &[Wheel],
        stats: T3_WHEEL,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1177366035,
        idents: &[Wheel, Steering],
        stats: T3_WHEEL,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1223384335,
        idents: &[Wheel],
        stats: T4_WHEEL,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 345048522,
        idents: &[Wheel, Steering],
        stats: T4_WHEEL,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2105254905,
        idents: &[Wheel, Cardboard],
        stats: T4_WHEEL,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 32187483,
        idents: &[Wheel, Cardboard, Steering],
        stats: T4_WHEEL,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3301162279,
        idents: &[Wheel],
        stats: T5_WHEEL,
        connections: connections::T5_WHEEL,
    },
    Cube {
        id: 48208435,
        idents: &[Wheel, Steering],
        stats: T5_WHEEL,
        connections: connections::T5_WHEEL,
    },
    Cube {
        id: 2305706992,
        idents: &[Wheel, Golden],
        stats: T5_WHEEL,
        connections: connections::T5_WHEEL,
    },
    Cube {
        id: 3546777424,
        idents: &[Wheel, Golden, Steering],
        stats: T5_WHEEL,
        connections: connections::T5_WHEEL,
    },
    Cube {
        id: 4050988656,
        idents: &[Tank, Track],
        stats: Stats {
            tier: 1,
            cpu: 45,
            health: 117000,
            mass: 2250.0,
        },
        connections: connections::T1_TANK_TRACK,
    },
    Cube {
        id: 1691117057,
        idents: &[Tank, Track],
        stats: Stats {
            tier: 2,
            cpu: 50,
            health: 132500,
            mass: 2500.0,
        },
        connections: connections::T2_TANK_TRACK,
    },
    Cube {
        id: 184017928,
        idents: &[Tank, Track],
        stats: Stats {
            tier: 3,
            cpu: 55,
            health: 148500,
            mass: 2750.0,
        },
        connections: connections::T3_TANK_TRACK,
    },
    Cube {
        id: 5728820,
        idents: &[Tank, Track],
        stats: Stats {
            tier: 4,
            cpu: 60,
            health: 165000,
            mass: 3000.0,
        },
        connections: connections::T4_TANK_TRACK,
    },
    Cube {
        id: 431289744,
        idents: &[Tank, Track],
        stats: Stats {
            tier: 5,
            cpu: 125,
            health: 350000,
            mass: 6250.0,
        },
        connections: connections::T5_TANK_TRACK,
    },
    Cube {
        id: 3066502430,
        idents: &[Mech, Leg],
        stats: Stats {
            tier: 0,
            cpu: 50,
            health: 115000,
            mass: 2000.0,
        },
        connections: connections::OPEN_CUBE,
    },
    Cube {
        id: 76192516,
        idents: &[Mech, Leg],
        stats: Stats {
            tier: 1,
            cpu: 62,
            health: 146320,
            mass: 2480.0,
        },
        connections: connections::OPEN_CUBE,
    },
    Cube {
        id: 1789991181,
        idents: &[Mech, Leg],
        stats: Stats {
            tier: 2,
            cpu: 74,
            health: 179080,
            mass: 2960.0,
        },
        connections: connections::T2_MECH_LEG,
    },
    Cube {
        id: 1611765553,
        idents: &[Mech, Leg],
        stats: Stats {
            tier: 3,
            cpu: 86,
            health: 213280,
            mass: 3440.0,
        },
        connections: connections::T3_MECH_LEG,
    },
    Cube {
        id: 2452579137,
        idents: &[Mech, Leg],
        stats: Stats {
            tier: 4,
            cpu: 98,
            health: 248920,
            mass: 3920.0,
        },
        connections: connections::T4_MECH_LEG,
    },
    Cube {
        id: 4140335767,
        idents: &[Mech, Leg],
        stats: T5_MECH_LEG,
        connections: connections::T5_MECH_LEG,
    },
    Cube {
        id: 3660410821,
        idents: &[Mech, Leg, Golden],
        stats: T5_MECH_LEG,
        connections: connections::T5_MECH_LEG,
    },
    Cube {
        id: 3345472872,
        idents: &[Sprinter, Leg],
        stats: Stats {
            tier: 3,
            cpu: 50,
            health: 70000,
            mass: 1750.0,
        },
        connections: connections::T2_MECH_LEG,
    },
    Cube {
        id: 1169080311,
        idents: &[Sprinter, Leg],
        stats: T4_SPRINTER_LEG,
        connections: connections::T4_MECH_LEG,
    },
    Cube {
        id: 2474131397,
        idents: &[Sprinter, Leg, Carbon6],
        stats: T4_SPRINTER_LEG,
        connections: connections::T4_MECH_LEG,
    },
    Cube {
        id: 127661231,
        idents: &[Insect, Leg],
        stats: T3_INSECT_LEG,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2671394050,
        idents: &[Insect, Leg, Bladed],
        stats: T3_INSECT_LEG,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1668760037,
        idents: &[Insect, Leg],
        stats: T4_INSECT_LEG,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 875393229,
        idents: &[Insect, Leg, Spider],
        stats: T4_INSECT_LEG,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 347391203,
        idents: &[Ski],
        stats: T3_SKI,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3080624795,
        idents: &[Ski, Steering],
        stats: T3_SKI,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1851713730,
        idents: &[Hover],
        stats: Stats {
            tier: 0,
            cpu: 20,
            health: 16000,
            mass: 100.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3170960304,
        idents: &[Hover],
        stats: Stats {
            tier: 1,
            cpu: 22,
            health: 21450,
            mass: 110.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1639979618,
        idents: &[Hover],
        stats: Stats {
            tier: 2,
            cpu: 24,
            health: 27600,
            mass: 120.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 465491285,
        idents: &[Hover],
        stats: Stats {
            tier: 3,
            cpu: 26,
            health: 34450,
            mass: 130.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3347041415,
        idents: &[Hover],
        stats: Stats {
            tier: 4,
            cpu: 28,
            health: 42000,
            mass: 140.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3835205776,
        idents: &[Hover],
        stats: T5_HOVER,
        connections: connections::T5_HOVER,
    },
    Cube {
        id: 2217471839,
        idents: &[Hover, Golden],
        stats: T5_HOVER,
        connections: connections::T5_HOVER,
    },
    Cube {
        id: 3947193935,
        idents: &[Rotor],
        stats: Stats {
            tier: 2,
            cpu: 30,
            health: 36000,
            mass: 150.0,
        },
        connections: connections::OPEN_CUBE,
    },
    Cube {
        id: 3789940851,
        idents: &[Rotor],
        stats: Stats {
            tier: 3,
            cpu: 45,
            health: 72000,
            mass: 225.0,
        },
        connections: connections::OPEN_CUBE,
    },
    Cube {
        id: 1802864184,
        idents: &[Rotor],
        stats: Stats {
            tier: 4,
            cpu: 60,
            health: 120000,
            mass: 300.0,
        },
        connections: connections::OPEN_CUBE,
    },
    Cube {
        id: 3980928804,
        idents: &[Wing],
        stats: Stats {
            tier: 2,
            cpu: 22,
            health: 15400,
            mass: 22.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2205394221,
        idents: &[Wing],
        stats: Stats {
            tier: 3,
            cpu: 24,
            health: 19200,
            mass: 24.0,
        },
        connections: connections::T3_WING,
    },
    Cube {
        id: 2312317713,
        idents: &[Wing],
        stats: T4_WING,
        connections: connections::T4_WING,
    },
    Cube {
        id: 2865954846,
        idents: &[Wing, Bat],
        stats: T4_WING,
        connections: connections::T4_WING,
    },
    Cube {
        id: 2464192350,
        idents: &[Wing],
        stats: T5_WING,
        connections: connections::T5_WING,
    },
    Cube {
        id: 2863842421,
        idents: &[Wing, Bat],
        stats: T5_WING,
        connections: connections::T5_WING,
    },
    Cube {
        id: 2369778644,
        idents: &[Rudder],
        stats: Stats {
            tier: 2,
            cpu: 19,
            health: 11400,
            mass: 19.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3808719325,
        idents: &[Rudder],
        stats: Stats {
            tier: 3,
            cpu: 21,
            health: 14700,
            mass: 21.0,
        },
        connections: connections::T3_WING,
    },
    Cube {
        id: 3919904737,
        idents: &[Rudder],
        stats: T4_RUDDER,
        connections: connections::T4_RUDDER,
    },
    Cube {
        id: 2404311559,
        idents: &[Rudder, Bat],
        stats: T4_RUDDER,
        connections: connections::T4_RUDDER,
    },
    Cube {
        id: 2608683011,
        idents: &[Rudder],
        stats: T5_RUDDER,
        connections: connections::T5_RUDDER,
    },
    Cube {
        id: 2928920064,
        idents: &[Rudder, Bat],
        stats: T5_RUDDER,
        connections: connections::T5_RUDDER,
    },
    Cube {
        id: 844778245,
        idents: &[Thruster],
        stats: Stats {
            tier: 1,
            cpu: 10,
            health: 5000,
            mass: 8.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2498076128,
        idents: &[Thruster],
        stats: Stats {
            tier: 2,
            cpu: 12,
            health: 7800,
            mass: 9.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3764614578,
        idents: &[Thruster],
        stats: Stats {
            tier: 3,
            cpu: 14,
            health: 11200,
            mass: 11.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3938712462,
        idents: &[Thruster],
        stats: Stats {
            tier: 4,
            cpu: 16,
            health: 15200,
            mass: 12.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2969949587,
        idents: &[Thruster],
        stats: T5_THRUSTER,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 944347375,
        idents: &[Thruster, Carbon6],
        stats: T5_THRUSTER,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3286209719,
        idents: &[Propeller],
        stats: Stats {
            tier: 3,
            cpu: 50,
            health: 65000,
            mass: 250.0,
        },
        connections: connections::T3_PROP,
    },
    Cube {
        id: 2784549582,
        idents: &[Propeller],
        stats: Stats {
            tier: 4,
            cpu: 100,
            health: 130000,
            mass: 500.0,
        },
        connections: connections::T4_PROP,
    },
    Cube {
        id: 3809895395,
        idents: &[Laser],
        stats: T0_LASER,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3402350870,
        idents: &[Laser, Front],
        stats: T0_LASER,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3178463503,
        idents: &[Laser],
        stats: T1_LASER,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2494440442,
        idents: &[Laser, Front],
        stats: T1_LASER,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2007965780,
        idents: &[Laser],
        stats: T2_LASER,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1584562849,
        idents: &[Laser, Front],
        stats: T2_LASER,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3064235218,
        idents: &[Laser],
        stats: T3_LASER,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2675516967,
        idents: &[Laser, Front],
        stats: T3_LASER,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2088248713,
        idents: &[Laser],
        stats: T4_LASER,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1436911484,
        idents: &[Laser, Front],
        stats: T4_LASER,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3632954889,
        idents: &[Laser, Carbon6],
        stats: T4_LASER,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3209000021,
        idents: &[Laser],
        stats: T5_LASER,
        connections: connections::T5_LASER,
    },
    Cube {
        id: 1199458351,
        idents: &[Laser, Carbon6],
        stats: T5_LASER,
        connections: connections::T5_LASER,
    },
    Cube {
        id: 3851710054,
        idents: &[Laser, Golden],
        stats: T5_LASER,
        connections: connections::T5_LASER,
    },
    Cube {
        id: 2655409492,
        idents: &[Plasma],
        stats: Stats {
            tier: 0,
            cpu: 14,
            health: 14000,
            mass: 28.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1293532710,
        idents: &[Plasma],
        stats: Stats {
            tier: 1,
            cpu: 18,
            health: 18000,
            mass: 36.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2446895092,
        idents: &[Plasma],
        stats: Stats {
            tier: 2,
            cpu: 22,
            health: 22000,
            mass: 44.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3953551555,
        idents: &[Plasma],
        stats: Stats {
            tier: 3,
            cpu: 26,
            health: 26000,
            mass: 52.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 929526033,
        idents: &[Plasma],
        stats: T4_PLASMA,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1593059007,
        idents: &[Plasma, Carbon6],
        stats: T4_PLASMA,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 171448327,
        idents: &[Plasma],
        stats: T5_PLASMA,
        connections: connections::T5_LASER,
    },
    Cube {
        id: 2943284935,
        idents: &[Plasma, Carbon6],
        stats: T5_PLASMA,
        connections: connections::T5_LASER,
    },
    Cube {
        id: 3594602569,
        idents: &[Plasma, Golden],
        stats: T5_PLASMA,
        connections: connections::T5_LASER,
    },
    Cube {
        id: 1640043587,
        idents: &[Rail],
        stats: Stats {
            tier: 1,
            cpu: 18,
            health: 18000,
            mass: 36.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2870850840,
        idents: &[Rail],
        stats: Stats {
            tier: 2,
            cpu: 22,
            health: 22000,
            mass: 44.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1779831198,
        idents: &[Rail],
        stats: Stats {
            tier: 3,
            cpu: 26,
            health: 26000,
            mass: 52.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2697638085,
        idents: &[Rail],
        stats: Stats {
            tier: 4,
            cpu: 30,
            health: 36000,
            mass: 60.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2083340214,
        idents: &[Rail],
        stats: T5_RAIL,
        connections: connections::T4_PROP,
    },
    Cube {
        id: 447003593,
        idents: &[Rail, Golden],
        stats: T5_RAIL,
        connections: connections::T4_PROP,
    },
    Cube {
        id: 1761903423,
        idents: &[Nano],
        stats: Stats {
            tier: 2,
            cpu: 18,
            health: 18000,
            mass: 36.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1671695619,
        idents: &[Nano],
        stats: Stats {
            tier: 3,
            cpu: 22,
            health: 22000,
            mass: 44.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2364175165,
        idents: &[Nano],
        stats: Stats {
            tier: 4,
            cpu: 26,
            health: 31200,
            mass: 52.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2017654588,
        idents: &[Tesla],
        stats: Stats {
            tier: 2,
            cpu: 16,
            health: 16000,
            mass: 32.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3479123512,
        idents: &[Tesla],
        stats: Stats {
            tier: 3,
            cpu: 20,
            health: 28000,
            mass: 40.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2703353899,
        idents: &[Tesla],
        stats: Stats {
            tier: 4,
            cpu: 24,
            health: 43200,
            mass: 48.0,
        },
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 48150119,
        idents: &[Seeker],
        stats: Stats {
            tier: 3,
            cpu: 100,
            health: 100000,
            mass: 200.0,
        },
        connections: connections::OPEN_CUBE,
    },
    Cube {
        id: 350778877,
        idents: &[Seeker],
        stats: Stats {
            tier: 4,
            cpu: 135,
            health: 135000,
            mass: 280.0,
        },
        connections: connections::OPEN_CUBE,
    },
    Cube {
        id: 2419806013,
        idents: &[Seeker],
        stats: T5_SEEKER,
        connections: connections::T5_SEEKER,
    },
    Cube {
        id: 3832533619,
        idents: &[Seeker, Golden],
        stats: T5_SEEKER,
        connections: connections::T5_SEEKER,
    },
    Cube {
        id: 3437168371,
        idents: &[Seeker, Firework],
        stats: T5_SEEKER,
        connections: connections::T5_SEEKER,
    },
    Cube {
        id: 3947892032,
        idents: &[AeroFlak],
        stats: Stats {
            tier: 4,
            cpu: 150,
            health: 150000,
            mass: 300.0,
        },
        connections: connections::OPEN_CUBE,
    },
    Cube {
        id: 73607413,
        idents: &[AeroFlak],
        stats: Stats {
            tier: 5,
            cpu: 300,
            health: 300000,
            mass: 600.0,
        },
        connections: connections::T5_AEROFLAK,
    },
    Cube {
        id: 1318129004,
        idents: &[Chaingun],
        stats: Stats {
            tier: 4,
            cpu: 140,
            health: 182000,
            mass: 330.0,
        },
        connections: connections::OPEN_CUBE,
    },
    Cube {
        id: 1795842454,
        idents: &[Chaingun],
        stats: T5_CHAIN,
        connections: connections::T5_CHAINGUN,
    },
    Cube {
        id: 194724715,
        idents: &[Chaingun, Golden],
        stats: T5_CHAIN,
        connections: connections::T5_CHAINGUN,
    },
    Cube {
        id: 3541341953,
        idents: &[Ion],
        stats: Stats {
            tier: 4,
            cpu: 120,
            health: 120000,
            mass: 290000.0,
        },
        connections: connections::OPEN_CUBE,
    },
    Cube {
        id: 1184153980,
        idents: &[Ion],
        stats: T5_ION,
        connections: connections::T5_SEEKER,
    },
    Cube {
        id: 2295212085,
        idents: &[Ion, Cardboard],
        stats: T5_ION,
        connections: connections::T5_SEEKER,
    },
    Cube {
        id: 105639928,
        idents: &[Ion, Golden],
        stats: T5_ION,
        connections: connections::T5_SEEKER,
    },
    Cube {
        id: 1885850433,
        idents: &[Mortar],
        stats: T5_MORTAR,
        connections: connections::T4_PROP,
    },
    Cube {
        id: 2256680330,
        idents: &[Mortar, Golden],
        stats: T5_MORTAR,
        connections: connections::T4_PROP,
    },
    Cube {
        id: 3087866254,
        idents: &[Mortar, Egg],
        stats: T5_MORTAR,
        connections: connections::T4_PROP,
    },
    Cube {
        id: 1692614192,
        idents: &[Module, Power],
        stats: Stats {
            tier: 3,
            cpu: 150,
            health: 123750,
            mass: 450.0,
        },
        connections: connections::POWER,
    },
    Cube {
        id: 3263542661,
        idents: &[Module, Disc],
        stats: Stats {
            tier: 3,
            cpu: 205,
            health: 169125,
            mass: 615.0,
        },
        connections: connections::DISC,
    },
    Cube {
        id: 3987245394,
        idents: &[Module, Blink],
        stats: Stats {
            tier: 3,
            cpu: 200,
            health: 165000,
            mass: 600.0,
        },
        connections: connections::BLINK,
    },
    Cube {
        id: 4230376397,
        idents: &[Module, Ghost],
        stats: Stats {
            tier: 3,
            cpu: 180,
            health: 148500,
            mass: 690.0,
        },
        connections: connections::GHOST,
    },
    Cube {
        id: 658488560,
        idents: &[Module, EMP],
        stats: Stats {
            tier: 3,
            cpu: 175,
            health: 144375,
            mass: 525.0,
        },
        connections: connections::GHOST,
    },
    Cube {
        id: 344080878,
        idents: &[Module, Windowmaker],
        stats: Stats {
            tier: 3,
            cpu: 100,
            health: 280000,
            mass: 300.0,
        },
        connections: connections::T4_PROP,
    },
    Cube {
        id: 1264237222,
        idents: &[ElectroShield, A, Left],
        stats: T2_EP_A,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2965095316,
        idents: &[ElectroShield, A, Right],
        stats: T2_EP_A,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2457663915,
        idents: &[ElectroShield, B, Left],
        stats: T2_EP_B,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1771929753,
        idents: &[ElectroShield, B, Right],
        stats: T2_EP_B,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 627128495,
        idents: &[ElectroShield, C, Left],
        stats: T2_EP_C,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3732947357,
        idents: &[ElectroShield, C, Right],
        stats: T2_EP_C,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 540114865,
        idents: &[ElectroShield, A, Left],
        stats: T3_EP_A,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3687843459,
        idents: &[ElectroShield, A, Right],
        stats: T3_EP_A,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2536292021,
        idents: &[ElectroShield, B, Left],
        stats: T3_EP_B,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1825425287,
        idents: &[ElectroShield, B, Right],
        stats: T3_EP_B,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 726724318,
        idents: &[ElectroShield, B, Left, Spiked],
        stats: T3_EP_B,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1361801646,
        idents: &[ElectroShield, B, Right, Spiked],
        stats: T3_EP_B,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3248328684,
        idents: &[ElectroShield, B, Left, Football],
        stats: T3_EP_B,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3152416924,
        idents: &[ElectroShield, B, Right, Football],
        stats: T3_EP_B,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1309343160,
        idents: &[ElectroShield, C, Left],
        stats: T3_EP_C,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3052110986,
        idents: &[ElectroShield, C, Right],
        stats: T3_EP_C,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 4179049660,
        idents: &[ElectroShield, A, Left],
        stats: T4_EP_A,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 49694094,
        idents: &[ElectroShield, A, Right],
        stats: T4_EP_A,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1152021380,
        idents: &[ElectroShield, B, Left],
        stats: T4_EP_B,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3209366198,
        idents: &[ElectroShield, B, Right],
        stats: T4_EP_B,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 4088902272,
        idents: &[ElectroShield, C, Left],
        stats: T4_EP_C,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 139905970,
        idents: &[ElectroShield, C, Right],
        stats: T4_EP_C,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3823106277,
        idents: &[ElectroShield, A, Left],
        stats: T5_EP_A,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1889152405,
        idents: &[ElectroShield, A, Right],
        stats: T5_EP_A,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3001609712,
        idents: &[ElectroShield, B, Left],
        stats: T5_EP_B,
        connections: connections::TX_T5_B_LEFT,
    },
    Cube {
        id: 3365106304,
        idents: &[ElectroShield, B, Right],
        stats: T5_EP_B,
        connections: connections::TX_T5_B_RIGHT,
    },
    Cube {
        id: 991790322,
        idents: &[ElectroShield, C, Left],
        stats: T5_EP_B,
        connections: connections::TX_T5_C,
    },
    Cube {
        id: 1097210754,
        idents: &[ElectroShield, C, Right],
        stats: T5_EP_B,
        connections: connections::TX_T5_C,
    },
    Cube {
        id: 988387568,
        idents: &[Medal, Bronze],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1551110695,
        idents: &[Medal, Silver],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2514755079,
        idents: &[Medal, Golden],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2820370110,
        idents: &[Altimeter],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3531325242,
        idents: &[Speedometer],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1367205243,
        idents: &[Headlamp],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2782198650,
        idents: &[Robot, Name, Banner],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2843561040,
        idents: &[PilotSeat, Cray],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1427441647,
        idents: &[PilotSeat, Gene],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2325944783,
        idents: &[PilotSeat, Mega],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2743059293,
        idents: &[PilotSeat, Retro],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1061758673,
        idents: &[PilotSeat, Bunny],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2484686408,
        idents: &[BubbleBlower],
        stats: COSMETIC_3CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 346272734,
        idents: &[Balloon],
        stats: COSMETIC_3CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3610833057,
        idents: &[Vapor, Trail, Single],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1810319153,
        idents: &[Vapor, Trail, Firework],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3297983510,
        idents: &[Vapor, Trail, Snowflake],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3137009174,
        idents: &[Vapor, Trail, Flowers],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 236893100,
        idents: &[Vapor, Trail, Twin],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1704793982,
        idents: &[Eyeball, Vigilant, Left],
        stats: COSMETIC_3CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3558508089,
        idents: &[Eyeball, Vigilant, Right],
        stats: COSMETIC_3CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 767825476,
        idents: &[Eyeball, Cat, Left],
        stats: COSMETIC_3CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 683945560,
        idents: &[Eyeball, Cat, Right],
        stats: COSMETIC_3CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1609561971,
        idents: &[Eyeball, Cyborg],
        stats: COSMETIC_3CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 108680691,
        idents: &[Spike, Pin],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 955529870,
        idents: &[Spike, Needle],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 688123941,
        idents: &[Spike, Dagger],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1991524976,
        idents: &[Spike, Claw],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 138993629,
        idents: &[Present, Small],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 568116457,
        idents: &[Present, Large],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2183972675,
        idents: &[Fairy, Lights],
        stats: COSMETIC_1CPU,
        connections: connections::ROD_LONG,
    },
    Cube {
        id: 149580437,
        idents: &[Exhaust, Blower],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3489925411,
        idents: &[Exhaust, Stack, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1679554409,
        idents: &[Exhaust, Stack, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2707915966,
        idents: &[Rectifier, Small],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3083647184,
        idents: &[Rectifier, Medium],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2297667466,
        idents: &[Rectifier, Large],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 198393630,
        idents: &[Jammer, Small],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3106939802,
        idents: &[Jammer, Medium],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 575087658,
        idents: &[Jammer, Large],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3782348815,
        idents: &[Radar, Small],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2115785469,
        idents: &[Radar, Medium],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3370519355,
        idents: &[Radar, Large],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2330463949,
        idents: &[Receiver, Small],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2312945847,
        idents: &[Receiver, Medium],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2742818297,
        idents: &[Receiver, Large],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 757806708,
        idents: &[Holoflag, Top, OneHundred],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 354715741,
        idents: &[Holoflag, Argentina],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 664490215,
        idents: &[Holoflag, Australia],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1768151340,
        idents: &[Holoflag, Belarus],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1798659636,
        idents: &[Holoflag, Belgium],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2243089188,
        idents: &[Holoflag, Brazil],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1385335421,
        idents: &[Holoflag, Canada],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3910121366,
        idents: &[Holoflag, China],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 932603502,
        idents: &[Holoflag, Danish],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2373812955,
        idents: &[Holoflag, France],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2596479965,
        idents: &[Holoflag, Germany],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1796059677,
        idents: &[Holoflag, Iceland],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3447406366,
        idents: &[Holoflag, Ireland],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 4166201860,
        idents: &[Holoflag, Italy],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2020064218,
        idents: &[Holoflag, Japan],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3330651823,
        idents: &[Holoflag, Kazakhstan],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 88051662,
        idents: &[Holoflag, Netherlands],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2855651676,
        idents: &[Holoflag, New, Zealand],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2730691972,
        idents: &[Holoflag, Poland],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 4065608086,
        idents: &[Holoflag, Russia],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 460733827,
        idents: &[Holoflag, South, Korea],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3114896263,
        idents: &[Holoflag, Spain],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1786715320,
        idents: &[Holoflag, Sweden],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 4070927930,
        idents: &[Holoflag, Turkish],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 4175295691,
        idents: &[Holoflag, UK],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3559676835,
        idents: &[Holoflag, USA],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 68277216,
        idents: &[Holoflag, Ukraine],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1958111892,
        idents: &[Holoflag, Robocraft],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 4200145444,
        idents: &[Holoflag, Dev, Supporter],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3370765914,
        idents: &[Holoflag, Dev, Supporter, Golden],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3996493866,
        idents: &[Holoflag, Three, Years],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 507555577,
        idents: &[Holoflag, Four, Years],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3210649465,
        idents: &[Holoflag, Five, Years],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2537158398,
        idents: &[Holoflag, Six, Years],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 4124816651,
        idents: &[Holoflag, Pirate],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1042028710,
        idents: &[Holoflag, Nyan, Cray],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1607707925,
        idents: &[Holoflag, Able, Gamers, Charity],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1020482017,
        idents: &[Holoflag, Overwolf],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1039436321,
        idents: &[Holoflag, Curse],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3266439135,
        idents: &[Holoflag, Flowers],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 4079824960,
        idents: &[Holoflag, Yogscast],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2345098755,
        idents: &[Holoflag, Alienware],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 862329300,
        idents: &[Holoflag, ChronoGG],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3024755361,
        idents: &[Holoflag, Humble, Bundle],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 216301182,
        idents: &[Holoflag, TwoThousandNineTeen],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2412453676,
        idents: &[Holoflag, Candy, Cane],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 327459709,
        idents: &[Holoflag, Snowflake],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2990880144,
        idents: &[Holoflag, Santa, Cray],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3058601525,
        idents: &[Holoflag, Robopass, Season, One],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3411783298,
        idents: &[Holoflag, Robopass, Season, Two],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2663263630,
        idents: &[Holoflag, Bunny],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3131616996,
        idents: &[Holoflag, EasterEgg],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1354711861,
        idents: &[Cockpit, Front, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 717544517,
        idents: &[Cockpit, Front, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2711624874,
        idents: &[Cockpit, Mid, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3688596442,
        idents: &[Cockpit, Mid, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2530225659,
        idents: &[Cockpit, Top, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3970675339,
        idents: &[Cockpit, Top, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 604924218,
        idents: &[Sabretooth, Ear, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1584789066,
        idents: &[Sabretooth, Ear, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1217647354,
        idents: &[Sabretooth, Face, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 854085002,
        idents: &[Sabretooth, Face, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2906446214,
        idents: &[Sabretooth, Jaw, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3611772662,
        idents: &[Sabretooth, Jaw, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 670637166,
        idents: &[Football, Face, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1568834334,
        idents: &[Football, Face, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1762483815,
        idents: &[Football, Guard, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 326549783,
        idents: &[Football, Guard, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 438539744,
        idents: &[Football, Helmet, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1616382608,
        idents: &[Football, Helmet, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1797279669,
        idents: &[Eagle, Face, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 291229893,
        idents: &[Eagle, Face, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2829693731,
        idents: &[Eagle, Feathers, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3536989267,
        idents: &[Eagle, Feathers, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 38268230,
        idents: &[Eagle, Neck, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2017227318,
        idents: &[Eagle, Neck, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 699401910,
        idents: &[Alienware, Ear, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3136672710,
        idents: &[Alienware, Ear, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3969591264,
        idents: &[Alienware, Head, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2145317520,
        idents: &[Alienware, Head, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1314825875,
        idents: &[Alienware, Eye, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3709563875,
        idents: &[Alienware, Eye, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2075163291,
        idents: &[Scary, Eyes, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 30139883,
        idents: &[Scary, Eyes, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2915102202,
        idents: &[Scary, Horn, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3619385994,
        idents: &[Scary, Horn, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 665365647,
        idents: &[Scary, Jaw, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1574122495,
        idents: &[Scary, Jaw, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1390502124,
        idents: &[Ninja, Bandana, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3248797084,
        idents: &[Ninja, Bandana, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 989441293,
        idents: &[Ninja, Head, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2847701117,
        idents: &[Ninja, Head, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2503490917,
        idents: &[Ninja, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 108914709,
        idents: &[Ninja, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 799302218,
        idents: &[Trex, Head, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 981970925,
        idents: &[Trex, Head, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 4069461414,
        idents: &[Trex, Nose, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 669458802,
        idents: &[Trex, Nose, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3618891370,
        idents: &[Trex, Jaw, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3670645745,
        idents: &[Trex, Jaw, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1839067602,
        idents: &[Honeydew, Beard, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 400977570,
        idents: &[Honeydew, Beard, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1108501665,
        idents: &[Honeydew, Face, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 946469841,
        idents: &[Honeydew, Face, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1268201954,
        idents: &[Honeydew, Horns, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 837592722,
        idents: &[Honeydew, Horns, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2479898264,
        idents: &[Mech7, Ear, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3920339432,
        idents: &[Mech7, Ear, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 451055140,
        idents: &[Mech7, Jaw, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1620644180,
        idents: &[Mech7, Jaw, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3561288328,
        idents: &[Mech7, Nose, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2923392504,
        idents: &[Mech7, Nose, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 4200383087,
        idents: &[Overwolf, Ear, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2150063391,
        idents: &[Overwolf, Ear, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3060186489,
        idents: &[Overwolf, Eye, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3424461321,
        idents: &[Overwolf, Eye, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1936618195,
        idents: &[Overwolf, Jaw, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 152432035,
        idents: &[Overwolf, Jaw, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 4155139139,
        idents: &[Rhino8, Ear, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2379348787,
        idents: &[Rhino8, Ear, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2123945215,
        idents: &[Rhino8, Jaw, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 82004879,
        idents: &[Rhino8, Jaw, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1554012392,
        idents: &[Rhino8, Nose, Left],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 651937688,
        idents: &[Rhino8, Nose, Right],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1301942811,
        idents: &[League, Badge, Bronze],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 4157599523,
        idents: &[League, Badge, Silver],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1893098069,
        idents: &[League, Badge, Golden],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 608517870,
        idents: &[League, Badge, Diamond],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2046961989,
        idents: &[League, Badge, Protonium],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1988022506,
        idents: &[League, Badge, Protonium, Five, Stars],
        stats: COSMETIC_1CPU,
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3795589065,
        idents: &[Armored, Cube],
        stats: RC14_CUBE(1),
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 3523011797,
        idents: &[Armored, Cube],
        stats: RC14_CUBE(2),
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 1428281037,
        idents: &[Armored, Cube],
        stats: RC14_CUBE(3),
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 148547544,
        idents: &[Armored, Cube],
        stats: RC14_CUBE(4),
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 3902577141,
        idents: &[Armored, Cube],
        stats: RC14_CUBE(5),
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 3217516252,
        idents: &[Armored, Cube],
        stats: RC14_CUBE(6),
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 1602312433,
        idents: &[Armored, Cube],
        stats: RC14_CUBE(7),
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 3130466754,
        idents: &[Armored, Cube],
        stats: RC14_CUBE(8),
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 936473914,
        idents: &[Armored, Cube],
        stats: RC14_CUBE(9),
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 1661086464,
        idents: &[Armored, Inner],
        stats: RC14_CUBE(1),
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 140476067,
        idents: &[Armored, Inner],
        stats: RC14_CUBE(2),
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 3558861316,
        idents: &[Armored, Inner],
        stats: RC14_CUBE(3),
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 3514383790,
        idents: &[Armored, Inner],
        stats: RC14_CUBE(4),
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 1772266812,
        idents: &[Armored, Inner],
        stats: RC14_CUBE(5),
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 1717894314,
        idents: &[Armored, Inner],
        stats: RC14_CUBE(6),
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 3737084984,
        idents: &[Armored, Inner],
        stats: RC14_CUBE(7),
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 1664400308,
        idents: &[Armored, Inner],
        stats: RC14_CUBE(8),
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 2704279673,
        idents: &[Armored, Inner],
        stats: RC14_CUBE(9),
        connections: connections::FULL_CUBE,
    },
    Cube {
        id: 3133051372,
        idents: &[Armored, Prism],
        stats: RC14_CUBE(1),
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 3521365071,
        idents: &[Armored, Prism],
        stats: RC14_CUBE(2),
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 228806888,
        idents: &[Armored, Prism],
        stats: RC14_CUBE(3),
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 147162946,
        idents: &[Armored, Prism],
        stats: RC14_CUBE(4),
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 2954764240,
        idents: &[Armored, Prism],
        stats: RC14_CUBE(5),
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 3218622022,
        idents: &[Armored, Prism],
        stats: RC14_CUBE(6),
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 117694164,
        idents: &[Armored, Prism],
        stats: RC14_CUBE(7),
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 3129475416,
        idents: &[Armored, Prism],
        stats: RC14_CUBE(8),
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 3159626035,
        idents: &[Armored, Prism],
        stats: RC14_CUBE(9),
        connections: connections::HALF_CUBE,
    },
    Cube {
        id: 3288882012,
        idents: &[Armored, Tetra],
        stats: RC14_CUBE(1),
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 2941587199,
        idents: &[Armored, Tetra],
        stats: RC14_CUBE(2),
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 1930795608,
        idents: &[Armored, Tetra],
        stats: RC14_CUBE(3),
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 1987268082,
        idents: &[Armored, Tetra],
        stats: RC14_CUBE(4),
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 3467171168,
        idents: &[Armored, Tetra],
        stats: RC14_CUBE(5),
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 3245247734,
        idents: &[Armored, Tetra],
        stats: RC14_CUBE(6),
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 2041910372,
        idents: &[Armored, Tetra],
        stats: RC14_CUBE(7),
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 3292449768,
        idents: &[Armored, Tetra],
        stats: RC14_CUBE(8),
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 3265763652,
        idents: &[Armored, Tetra],
        stats: RC14_CUBE(9),
        connections: connections::THIRD_CUBE,
    },
    Cube {
        id: 2866040116,
        idents: &[Wheel],
        stats: RC14_WHEEL(1),
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3769194911,
        idents: &[Wheel, Steering],
        stats: RC14_WHEEL(1),
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 299251682,
        idents: &[Wheel],
        stats: RC14_WHEEL(2),
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3158825472,
        idents: &[Wheel, Steering],
        stats: RC14_WHEEL(2),
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 182430853,
        idents: &[Wheel],
        stats: RC14_WHEEL(3),
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1526222854,
        idents: &[Wheel, Steering],
        stats: RC14_WHEEL(3),
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2137276546,
        idents: &[Wheel],
        stats: RC14_WHEEL(4),
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3539316064,
        idents: &[Wheel, Steering],
        stats: RC14_WHEEL(4),
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2632952818,
        idents: &[Wheel],
        stats: RC14_WHEEL(5),
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3435681649,
        idents: &[Wheel, Steering],
        stats: RC14_WHEEL(5),
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3372100958,
        idents: &[Wheel],
        stats: RC14_WHEEL(6),
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1701822652,
        idents: &[Wheel, Steering],
        stats: RC14_WHEEL(6),
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1065013100,
        idents: &[Wheel],
        stats: RC14_WHEEL(7),
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 1868379119,
        idents: &[Wheel, Steering],
        stats: RC14_WHEEL(7),
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2734731074,
        idents: &[Wheel],
        stats: RC14_WHEEL(8),
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 244400800,
        idents: &[Wheel, Steering],
        stats: RC14_WHEEL(8),
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 2840250395,
        idents: &[Wheel],
        stats: RC14_WHEEL(9),
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 4184683672,
        idents: &[Wheel, Steering],
        stats: RC14_WHEEL(9),
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 345686686,
        idents: &[Wheel],
        stats: RC14_WHEEL(10),
        connections: connections::ONE_CONNECTION,
    },
    Cube {
        id: 3104518012,
        idents: &[Wheel, Steering],
        stats: RC14_WHEEL(10),
        connections: connections::ONE_CONNECTION,
    },
];
