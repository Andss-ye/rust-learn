fn main() {
    let s1 = String::from("hola");
    let _len = calcular_longitud(&s1);
    println!("La longitud de '{}' es {}. {}", s1, calcular_longitud(&s1), _len);
    println!("el valor original es: {}", s1);

    let mut s2 = String::from("adios");
    modificar(&mut s2);
    println!("el valor modificado es: {}", s2);

    let mut s2 = String::from("andrew");
    {
        let _r1 = &mut s2;
    }
    let _r2 = &mut s2;

    let _referencia_a_la_nada = colgar();
}

fn calcular_longitud(s: &String) -> usize {
    s.len()
}

fn modificar(s: &mut String) {
    s.push_str(" kart");
}

fn colgar() -> String {
    let s = String::from("hello");
    s
}