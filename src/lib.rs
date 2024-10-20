use quick_xml::de;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Voc {
    pub filename: String,
    pub size: Size,
    #[serde(rename = "object")]
    pub objects: Vec<Object>,
}

#[derive(Deserialize, Debug)]
pub struct Size {
    width: u32,
    height: u32,
}

#[derive(Deserialize, Debug)]
pub struct Object {
    name: String,
    bndbox: Bndbox,
}

#[derive(Deserialize, Debug)]
pub struct Bndbox {
    xmin: f32,
    ymin: f32,
    xmax: f32,
    ymax: f32,
}

#[derive(Serialize, Debug)]
pub struct Yolo {
    class: usize,
    xcr: f32,
    ycr: f32,
    wr: f32,
    hr: f32,
}

pub struct Converter {
    classes: Vec<String>,
}

impl Converter {
    pub fn new(classes: Vec<String>) -> Converter {
        Converter { classes }
    }

    /// converting Voc  XML format to Yolo TXT format
    pub fn voc_to_yolo(&self, xml: &str, dest: &str) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(xml)?;
        let voc = de::from_str::<Voc>(content.as_str())?;

        let w = voc.size.width;
        let h = voc.size.height;

        let mut lines: Vec<String> = Vec::new();
        for object in voc.objects {
            let bndbox = object.bndbox;
            let xcr = (bndbox.xmin + bndbox.xmax) / 2.0 / w as f32;
            let ycr = (bndbox.ymin + bndbox.ymax) / 2.0 / h as f32;
            let wr = (bndbox.xmax - bndbox.xmin) / w as f32;
            let hr = (bndbox.ymax - bndbox.ymin) / h as f32;

            if !self.classes.contains(&object.name) {
                return Ok(());
            }

            let mut index = 0;
            for item in self.classes.iter().enumerate() {
                if item.1.eq(&object.name) {
                    index = item.0
                }
            }
            let line = format!("{} {} {} {} {}", index, xcr, ycr, wr, hr);

            lines.push(line);
        }


        fs::write(dest, lines.join("\n"))?;
        println!("{:#?}", lines);
        Ok(())
    }
}
