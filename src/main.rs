mod table_attachment;
use scad::*;

use crate::table_attachment::*;

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
