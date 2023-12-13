enum Weekday { //unit-like enums
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
//with this implementation we can use the method to_string() in the enum Weekday
impl Weekday {
    fn to_string(&self) -> String {
        match self {
            Weekday::Monday => String::from("Segunda-feira"),
            Weekday::Tuesday => String::from("Terça-feira"),
            Weekday::Wednesday => String::from("Quarta-feira"),
            Weekday::Thursday => String::from("Quinta-feira"),
            Weekday::Friday => String::from("Sexta-feira"),
            Weekday::Saturday => String::from("Sábado"),
            Weekday::Sunday => String::from("Domingo"),
        }
    }
}
impl std::fmt::Display for Weekday {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       //using the method to_string() in the enum Weekday
        write!(f, "{}", self.to_string())
    }
}

struct CssColor {
    hex_code: String,
    rgba_value: (u8, u8, u8, f32)
}
enum Color { //tuple-like enums
    RGB(u8, u8, u8),     // Representa cores RGB com canais vermelho, verde e azul.
    CMYK(u8, u8, u8, u8), // Representa cores CMYK com canais ciano, magenta, amarelo e preto.
    CSS(CssColor),         // é possivel usar um struct no enum.
}
// another way to implement a method to convet to string in a enum
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Color::RGB(r, g, b) => write!(f, "RGB: R={}, G={}, B={}", r, g, b),
            Color::CMYK(c, m, y, k) => write!(f, "CMYK: C={}, M={}, Y={}, K={}", c, m, y, k),
            Color::CSS(css_color) => write!(f, "CSS: Hex={}, RGBA={:?}", css_color.hex_code, css_color.rgba_value),
        }
    }
}

fn main() {
    let today = Weekday::Monday;
    println!("today is {}", today);

    let rgb_color = Color::RGB(255, 0, 0); // Vermelho em RGB
    let cmyk_color = Color::CMYK(0, 100, 100, 0); // Vermelho em CMYK
    let css_color = CssColor {
        hex_code: "#FF0000".to_string(),
        rgba_value: (255, 0, 0, 1.0),
    };
    let custom_color = Color::CSS(css_color);

    match rgb_color {
        Color::RGB(r, g, b) => println!("RGB: R={}, G={}, B={}", r, g, b),
        Color::CMYK(c, m, y, k) => println!("CMYK: C={}, M={}, Y={}, K={}", c, m, y, k),
        Color::CSS(css_color) => {
            println!("CSS: Hex={}, RGBA={:?}", css_color.hex_code, css_color.rgba_value)
        }
    }

    match cmyk_color {
        Color::RGB(r, g, b) => println!("RGB: R={}, G={}, B={}", r, g, b),
        Color::CMYK(c, m, y, k) => println!("CMYK: C={}, M={}, Y={}, K={}", c, m, y, k),
        Color::CSS(css_color) => {
            println!("CSS: Hex={}, RGBA={:?}", css_color.hex_code, css_color.rgba_value)
        }
    }

    match custom_color {
        Color::RGB(r, g, b) => println!("RGB: R={}, G={}, B={}", r, g, b),
        Color::CMYK(c, m, y, k) => println!("CMYK: C={}, M={}, Y={}, K={}", c, m, y, k),
        Color::CSS(css_color) => {
            println!("CSS: Hex={}, RGBA={:?}", css_color.hex_code, css_color.rgba_value)
        }
    }
}