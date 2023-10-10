struct User {
    username: String,
    email: String
}
struct MyTupleStructure(i32, f64); // tuple struct with two fields
struct _MyUnitStructure; // unit struct

#[derive(Debug)]
enum ButtonPlayer {
    Play,
    Pause,
    Next,
    Prev,
    Stop
}
struct Comand {
    user: User,
    player: ButtonPlayer
}

fn complex_intern_struct(){
    struct ItemLista {
        dado: String,
        proximo: Option<Box<ItemLista>>,
    }
    
    let mut f_item = None;

    let itens = vec![
        "Item 1",
        "Item 2",
        "Item 3",
    ];

    for dado in itens {
        let novo_item = ItemLista {
            dado: dado.to_string(),
            proximo: f_item,
        };
        f_item = Some(Box::new(novo_item));
    }

    let iterador = std::iter::successors(f_item.as_deref(), |item| item.proximo.as_deref());

    for item in iterador {
        println!("t1 - data: {}", item.dado);
    }

    let mut item_atual = &f_item;

    while let Some(item) = item_atual {
        println!("t2 - data: {}", item.dado);
        item_atual = &item.proximo;
    }
}

fn main (){
    let my_tuple = MyTupleStructure(5, 3.14);
    println!("my_tuple is: ({} {})", my_tuple.0, my_tuple.1);
    let user1 = User {
        username: String::from("jpsamarino"),
        email: String::from("jpsamarino@jpsamarino")
    };
    let my_comand = Comand {
        user: user1,
        player: ButtonPlayer::Pause
    };
    println!("{:?} {:?} {:?}", my_comand.user.username, my_comand.user.email, my_comand.player);
    complex_intern_struct();
}