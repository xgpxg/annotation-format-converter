Dataset annotation format conversion, currently only supports converting VOC annotation format to YOLO format


Example:
```rust
fn main() -> Result<(), Box<dyn Error>> {
    let converter = Converter::new(vec![String::from("")]);
    converter.voc_to_yolo("/your_path/0001.xml", "/your_path/0001.txt")
}
```