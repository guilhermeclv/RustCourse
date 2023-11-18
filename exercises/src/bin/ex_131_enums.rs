//We can also implement methods for enums.
#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    fn color(&self)->String{
       let color =  match self {
            TrafficLightColor::Green=> "green",
            TrafficLightColor::Yellow=> "yellow",
            TrafficLightColor::Red=> "red"
        };
        return color.to_string();
    }
    
}

fn main() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}
