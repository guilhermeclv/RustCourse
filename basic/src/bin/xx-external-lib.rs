use chrono::Local;
fn main(){
    let t = Local::now();
    println!("Agora é {}", t.format("%d/%m/%Y %H:%M"));
}