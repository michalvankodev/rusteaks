mod headphone_stand;
mod keyboard;
mod table_attachment;

use crate::headphone_stand::*;
use crate::keyboard::switch::{self, switch, switch_column, KEYCAP_WITH_KEY_HEIGHT};
use crate::table_attachment::*;
use scad::*;

fn main() {
    println!("Printing attachment");

    let table_height = 17.0; // milimeters
    let attachment_width = 25.;

    let (attachment_piece, attachment_props) = table_attachment(table_height, attachment_width);

    create_scad(
        &"table_attachment.scad".to_owned(),
        attachment_piece.clone(),
    );

    let (headphone_stand_piece, stand_props) = headphone_stand(47., 75., 43., 14.);

    let attachable_headphone_stand = attach_table_attachment_to_headphones(
        (headphone_stand_piece, stand_props),
        (attachment_piece.clone(), attachment_props),
    );

    create_scad("headphone_stand.scad", attachable_headphone_stand);

    let finger_pinky_length: f32 = 60.1;
    // KEYBOARD stuff
    let switch = switch_column(7, finger_pinky_length);
    create_scad("switch.scad", switch.0)
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
