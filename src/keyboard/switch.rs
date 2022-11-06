use scad::*;

pub const SWITCH_HOLE_WIDTH: f32 = 14.;
pub const SWITCH_BORDER: f32 = 0.8 * 2.; // We have multiplied it for the sake of test
pub const CUT_OFF_LENGTH: f32 = 1.;
pub const CUT_OFF_WIDTH: f32 = 4.;
pub const CUT_OFF_MARGIN: f32 = 1.41;
pub const KEY_MARGIN: f32 = 3.;
pub const PLATE_HEIGHT: f32 = 4.;

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
    top_x: f32,
    switch_props: SwitchProps,
}
pub fn switch_row(count: u8) -> (ScadObject, SwitchRowProps) {
    let seq = 0..count;
    let mut result = scad!(Union);

    seq.map(|row| {
        let (switch, props) = switch();
        // TODO: Add the margin between the keys

        let x = f32::from(row) * (props.width + KEY_MARGIN);
        let repositioned_switch = scad!(Translate(vec3(x, 0., 0.)); { switch });

        let top_x = x + props.width;
        return (
            repositioned_switch,
            RowInfo {
                index: row,
                top_x,
                switch_props: props,
            },
        );
    })
    .for_each(|(switch, row_info)| {
        let key_margin = scad!(Translate(vec3(row_info.top_x, 0., 0.)); {
            scad!(Cube(vec3(KEY_MARGIN, row_info.switch_props.width, PLATE_HEIGHT)))
        });
        result.add_child(switch);
        if (row_info.index < count - 1) {
            result.add_child(key_margin);
        }
    });
    return (result, SwitchRowProps {});
}
