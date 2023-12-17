#[derive(Debug)]    
struct Rectangle<T, U> {
    width: T,
    height: U
}

impl<T, U> Rectangle<T, U> {

    fn get_width(&self) -> &T {
        &self.width
    }
    fn get_height(&self) -> &U {
        &self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 1u8,
        height: 3u16
    };
    println!("rect is {:?}", rect);
    println!("width  is {}", rect.get_width());
    println!("height is {}", rect.get_height());
}