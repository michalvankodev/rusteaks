use scad::*;

const ATTACHMENT_LENGTH: f32 = 40.;
const TABLE_CUT_OUT_LENGTH: f32 = 30.;
pub const HEIGHT_PADDING: f32 = 5.;

pub struct TableAttachmentProps {
    pub width: f32,
    pub height: f32,
    pub length: f32,
}

pub fn table_attachment(table_height: f32, width: f32) -> (ScadObject, TableAttachmentProps) {
    let height = table_height + HEIGHT_PADDING * 2.;

    let table_cut_out = scad!(Translate(vec3(-0.1, -0.1, HEIGHT_PADDING + 0.1)); {
        scad!(Cube(vec3(TABLE_CUT_OUT_LENGTH, width, table_height)))
    });

    let cube = scad!(Cube(vec3(ATTACHMENT_LENGTH, width, height)));

    let result = scad!(Union; { scad!(Difference; {
        cube,
        prepare_for_diff(table_cut_out)
    })});

    return (
        result,
        TableAttachmentProps {
            width,
            height,
            length: ATTACHMENT_LENGTH,
        },
    );
}

pub fn prepare_for_diff(obj: ScadObject) -> ScadObject {
    return scad!(Translate(vec3(-0.01, -0.01, -0.01)); {
        scad!(Scale(vec3(1.01, 1.01, 1.01)); {
            obj
        })
    });
}
