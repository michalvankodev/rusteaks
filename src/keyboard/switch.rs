use scad::*;

pub const SWITCH_HOLE_WIDTH: f32 = 14.;
pub const SWITCH_BORDER: f32 = 0.8 * 2.; // We have multiplied it for the sake of test
pub const CUT_OFF_LENGTH: f32 = 1.;
pub const CUT_OFF_WIDTH: f32 = 4.;
pub const CUT_OFF_MARGIN: f32 = 1.41;
pub const KEY_MARGIN: f32 = 1.;
pub const PLATE_HEIGHT: f32 = 4.;

pub const KEYCAP_WITH_KEY_WIDTH: f32 = 18.0;
pub const KEYCAP_WITH_KEY_HEIGHT: f32 = 14.0;

pub struct SwitchProps {
    pub width: f32,
}
pub struct SwitchRowProps {}

pub fn switch() -> (ScadObject, SwitchProps) {
    let switch_width = SWITCH_HOLE_WIDTH + 2. * SWITCH_BORDER;
    let switch = scad!(Difference; {
        scad!(Cube(vec3(switch_width, switch_width, PLATE_HEIGHT))),
        scad!(Translate(vec3(SWITCH_BORDER, SWITCH_BORDER, 0.)); {
            scad!(Cube(vec3(SWITCH_HOLE_WIDTH, SWITCH_HOLE_WIDTH, PLATE_HEIGHT)))
        }),
        // Both cut off holes are created as one long cube
        scad!(Translate(vec3((switch_width - (SWITCH_HOLE_WIDTH + 2. * CUT_OFF_LENGTH)) / 2., switch_width / 2. - CUT_OFF_WIDTH / 2., - CUT_OFF_MARGIN)); {
            scad!(Cube(vec3(SWITCH_HOLE_WIDTH + 2. * CUT_OFF_LENGTH, CUT_OFF_WIDTH, PLATE_HEIGHT)))
        })
    });
    let result = switch;
    return (
        result,
        SwitchProps {
            width: switch_width,
        },
    );
}

struct RowInfo {
    index: u8,
    switch_props: SwitchProps,
    rotation: f32,
}

pub fn switch_column(count: u8, finger_concave_radius: f32) -> (ScadObject, SwitchRowProps) {
    let seq = 0..count;
    let mut result = scad!(Union);

    seq.map(|row| {
        let (switch, props) = switch();
        let rotation = 180. - 2. * get_rotation(finger_concave_radius, finger_concave_radius, KEYCAP_WITH_KEY_WIDTH + KEY_MARGIN).to_degrees();

        println!("rotation:{}", rotation);


        // Put switch into middle so it is rotated by center
        let repositioned_switch = scad!(Translate(vec3(- props.width / 2., 0., -finger_concave_radius - KEYCAP_WITH_KEY_HEIGHT)); { switch });

        let rotation_multi = (f32::from(row) + 0.5) - f32::from(count) / 2. ;
        let rotated_switch = scad!(Rotate(rotation * rotation_multi, vec3(0., 1., 0.)); { repositioned_switch});

        return (
            rotated_switch,
            RowInfo {
                index: row,
                switch_props: props,
                rotation ,
            },
        );
    })
    .for_each(|(switch, row_info)| {
        let base = scad!(Polygon(PolygonParameters::new(vec![
            vec2(0., 0.),
            vec2(0., PLATE_HEIGHT),
            // TODO: Calculate these 2 vectors
            vec2(5., PLATE_HEIGHT + 0.4),
            vec2(6., 0.4)
        ])));

        let extruded = scad!(LinearExtrude(LinExtrudeParams {
            height: row_info.switch_props.width,
            twist: 0.,
            center: false,
            slices: 0,
            convexity: 1
        }); { base });

        let rotated = scad!(Rotate(90., vec3(1., 0., 0.)); { extruded });

        // let key_margin = scad!(Translate(vec3(0., 0.,- finger_concave_radius - KEYCAP_WITH_KEY_HEIGHT)); {
        let moved = scad!(Translate(vec3(0., row_info.switch_props.width, 0.)); {
            rotated
        });
        // TODO: Make the switches connected
        result.add_child(switch);
        if row_info.index < count - 1 {
            result.add_child(moved);
        }
    });
    return (result, SwitchRowProps {});
}

pub fn third_side(side1: f32, side2: f32) -> f32 {
    let side3pow2 = side1.powi(2) + side2.powi(2);
    return side3pow2.sqrt();
}

pub fn get_rotation(a: f32, b: f32, c: f32) -> f32 {
    let sum = b.powi(2) + c.powi(2) - a.powi(2);
    let div = sum / (2. * b * c);
    return div.acos();
}
