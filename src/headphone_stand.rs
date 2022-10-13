use crate::table_attachment::prepare_for_diff;
use nalgebra::*;
use scad::*;

const STAND_THICKNESS: f32 = 1.5;
const BORDER_THICKNESS: f32 = 1.5;

pub fn headphone_stand(width: f32, length: f32, radius: f32, border_height: f32) -> ScadObject {
    let base = scad!(Cylinder(width, Radius(radius)));

    let cut_base = scad!(Cylinder(
        width + BORDER_THICKNESS,
        Radius(radius - STAND_THICKNESS)
    ));

    // TODO calculate the length from the center where we should cut out the rest of the
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
        prepare_for_diff(cut_base),
        prepare_for_diff(cut_rest)
    });
    return result;
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
