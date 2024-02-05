use crate::table_attachment::TableAttachmentProps;
use scad::*;

const BORDER_THICKNESS: f32 = 2.8;

pub fn attach_table_attachment_to_headphones(
    (headphone_stand_piece, stand_props): (ScadObject, HeadPhoneStandProps),
    (attachment, attachment_props): (ScadObject, TableAttachmentProps),
) -> ScadObject {
    let rotated_attachment = scad!(Translate(vec3(stand_props.distance, attachment_props.width / - 2., stand_props.width + attachment_props.length - BORDER_THICKNESS)); {
        scad!(Rotate(90., vec3(0., 1., 0.)); {
            attachment
        })
    });
    let result = scad!(Union; {
        headphone_stand_piece,
        rotated_attachment
    });
    return result;
}

pub struct HeadPhoneStandProps {
    pub distance: f32,
    pub radius: f32,
    pub width: f32,
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

    let top_border = scad!(Translate(vec3(0., 0., width - BORDER_THICKNESS)); {
        border.clone()
    });

    let base_with_borders = scad!(Union; {
        base,
        border,
        top_border,
    });
    let result = scad!(Difference; {
        base_with_borders,
        cut_rest
    });
    return (
        result,
        HeadPhoneStandProps {
            distance,
            radius,
            width,
        },
    );
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
