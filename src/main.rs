use nalgebra::*;
use scad::*;

fn main() {
    println!("Printing attachment");

    let table_height = 10.0;

    let mut attachment_piece = table_attachment(table_height);
    let puzzle_ins = puzzle_insert(table_height + 3., table_height);

    let preview = scad!(Translate(vec3(-50., 0., 0.) ); {
        puzzle_ins
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
    let height_padding: f32 = 3.;
    let y_z_size = table_height + height_padding * 2.;

    let table_cut_out = scad!(Translate(vec3(-0.1, -0.1, height_padding + 0.1)); {
        scad!(Cube(vec3(20., y_z_size + 0.2, table_height)))
    });

    let cube = scad!(Cube(vec3(30., y_z_size, y_z_size)));

    let result = scad!(Union; { scad!(Difference; {
        cube,
        table_cut_out
    })});

    return result;
}

fn puzzle_insert(y: f32, table_height: f32) -> ScadObject {
    let x = 10.;
    let z = table_height * 2. / 3.;
    let whole = scad!(Cube(vec3(x, y, z)));
    let bigger_triangle = triangle(
        vec2(x / 3., y / 2.),
        vec2(x - 0.3, 0.3),
        vec2(x - 0.3, y - 0.3),
        1.,
        z,
    );
    let smaller_triangle = triangle(
        vec2(x / 4., y / 2.),
        vec2(x / 3., y / 3.),
        vec2(x - 0.3, y * 2. / 3.),
        1.,
        z,
    );

    // TODO Make difference to cut out the triangles
    let diff = scad!(Union; { // Difference
        whole,
        prepare_for_diff(bigger_triangle),
        prepare_for_diff(smaller_triangle),
    });
    return diff;
}

fn triangle(
    xy1: Vector2<f32>,
    xy2: Vector2<f32>,
    xy3: Vector2<f32>,
    corner_radius: f32,
    height: f32,
) -> ScadObject {
    let cylinder = scad!(Cylinder(height, Radius(corner_radius)));
    let tg = scad!(Translate(vec3(corner_radius, corner_radius, 0.0)); {
        scad!(Hull; {
            scad!(Translate(vec3(xy1.x, xy1.y, height)); {
                cylinder.clone(),
            }),
            scad!(Translate(vec3(xy2.x, xy2.y, height)); {
                cylinder.clone(),
            }),
            scad!(Translate(vec3(xy3.x, xy3.y, height)); {
                cylinder.clone(),
            })
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
