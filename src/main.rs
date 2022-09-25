use scad::*;

fn main() {
    println!("Hello, worldA!");
    create_scad(&"test1.scad".to_owned());
}

fn create_scad(filename: &str) {
    //Create an scad file object for storing the scad objects. This
    //allows us to set things like the detail level ($fn) for the models.
    let mut scad_file = ScadFile::new();

    //Sets the $fn variable in scad which controls the detail level of things
    //like spheres. Look at the scad wiki for details
    scad_file.set_detail(50);

    //Create an scad object
    let mut cube = scad!(Translate(vec3(2.0, 3.0, 3.0)); {
        scad!(Cube(vec3(2.0,1.0,8.0)))
    });

    //Create a cylinder with a height of 10 and a diameter of 3 mm
    let cylinder = scad!(Cylinder(10., Diameter(3.)));

    //Add the cylinder to the cubes translation.
    cube.add_child(cylinder);

    //Add the cube object to the file
    scad_file.add_object(cube.clone());

    //Save the scad code to a file
    scad_file.write_to_file(String::from("output/") + filename);

    //You can also print the code for the object manually since it's just a string
    println!("{}", scad!(Cube(vec3(5., 3., 2.))).get_code());
}
