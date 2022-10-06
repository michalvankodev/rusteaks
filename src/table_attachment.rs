use nalgebra::*;
use scad::*;

const ATTACHMENT_LENGTH: f32 = 30.;
const HEIGHT_PADDING: f32 = 3.;
const PUZZLE_LENGTH: f32 = 10.;
const COVER_UP_HEIGHT: f32 = 1.;

pub fn table_attachment(table_height: f32) -> ScadObject {
    let y_z_size = table_height + HEIGHT_PADDING * 2.;

    let table_cut_out = scad!(Translate(vec3(-0.1, -0.1, HEIGHT_PADDING + 0.1)); {
        scad!(Cube(vec3(ATTACHMENT_LENGTH - 10., y_z_size + 0.2, table_height)))
    });

    let cube = scad!(Cube(vec3(ATTACHMENT_LENGTH, y_z_size, y_z_size)));

    let table_part = scad!(Union; { scad!(Difference; {
        cube,
        table_cut_out
    })});

    let plug = scad!(Translate(vec3(ATTACHMENT_LENGTH, 0., 0.)); {
        puzzle_plug(y_z_size, y_z_size)
    });

    let result = scad!(Union; {
        table_part,
        plug
    });

    return result;
}

pub fn puzzle_insert(y: f32, attachment_height: f32) -> ScadObject {
    let stand = scad!(Cube(vec3(PUZZLE_LENGTH, y, attachment_height)));

    let triangle_cut_out = scad!(Translate(vec3(0., 0., attachment_height)); {
        scad!(Rotate(180., vec3(0. , 1., 0.)); {
            triangle_union(PUZZLE_LENGTH, y, attachment_height * 3. / 5.)
        })
    });

    let cover_up = scad!(Translate(vec3(-PUZZLE_LENGTH, 0., attachment_height - COVER_UP_HEIGHT)); {
        scad!(Cube(vec3(2. * PUZZLE_LENGTH, y, COVER_UP_HEIGHT)))
    });

    let union = scad!(Union; {
        stand,
        triangle_cut_out,
        cover_up,
    });

    let insert = scad!(Translate(vec3(PUZZLE_LENGTH, y, 0.)); {
        scad!(Rotate(180., vec3(0., 0., 1.)); {
            union
        })
    });
    return insert;
}

pub fn puzzle_plug(y: f32, attachment_height: f32) -> ScadObject {
    let stand = scad!(Cube(vec3(PUZZLE_LENGTH, y, attachment_height)));
    let triangle_cut_out = scad!(Translate(vec3(PUZZLE_LENGTH, 0., attachment_height)); {
        scad!(Rotate(180., vec3(0. , 1., 0.)); {
            triangle_union(PUZZLE_LENGTH, y, attachment_height * 3. / 5.)
        })
    });
    let cover_down = scad!(Translate(vec3(-PUZZLE_LENGTH, 0., attachment_height - COVER_UP_HEIGHT)); {
        scad!(Cube(vec3(2. * PUZZLE_LENGTH, y, COVER_UP_HEIGHT)))
    });

    let union = scad!(Difference; {
        stand,
        prepare_for_diff(triangle_cut_out),
        prepare_for_diff(cover_down)
    });
    return union;
}

pub fn triangle_union(x: f32, y: f32, z: f32) -> ScadObject {
    let corner_radius = 0.2;
    let bigger_triangle = triangle(
        vec2(0., y - 0.3 - corner_radius),
        vec2(0., 0.3 + corner_radius),
        vec2(PUZZLE_LENGTH / 2., y / 2.),
        corner_radius,
        z,
    );
    let smaller_triangle = triangle(
        vec2(PUZZLE_LENGTH * 3. / 4., y * 1. / 6. + corner_radius),
        vec2(PUZZLE_LENGTH * 3. / 4., y * 5. / 6. - corner_radius),
        vec2(PUZZLE_LENGTH / 3., y / 2.),
        corner_radius,
        z,
    );

    let union = scad!(Union; {
        bigger_triangle,
        smaller_triangle,
    });
    return union;
}

pub fn triangle(
    xy1: Vector2<f32>,
    xy2: Vector2<f32>,
    xy3: Vector2<f32>,
    corner_radius: f32,
    height: f32,
) -> ScadObject {
    let cylinder = scad!(Cylinder(height, Radius(corner_radius)));
    let tg = scad!(Hull; {
        scad!(Translate(vec3(xy1.x, xy1.y, 0.)); {
            cylinder.clone(),
        }),
        scad!(Translate(vec3(xy2.x, xy2.y, 0.)); {
            cylinder.clone(),
        }),
        scad!(Translate(vec3(xy3.x, xy3.y, 0.)); {
            cylinder.clone(),
        })
    });
    return tg;
}

pub fn prepare_for_diff(obj: ScadObject) -> ScadObject {
    return scad!(Translate(vec3(-0.01, -0.01, -0.01)); {
        scad!(Scale(vec3(1.01, 1.01, 1.01)); {
            obj
        })
    });
}
