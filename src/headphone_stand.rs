use crate::table_attachment::{prepare_for_diff, HEIGHT_PADDING, PUZZLE_LENGTH};
use scad::*;

const BORDER_THICKNESS: f32 = 2.8;

pub fn attach_insert_headphones(
    headphone_stand: (ScadObject, HeadPhoneStandProps),
    insert: ScadObject,
    headphone_radius: f32,
    headphone_width: f32,
    table_height: f32,
) -> ScadObject {
    let insert_y_z = table_height + 2. * HEIGHT_PADDING;
    let rotated_insert = scad!(Translate(vec3(headphone_radius, insert_y_z / 2., headphone_width)); {
        scad!(Rotate(180., vec3(1., 0., 1.)); {
            insert
        })
    });
    // TODO Refactor and use the calculations
    // radius - distance
    let continuiation = scad!(Translate(vec3(headphone_radius - 21.7, - insert_y_z / 2., headphone_width)); {
        scad!(Cube(vec3(21.7, insert_y_z, PUZZLE_LENGTH)))
    });
    let result = scad!(Union; {
        headphone_stand.0,
        rotated_insert,
        continuiation
    });
    return result;
}

pub struct HeadPhoneStandProps {
    pub distance: f32,
}

pub fn headphone_stand(
    width: f32,
    length: f32,
    radius: f32,
    border_height: f32,
) -> (ScadObject, HeadPhoneStandProps) {
    let base = scad!(Cylinder(width, Radius(radius)));

    let distance = distance_from_middle(length, radius);

    let cut_rest = scad!(Translate(vec3(- 2. * radius + distance, - radius, 0.)); {
        scad!(Cube(vec3(2. * radius, radius * 2., width + BORDER_THICKNESS)))
    });

    let x_scale = 2. * border_height / length * 2.;
    let border = scad!(Translate(vec3(distance, 0., 0.)); {
        scad!(Scale(vec3(x_scale, 1., 1.)); {
             scad!(Cylinder(BORDER_THICKNESS, Radius(length / 2.)))
        })
    });

    let top_border = scad!(Translate(vec3(0., 0., width)); {
        border.clone()
    });

    let base_with_borders = scad!(Union; {
        base,
        border,
        top_border,
    });
    let result = scad!(Difference; {
        base_with_borders,
        prepare_for_diff(cut_rest)
    });
    return (result, HeadPhoneStandProps { distance });
}

pub fn distance_from_middle(length: f32, radius: f32) -> f32 {
    return (radius.powf(2.) - (length / 2.).powf(2.)).sqrt();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_from_middle() {
        assert_eq!(distance_from_middle(24., 20.), 16.);
    }
}
