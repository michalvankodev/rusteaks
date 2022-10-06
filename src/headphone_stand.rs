use crate::table_attachment::prepare_for_diff;
use nalgebra::*;
use scad::*;

const STAND_THICKNESS: f32 = 1.5;

pub fn headphone_stand(width: f32, length: f32, radius: f32, border_height: f32) -> ScadObject {
    let base = scad!(Cylinder(width, Radius(radius)));

    let cut_base = scad!(Cylinder(width, Radius(radius - STAND_THICKNESS)));

    // TODO calculate the length from the center where we should cut out the rest of the
    // circle
    // @see figma

    // TODO borders
    let result = scad!(Difference; {
        base,
        prepare_for_diff(cut_base)
    });
    return result;
}
