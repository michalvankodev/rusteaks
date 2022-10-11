mod headphone_stand;
mod table_attachment;

use crate::headphone_stand::*;
use crate::table_attachment::*;
use scad::*;

fn main() {
    println!("Printing attachment");

    let table_height = 10.0; // milimeters

    let attachment_piece = table_attachment(table_height);

    // Get rid of the magic `0.6`
    // It is a `HEIGHT_PADDING * 2 + table_height`
    //let insert = puzzle_insert(table_height + 0.6, table_height + 0.6);

    create_scad(&"table_attachment.scad".to_owned(), attachment_piece);

    let headphone_stand_piece = headphone_stand(35., 50., 35., 9.);

    create_scad("headphone_stand.scad", headphone_stand_piece);
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
