use crate::connections::ConnectionsBuilder;
use crate::cubes::simple::HashCubes;
use crate::position::CubePosition;
use bobocraft_cubes::Cube;
use bobocraft_format::Rotation;

fn verify(
    (cube_str, (x, y, z), orientation): (&str, (u8, u8, u8), u8),
    placements: &[(u8, u8, u8)],
) {
    let mut connections_builder = ConnectionsBuilder::with_capacity(placements.len());
    let mut cubes = HashCubes::with_capacity(placements.len());

    let rotation = Rotation::try_from(orientation).unwrap();
    let cube: &'static Cube = cube_str.parse().unwrap();

    let position = CubePosition { x, y, z };
    connections_builder.push(cube, position, rotation).unwrap();
    cubes.push(cube, position).unwrap();

    let cube: &'static Cube = "medium_cube".parse().unwrap();

    for &(x, y, z) in placements {
        let position = CubePosition { x, y, z };
        connections_builder
            .push(cube, position, Rotation { x: 0, y: 0, z: 0 })
            .unwrap();
        cubes.push(cube, position).unwrap();
    }

    let connections = connections_builder.build();

    let cube = cubes[&position];

    let connections = &connections[&position];

    assert_eq!(cube.connections.len(), connections.len());
}

#[test]
fn connections_rod() {
    verify(("rod", (0, 0, 1), 0), &[(0, 0, 0), (0, 0, 4)]);
}

#[test]
fn connections_rod_short() {
    verify(("rod_short", (0, 0, 1), 0), &[(0, 0, 3), (0, 0, 0)]);
}

#[test]
fn connections_rod_long() {
    verify(("rod_long", (0, 0, 1), 0), &[(0, 0, 0), (0, 0, 6)]);
}

#[test]
fn connections_rod_arc() {
    verify(("rod_arc", (0, 3, 1), 4), &[(0, 0, 3), (0, 3, 0)]);
}

#[test]
fn connections_rod_arc_short() {
    verify(("rod_arc_short", (0, 2, 1), 4), &[(0, 2, 0), (0, 0, 2)]);
}

#[test]
fn connections_rod_diagonal_2d() {
    verify(("rod_diagonal_2d", (0, 2, 1), 4), &[(0, 0, 4), (0, 2, 0)]);
}

#[test]
fn connections_rod_diagonal_2d_short() {
    verify(
        ("rod_diagonal_2d_short", (0, 1, 1), 4),
        &[(0, 0, 3), (0, 1, 0)],
    );
}

#[test]
fn connections_rod_diagonal_3d() {
    verify(("rod_diagonal_3d", (2, 2, 1), 4), &[(0, 0, 4), (2, 2, 0)]);
}

#[test]
fn connections_rod_plus() {
    verify(
        ("rod_plus", (2, 0, 1), 0),
        &[(0, 0, 2), (2, 0, 4), (4, 0, 2), (2, 0, 0)],
    );
}

#[test]
fn connections_rod_frame() {
    verify(
        ("rod_frame", (0, 1, 1), 18),
        &[(0, 4, 1), (0, 0, 3), (0, 1, 0), (0, 3, 4)],
    );
}

#[test]
fn connections_rod_cross() {
    verify(
        ("rod_cross", (0, 0, 1), 18),
        &[(0, 0, 0), (0, 2, 4), (0, 0, 4), (0, 2, 0)],
    );
}

#[test]
fn connections_strut_diagonal_3d_left() {
    verify(
        ("strut_diagonal_3d_left", (0, 0, 1), 18),
        &[(2, 2, 4), (0, 0, 0)],
    );
}

#[test]
fn connections_strut_diagonal_3d_right() {
    verify(
        ("strut_diagonal_3d_right", (2, 0, 1), 4),
        &[(2, 0, 0), (0, 2, 4)],
    );
}

#[test]
fn connections_strut_diagonal_3d_left_short() {
    verify(
        ("strut_diagonal_3d_left_short", (0, 0, 1), 18),
        &[(1, 1, 3), (0, 0, 0)],
    );
}

#[test]
fn connections_strut_diagonal_3d_right_short() {
    verify(
        ("strut_diagonal_3d_right_short", (1, 0, 1), 4),
        &[(0, 1, 3), (1, 0, 0)],
    );
}

#[test]
fn connections_tank_track_t5() {
    verify(
        ("t5_tank_track", (1, 2, 0), 15),
        &[
            (0, 0, 0),
            (0, 4, 0),
            (3, 4, 2),
            (6, 4, 0),
            (6, 2, 0),
            (6, 0, 0),
            (3, 0, 2),
            (0, 2, 0),
            (3, 2, 2),
        ],
    );
}

#[test]
fn connections_tank_track_t4() {
    verify(
        ("t4_tank_track", (1, 2, 0), 15),
        &[
            (3, 2, 2),
            (0, 2, 0),
            (6, 2, 0),
            (3, 0, 2),
            (0, 0, 0),
            (6, 0, 0),
        ],
    );
}

#[test]
fn connections_tank_track_t3() {
    verify(
        ("t3_tank_track", (4, 0, 0), 1),
        &[(3, 0, 2), (5, 0, 0), (0, 0, 0), (2, 0, 2)],
    );
}

#[test]
fn connections_tank_track_t2() {
    verify(
        ("t2_tank_track", (1, 0, 0), 15),
        &[(0, 0, 0), (5, 0, 0), (3, 0, 2)],
    );
}

#[test]
fn connections_tank_track_t1() {
    verify(
        ("t1_tank_track", (1, 0, 0), 15),
        &[(0, 0, 0), (2, 0, 2), (4, 0, 0)],
    );
}

#[test]
fn connections_wheel_t5() {
    verify(
        ("t5_wheel", (1, 2, 1), 6),
        &[
            (0, 0, 1),
            (0, 2, 1),
            (0, 0, 0),
            (0, 4, 1),
            (0, 2, 0),
            (0, 4, 0),
        ],
    );
}

#[test]
fn connections_hover_t5() {
    verify(
        ("t5_hover", (1, 2, 0), 6),
        &[(0, 1, 1), (0, 3, 1), (0, 2, 0), (0, 0, 0), (0, 4, 0)],
    );
}

#[test]
fn connections_module_ghost() {
    verify(
        ("module_ghost", (2, 0, 1), 0),
        &[(2, 0, 4), (4, 0, 2), (0, 0, 2), (2, 0, 0)],
    );
}

#[test]
fn connections_module_blink() {
    verify(
        ("module_blink", (1, 0, 1), 0),
        &[
            (0, 0, 0),
            (2, 0, 0),
            (2, 0, 4),
            (0, 0, 4),
            (1, 0, 0),
            (1, 0, 4),
        ],
    );
}

#[test]
fn connections_module_disc() {
    verify(
        ("module_disc", (2, 2, 1), 0),
        &[
            (2, 2, 0),
            (2, 0, 2),
            (2, 4, 2),
            (2, 2, 4),
            (4, 2, 2),
            (0, 2, 2),
        ],
    );
}

#[test]
fn connections_module_power() {
    verify(
        ("module_power", (1, 2, 1), 0),
        &[
            (0, 1, 0),
            (0, 3, 0),
            (1, 2, 0),
            (2, 3, 0),
            (1, 0, 2),
            (2, 1, 0),
        ],
    );
}

#[test]
fn connections_chaingun_t5() {
    verify(
        ("t5_chaingun", (1, 1, 1), 0),
        &[(2, 0, 0), (0, 0, 0), (0, 2, 0), (1, 1, 0), (2, 2, 0)],
    );
}

#[test]
fn connections_aeroflak_t5() {
    verify(
        ("t5_aeroflak", (5, 5, 1), 0),
        &[(5, 5, 0), (5, 10, 0), (5, 0, 0), (0, 5, 0), (10, 5, 0)],
    );
}

#[test]
fn connections_seeker_t5() {
    verify(
        ("t5_seeker", (2, 2, 1), 0),
        &[(2, 4, 0), (0, 2, 0), (2, 2, 0), (4, 2, 0), (2, 0, 0)],
    );
}

#[test]
fn connections_wing_t5() {
    verify(
        ("t5_wing", (1, 2, 0), 3),
        &[(0, 2, 0), (0, 1, 0), (0, 0, 0), (0, 3, 0)],
    );
}

#[test]
fn connections_wing_t4() {
    verify(
        ("t4_wing", (1, 1, 0), 3),
        &[(0, 2, 0), (0, 0, 0), (0, 1, 0)],
    );
}

#[test]
fn connections_rudder_t5() {
    verify(
        ("t5_rudder", (0, 3, 1), 0),
        &[(0, 0, 0), (0, 1, 0), (0, 2, 0), (0, 3, 0)],
    );
}

#[test]
fn connections_rudder_t4() {
    verify(
        ("t4_rudder", (0, 2, 1), 0),
        &[(0, 0, 0), (0, 1, 0), (0, 2, 0)],
    );
}

#[test]
fn connections_wing_t3() {
    verify(("t3_wing", (1, 1, 0), 3), &[(0, 1, 0), (0, 0, 0)]);
}

#[test]
fn connections_electroshield_c_right_t5() {
    verify(
        ("t5_electroshield_c_right", (0, 6, 1), 4),
        &[
            (0, 12, 0),
            (0, 6, 0),
            (2, 0, 0),
            (0, 0, 0),
            (2, 12, 0),
            (2, 6, 0),
        ],
    );
}

#[test]
fn connections_electroshield_b_right_t5() {
    verify(
        ("t5_electroshield_b_right", (3, 2, 0), 6),
        &[(0, 5, 1), (2, 2, 0), (0, 0, 0), (2, 5, 0), (0, 2, 1)],
    );
}

#[test]
fn connections_electroshield_b_left_t5() {
    verify(
        ("t5_electroshield_b_left", (0, 2, 0), 12),
        &[(3, 5, 1), (1, 5, 0), (3, 0, 0), (1, 2, 0), (3, 2, 1)],
    );
}

#[test]
fn connections_mech_leg_t5() {
    verify(
        ("t5_mech_leg", (3, 3, 0), 16),
        &[
            (6, 2, 0),
            (6, 3, 0),
            (4, 0, 0),
            (3, 0, 0),
            (2, 0, 0),
            (1, 0, 0),
            (6, 1, 0),
            (0, 5, 0),
            (1, 6, 0),
            (3, 6, 0),
            (2, 6, 0),
            (4, 6, 0),
            (5, 6, 0),
            (6, 5, 0),
            (6, 4, 0),
            (5, 0, 0),
            (0, 4, 0),
            (0, 3, 0),
            (0, 2, 0),
            (0, 1, 0),
            (1, 5, 1),
            (1, 4, 1),
            (1, 3, 1),
            (1, 2, 1),
            (1, 1, 1),
            (2, 2, 1),
            (2, 4, 1),
            (2, 3, 1),
            (2, 5, 1),
            (3, 3, 1),
            (3, 4, 1),
            (3, 1, 1),
            (3, 2, 1),
            (2, 1, 1),
            (3, 5, 1),
            (4, 3, 1),
            (4, 5, 1),
            (4, 2, 1),
            (4, 4, 1),
            (4, 1, 1),
            (5, 4, 1),
            (5, 5, 1),
            (5, 1, 1),
            (5, 2, 1),
            (5, 3, 1),
        ],
    );
}

#[test]
fn connections_mech_leg_t4() {
    verify(
        ("t4_mech_leg", (2, 2, 0), 16),
        &[
            (0, 1, 0),
            (4, 1, 0),
            (1, 0, 0),
            (4, 2, 0),
            (3, 3, 1),
            (3, 2, 1),
            (2, 1, 1),
            (1, 4, 0),
            (1, 2, 1),
            (1, 3, 1),
            (2, 4, 0),
            (1, 1, 1),
            (3, 1, 1),
            (0, 2, 0),
            (0, 3, 0),
            (2, 3, 1),
            (4, 3, 0),
            (3, 4, 0),
            (3, 0, 0),
            (2, 0, 0),
            (2, 2, 1),
        ],
    );
}

#[test]
fn connections_mech_leg_t3() {
    verify(
        ("t3_mech_leg", (2, 2, 0), 16),
        &[
            (4, 2, 0),
            (3, 3, 0),
            (3, 2, 1),
            (1, 3, 0),
            (2, 1, 1),
            (2, 2, 1),
            (1, 2, 1),
            (1, 1, 0),
            (0, 2, 0),
            (3, 1, 0),
            (2, 3, 1),
            (2, 4, 0),
            (2, 0, 0),
        ],
    );
}

#[test]
fn connections_mech_leg_t2() {
    verify(
        ("t2_mech_leg", (1, 2, 0), 16),
        &[
            (1, 3, 1),
            (1, 2, 1),
            (0, 3, 0),
            (1, 4, 0),
            (2, 2, 0),
            (2, 3, 0),
            (2, 1, 0),
            (1, 0, 0),
            (0, 2, 0),
            (1, 1, 1),
            (0, 1, 0),
        ],
    );
}

#[test]
fn connections_laser_t5() {
    verify(
        ("t5_laser", (2, 5, 1), 0),
        &[(4, 0, 0), (2, 5, 0), (0, 0, 0)],
    );
}

#[test]
fn connections_propeller_t4() {
    verify(
        ("t4_propeller", (1, 1, 1), 0),
        &[(0, 1, 0), (1, 0, 0), (1, 2, 0), (2, 1, 0), (1, 1, 0)],
    );
}

#[test]
fn connections_propeller_t3() {
    verify(
        ("t3_propeller", (0, 1, 1), 0),
        &[(0, 0, 0), (0, 1, 0), (0, 2, 0)],
    );
}

#[test]
fn connections_half_cube() {
    verify(
        ("medium_edge", (0, 1, 1), 14),
        &[(0, 0, 1), (0, 1, 0), (0, 2, 1), (1, 1, 1)],
    );
}

#[test]
fn connections_third_cube() {
    verify(
        ("medium_corner", (1, 1, 1), 0),
        &[(1, 0, 1), (0, 1, 1), (1, 1, 0)],
    );
}

#[test]
fn connections_open_cube() {
    verify(
        ("t3_seeker", (1, 1, 1), 0),
        &[(1, 0, 1), (2, 1, 1), (0, 1, 1), (1, 1, 0), (1, 2, 1)],
    );
}

#[test]
fn connections_full_cube() {
    verify(
        ("medium_cube", (1, 1, 1), 0),
        &[
            (1, 1, 2),
            (1, 2, 1),
            (1, 1, 0),
            (0, 1, 1),
            (2, 1, 1),
            (1, 0, 1),
        ],
    );
}
