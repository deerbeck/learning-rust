enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Blue => println!("Color: Blue"),
            Color::Green => println!("Color: Green"),
            Color::Red => println!("Color: Red"),
        }
    }
}
struct Dimensions {
    height: f64,
    width: f64,
    depth: f64,
}
impl Dimensions {
    fn print(&self) {
        println!("Height: {:?}", self.height);
        println!("Width: {:?}", self.width);
        println!("Depth: {:?}", self.depth);
    }
}
struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

impl ShippingBox {
    fn new_box(dimensions: Dimensions, color: Color, weight: f64) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }
    fn print_characteristics(&self) {
        self.dimensions.print();
        println!("Weight: {:?}", self.weight);
        self.color.print();
    }
}

fn main() {
    let box_dim = Dimensions {
        height: 10.,
        width: 19.,
        depth: 21.,
    };
    let new_box: ShippingBox = ShippingBox::new_box(box_dim, Color::Green, 32.);
    new_box.print_characteristics();
}
