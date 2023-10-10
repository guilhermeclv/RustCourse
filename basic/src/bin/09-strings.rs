use unicode_segmentation::UnicodeSegmentation;

fn using_graphemes(){
    
    let minha_string = String::from("Olá, coração! 🌍🚀 é um अभिवादन");

    let graphemes: Vec<&str> = minha_string.graphemes(true).collect(); // it is when you use text with special unicodes bigger than 4 bytes
    println!("Graphemes: {:?}", graphemes); //use &str as type of vector because it is a slice of string

    let chars: Vec<char> = minha_string.chars().collect(); // every try use this way,rarely you will need to use graphemes (because it is more efficient)
    println!("Chars:     {:?}", chars);

    for s in graphemes{
        println!("{} - {:?}",s, s.bytes().collect::<Vec<u8>>());
    }
}

fn main(){
    //rust use utf8 aS default encoding and unicode, because it is necessary use 4 bytes (32 bits) for a simple char
    //Strings slice
    let _str_text = "my name"; //it is a slice of a string (immutable), this value ("my name") is allocated in static memory and str_text is a pointer to this value this is allocated in stack memory
    
    //String OR HEAP STRING
    let mut _string_00 = String::new(); 
    _string_00 = "pedro ".to_string();
    _string_00 += "samarino";
    _string_00.push_str(" braga");
    println!("string_00 = {}",_string_00);
    let mut string_text = String::from("my name"); //it is a string (mutable), this value ("my name") is allocated in heap memory
    string_text.push_str(" is joão pedro "); // you can add a string in a string
    string_text.push_str("samarino ");
    string_text = string_text.trim().to_string(); // trim remove the white spaces in the start and end of a string
    string_text.push('!'); // you can add a char in a string

    let string_text_2 = _str_text.to_string(); // you can convert a slice to a string
    let _string_text_3: String = "olther string 3".into(); // into is a generic method to convert vars, it verify the type of variable
    let _string_text_4 = "olther string 4".to_owned(); // This method use de context of the variable to convert it is equivalent to String::from("olther string 4")
    println!("string_text = ({string_text}) it have {} chars and {} bytes => because (ã) use 2 bytes",string_text.chars().count(),string_text.len());
    println!("string_text_2 = {}",string_text_2);
    let upcase_string = string_text_2.to_uppercase();
    println!("string_text_2 UPCASE = {upcase_string}");
    let lowercase_string = string_text_2.to_lowercase();
    println!("string_text_2 lowercase = {lowercase_string}");
    let replace_string = string_text_2.replace("name","nome");
    println!("string_text_2 replace = {replace_string}");
    println!("{}","-".repeat(40));//repeat a string
    let _string_interpolation =  "string interpolation
                                        is possible
                                        in rust";
    let _string_interpolation_2 = "string interpolation\
                                        is possible\
                                        in rust 2";
    //using regex
    use regex::Regex;
    let re = Regex::new(r"(\w{2})").unwrap();
    let replace_string_regex = re.replace_all(&string_text_2, r"($1)");
    println!("string_text_2 replace with regex = {replace_string_regex}");
    using_graphemes();

}