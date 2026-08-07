use std::io;

enum MonaiValue {
    Piece1Centime,
    Piece2Centime,
    Piece5Centime,
    Piece10Centime,
    Piece50Centime,
    Piece1Euros,
    Piece2Euros
}

struct Monai {
    quantity: i32,
    value: MonaiValue
}

struct Product {
    name: String,
    price: i32
}

fn get_value_monai(valeur_monaie: MonaiValue)-> i32 {
    match valeur_monaie {
        MonaiValue::Piece1Centime => 1,
        MonaiValue::Piece2Centime => 2,
        MonaiValue::Piece5Centime => 5,
        MonaiValue::Piece10Centime => 10,
        MonaiValue::Piece50Centime => 50,
        MonaiValue::Piece1Euros => 100,
        MonaiValue::Piece2Euros => 200
    }
}

fn main() {

    let products = vec![
        Product {
            name: String::from("Pates"),
            price: 200,
        },
        Product {
            name: String::from("Epices"),
            price: 20,
        }
    ];

    // let pieces = vec![
    //     Monai {
    //         quantity: 10,
    //         value: MonaiValue::Piece1Centime,
    //     },
    //     Monai {
    //         quantity: 10,
    //         value: MonaiValue::Piece2Centime,
    //     },
    //     Monai {
    //         quantity: 10,
    //         value: MonaiValue::Piece5Centime,
    //     },
    //     Monai {
    //         quantity: 10,
    //         value: MonaiValue::Piece10Centime,
    //     },
    //     Monai {
    //         quantity: 10,
    //         value: MonaiValue::Piece50Centime,
    //     },
    //     Monai {
    //         quantity: 10,
    //         value: MonaiValue::Piece1Euros,
    //     },
    //     Monai {
    //         quantity: 10,
    //         value: MonaiValue::Piece2Euros,
    //     },
    // ];

    let mut input_product = String::new();


    for product in products {
        println!("{} : {} €", product.name, product.price as f64 / 100.0);
    }

    println!("Quel produit souhaitez vous ?");

    io::stdin()
    .read_line(&mut input_product)
    .expect("Erreur pendant la lecture du produit");

    let mut price_to_pay: i32 = 0;

    let product_wanted = input_product.trim();
    
    if product_wanted == "Pates" {
        price_to_pay = 200;
    } else if product_wanted == "Epices" {
        price_to_pay = 20;
    } else {
        println!("oh ca focntionne pas");
    }

    print!("Le prix a payer est de : {} €", price_to_pay as f32 / 100.0);

    //demander piece par piece si il en donne puis calculer et redonner si necessaire




}
