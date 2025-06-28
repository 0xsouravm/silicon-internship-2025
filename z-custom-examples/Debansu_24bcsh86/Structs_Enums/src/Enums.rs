fn main(){
    println!("🎯 Enum Examples");

    direction_example();
    ice_cream_example();
    shape_example();
}

// Enum With Variants
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn is_north(&self) -> bool {
        matches!(self, Direction::North)
    }

    fn is_south(&self) -> bool {
        matches!(self, Direction::South)
    }

    fn is_east(&self) -> bool {
        matches!(self, Direction::East)
    }

    fn is_west(&self) -> bool {
        matches!(self, Direction::West)
    }
}

fn direction_example(){
    println!("\nDirection Enum");

    let direction_one = Direction::South;
    let direction_two = Direction::West;

    match direction_one {
        Direction::North => println!("Going North"),
        Direction::South => println!("Going South"),
        Direction::East => println!("Going East"),
        Direction::West => println!("Going West"),
    }

    let north = Direction::North;
    let east = Direction::East;

    println!("Is North?: {}", north.is_north());
    println!("Is South?: {}", direction_one.is_south());
    println!("Is East?: {}", east.is_east());
    println!("Is West?: {}", direction_two.is_west());
}

// Using Utility Method
#[derive(Debug)]
enum IceCreamFlavours {
    BelgiumChocolate,
    ButterScotch,
    Vanilla,
    BlackCurrant,
}

impl IceCreamFlavours {
    fn is_belgium_chocolate(&self) -> bool {
        matches!(self, IceCreamFlavours::BelgiumChocolate)
    }

    fn is_butter_scotch(&self) -> bool {
        matches!(self, IceCreamFlavours::ButterScotch)
    }

    fn is_vanilla(&self) -> bool {
        matches!(self, IceCreamFlavours::Vanilla)
    }

    fn is_black_currant(&self) -> bool {
        matches!(self, IceCreamFlavours::BlackCurrant)
    }

    fn supreme_flavour() -> Self {
        IceCreamFlavours::BelgiumChocolate
    }
}

fn ice_cream_example(){
    println!("\nIce Cream Enum");

    let flavour_one = IceCreamFlavours::ButterScotch;
    let flavour_two = IceCreamFlavours::Vanilla;
    let flavour_three = IceCreamFlavours::BlackCurrant;

    println!("Is Butter Scotch?: {}", flavour_one.is_butter_scotch());
    println!("Is Vanilla?: {}", flavour_two.is_vanilla());
    println!("Is Black Currant?: {}", flavour_three.is_black_currant());

    let supreme_flavour = IceCreamFlavours::supreme_flavour();
    println!("Is Supreme Flavour: {}, I.E: {:?}", supreme_flavour.is_belgium_chocolate(), supreme_flavour);
}

// With Associated Data
#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64)
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => 3.14 * radius * radius,
            Self::Square(side) => side * side,
            Self::Rectangle(width, height) => width * height,
        }
    }
}

fn shape_example(){
    println!("\nShape Enum");

    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    println!("Area Of Circle: {}", circle.area());
    println!("Area Of Square: {}", square.area());
    println!("Area Of Rectangle: {}", rectangle.area());
}