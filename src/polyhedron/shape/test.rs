use super::*;
use crate::render::message::PresetMessage::{self, *};
use std::fs::create_dir_all;

use test_case::test_case;

impl Shape {
    pub fn tetrahedron() -> Shape {
        Shape::from(Distance::tetrahedron())
    }

    pub fn png(&self) {
        use image::{ImageError, ImageReader, RgbaImage};
        use std::io::Cursor;
        use viuer::{print, Config};
        if let Some(bytes) = self.distance.png() {
            let mut reader = ImageReader::new(Cursor::new(bytes));
            reader.set_format(image::ImageFormat::Png);
            let img = reader.decode().unwrap();
            // let cfg = Config {
            //     width: Some(300),
            //     height: Some(300),
            //     use_kitty: true,
            //     ..Default::default()
            // };
            print(&img, &Config::default()).unwrap();
        }
    }
}

#[test]
fn truncate_contract() {
    let prefix = "tests/truncate_contract/";
    create_dir_all(prefix).unwrap();
    let mut shape = Shape::from(Distance::tetrahedron());
    //shape.distance.render(prefix, "tetrahedron.svg");
    let edges = shape.truncate(None);
    println!("edges: {edges:?}");
    //shape.distance.render(prefix, "truncated_tetrahedron.svg");
    shape.distance.contract_edges(edges);
    // shape
    //     .distance
    //     .render(prefix, "contracted_truncated_tetrahedron.svg");
    assert_eq!(shape.distance, Distance::tetrahedron());
}

#[test]
fn split_vertex_contract() {
    let mut control = Distance::new(6);
    // Original outline
    control[[1, 2]] = 1;
    control[[2, 3]] = 1;
    control[[3, 1]] = 1;
    // Connections
    control[[0, 1]] = 1;
    control[[4, 2]] = 1;
    control[[5, 3]] = 1;
    // New face
    control[[0, 4]] = 1;
    control[[4, 5]] = 1;
    control[[5, 0]] = 1;
    let mut test = Shape::from(Distance::tetrahedron());
    let edges = test.split_vertex(0);
    test.distance.contract_edges(edges);
    assert_eq!(test.distance, Distance::tetrahedron());
}

// #[test]
// fn ambo() {
//     // let prefix = "tests/ambo/";
//     // create_dir_all(prefix).unwrap();
//     let tetrahedron = Shape::from(Distance::tetrahedron());
//
//     assert_eq!(
//         tetrahedron.ambod(),
//         Distance::preset(&PresetMessage::Octahedron)
//     );
// }
