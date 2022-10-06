use nalgebra::*;
use scad::*;

fn main() {
    println!("Printing attachment");

    let table_height = 10.0; // milimeters

    let mut attachment_piece = table_attachment(table_height);
    let insert = puzzle_insert(table_height + 0.6, table_height + 0.6);

    // TODO Move this to `table_attachment`

    let preview = scad!(Translate(vec3(150., 0., 0.) ); {
        insert
    });
    attachment_piece.add_child(preview);

    create_scad(&"test1.scad".to_owned(), attachment_piece);
}

fn create_scad(filename: &str, scad_obj: ScadObject) {
    //Create an scad file object for storing the scad objects. This
    //allows us to set things like the detail level ($fn) for the models.
    let mut scad_file = ScadFile::new();

    //Sets the $fn variable in scad which controls the detail level of things
    //like spheres. Look at the scad wiki for details
    scad_file.set_detail(50);

    //Add the cube object to the file
    scad_file.add_object(scad_obj.clone());

    //Save the scad code to a file
    scad_file.write_to_file(String::from("output/") + filename);
}

fn table_attachment(table_height: f32) -> ScadObject {
    let x = 30.; // THINK: How to justify `x` size of the table attachment
    let height_padding: f32 = 3.;
    let y_z_size = table_height + height_padding * 2.;

    let table_cut_out = scad!(Translate(vec3(-0.1, -0.1, height_padding + 0.1)); {
        scad!(Cube(vec3(x - 10., y_z_size + 0.2, table_height)))
    });

    let cube = scad!(Cube(vec3(x, y_z_size, y_z_size)));

    let table_part = scad!(Union; { scad!(Difference; {
        cube,
        table_cut_out
    })});

    let plug = scad!(Translate(vec3(x, 0., 0.)); {
        puzzle_plug(y_z_size, y_z_size)
    });

    let result = scad!(Union; {
        table_part,
        plug
    });

    return result;
}

// TODO: Cover up the attachment insert so it looks like a continual piece
fn puzzle_insert(y: f32, attachment_height: f32) -> ScadObject {
    let x = 10.;
    let cover_up_height = 1.;

    let stand = scad!(Cube(vec3(x, y, attachment_height)));

    let triangle_cut_out = scad!(Translate(vec3(0., 0., attachment_height)); {
        scad!(Rotate(180., vec3(0. , 1., 0.)); {
            triangle_union(x, y, attachment_height * 3. / 5.)
        })
    });

    let cover_up = scad!(Translate(vec3(-x, 0., attachment_height - cover_up_height)); {
        scad!(Cube(vec3(2. * x, y, cover_up_height)))
    });

    let union = scad!(Union; {
        stand,
        triangle_cut_out,
        cover_up,
    });

    let insert = scad!(Translate(vec3(x, y, 0.)); {
        scad!(Rotate(180., vec3(0., 0., 1.)); {
            union
        })
    });
    return insert;
}

fn puzzle_plug(y: f32, attachment_height: f32) -> ScadObject {
    let x = 10.;
    let cover_up_height = 1.;
    let stand = scad!(Cube(vec3(x, y, attachment_height)));
    let triangle_cut_out = scad!(Translate(vec3(x, 0., attachment_height)); {
        scad!(Rotate(180., vec3(0. , 1., 0.)); {
            triangle_union(x, y, attachment_height * 3. / 5.)
        })
    });
    let cover_down = scad!(Translate(vec3(-x, 0., attachment_height - cover_up_height)); {
        scad!(Cube(vec3(2. * x, y, cover_up_height)))
    });

    let union = scad!(Difference; {
        stand,
        prepare_for_diff(triangle_cut_out),
        prepare_for_diff(cover_down)
    });
    return union;
}

fn triangle_union(x: f32, y: f32, z: f32) -> ScadObject {
    let corner_radius = 0.2;
    let bigger_triangle = triangle(
        vec2(0., y - 0.3 - corner_radius),
        vec2(0., 0.3 + corner_radius),
        vec2(x / 2., y / 2.),
        corner_radius,
        z,
    );
    let smaller_triangle = triangle(
        vec2(x * 3. / 4., y * 1. / 6. + corner_radius),
        vec2(x * 3. / 4., y * 5. / 6. - corner_radius),
        vec2(x / 3., y / 2.),
        corner_radius,
        z,
    );

    let union = scad!(Union; {
        bigger_triangle,
        smaller_triangle,
    });
    return union;
}

fn triangle(
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

fn prepare_for_diff(obj: ScadObject) -> ScadObject {
    return scad!(Translate(vec3(-0.01, -0.01, -0.01)); {
        scad!(Scale(vec3(1.01, 1.01, 1.01)); {
            obj
        })
    });
}
