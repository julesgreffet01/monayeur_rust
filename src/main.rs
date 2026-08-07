enum MonaiValue {
    Piece1Centime,
    Piece2Centime,
    Piece5Centime,
    Piece10Centime,
    Piece50Centimes,
    Piece1Euros,
    Piece2Eurois
}

struct Monai {
    quantity: i32,
    value: MonaiValue
}

fn get_value_monai(valeur_monaie: MonaiValue)-> i32 {
    match valeur_monaie {
        MonaiValue::Piece1Centime => 1,
        MonaiValue::Piece2Centime => 2,
        MonaiValue::Piece5Centime => 5,
        MonaiValue::Piece10Centime => 10,
        MonaiValue::Piece50Centimes => 50,
        MonaiValue::Piece1Euros => 100,
        MonaiValue::Piece2Eurois => 200
    }
}

fn main() {
    let piece = Monai {
        quantity: 1,
        value: MonaiValue::Piece10Centime,
    };

    println!("votre piece vaux : {} et vous en avez : {}", get_value_monai(piece.value), piece.quantity);
}
