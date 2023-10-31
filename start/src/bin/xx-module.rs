#[path = "./xx-module2.rs"]
mod qualquer_nome; // this is a exteral module
mod xx_module3;
mod xx_module4;

mod computer {
    fn number_4_to_6(number:Option<i32>)->bool{ // is not possible use private function in other module
        match number {
            Some(4..=6)=> return true,
            None=>return false,
            _=>return false,
        }
    }
    pub fn number_1_to_6(number:Option<i32>)->bool{ // you can use public function in other module
        match number {
            Some(1..=3)=> return true,
            None=> return false,
            _=>return number_4_to_6(number),
        }
    }

}
use xx_module3::external_mod_3::externo::minha_funcao as xx_module3_minha_funcao;
fn main(){
    qualquer_nome::minha_funcao();
    xx_module3::external_mod_3::externo::minha_funcao();
    xx_module3::xx_module3_minha_funcao();
    xx_module4::minha_funcao2();
    xx_module3_minha_funcao();
    let result = computer::number_1_to_6(Some(1));
    println!("result is: {:?}",result);
}