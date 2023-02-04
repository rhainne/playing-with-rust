#![allow(unused)]

use std::borrow::Borrow;
use std::f32::consts::PI;
use std::{io, num, array};
use rand::{Rng, Error};
use std::io::{ Write, BufReader, BufRead, ErrorKind };
use std::fs::File;
use std::cmp::Ordering;


fn chapter_01() {
    /* Declaración de variables mutables, print y readline */
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";

    io::stdin().read_line(&mut name)
        .expect("Didn't Receive Input");

    println!("Hello {}, {}", name.trim_end(), greeting);
}

fn chapter_02() {
    /* Declaración de constantes que no pueden deducir el tipo de datos. 
    * Declaración de la misma variable dos veces pero con distinto tipo de dato
    */
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "32";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;

    println!("I'm {} and I want ${}", age, ONE_MIL);
}

fn chapter_03() {
    /* Valores máximos algunos  tipos de datos */
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);
}

fn chapter_04() {
    /* Deducción de los tipos de datos en base a los que asignamos */
    let is_true = true;
    let my_grade = 'A';
}

fn chapter_05() {
    /* Distintos tipos con distinta precision */
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64: {}", num_2 + 0.111111111111111);
}

fn chapter_06() {
    /* Operadores básicos */
    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);

    num_3 += 1;
}

fn chapter_07() {
    /* Usando randoms */
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random : {}", random_num);
}

fn chapter_08() {
    /* If statement */
    let age = 8;
    if (age >= 1) && (age <= 18) {
        println!("Important birthday");
    } else if (age == 21) || (age == 50){
        println!("Otro important birthday");
    } else if age >= 65 {
        println!("Viejo important birthday");
    } else {
        println!("Not an important birthday");
    }
}

fn chapter_09() {
    /* Ternary if operator para asignar directamente a una variable */
    let mut my_age = 12;
    let can_vote = if my_age >= 18 { true } else { false };
    println!("Can vote : {}", can_vote);
}

fn chapter_10() {
    /* match with range of values */
    let age2 = 1;
    match age2 {
        1..=18 => println!("Important birthday"),
        21 | 50 => println!("21 o 50 birthday"),
        65..=i32::MAX => println!("Viejo birthday"),
        _ => println!("Not an important birthday"),
    }
}

fn chapter_11() {
    /* match + Ordering*/
    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less =>  println!("Cannot vote"),
        Ordering::Greater =>  println!("Can vote"),
        Ordering::Equal =>  println!("Congrats, welcome to the democracy party"),
    }
}

fn chapter_12() {
    /* Arrays */
    let arr_1 = [1,2,3,4];
    println!("1st: {}", arr_1[0]);
    println!("length: {}", arr_1.len());

    /* Looping arrays */
    let arr_2 = [1,2,3,4,6,7,8,9];
    let mut index = 0;
    loop {
        if arr_2[index] % 2 == 0 {
            index += 1;
            continue;
        }
        if arr_2[index] == 9 {
            break;
        }
        println!("Val: {}", arr_2[index]);
        index += 1;
    }
}

fn chapter_13() {
    /* while */
    let arr_2 = [1,2,3,4,6,7,8,9];
    let mut index = 0;

    while index < arr_2.len() {
        println!("Array: {}", arr_2[index]);
        index += 1;
    }
}

fn chapter_14() {
    /* for loop */
    let arr_2 = [1,2,3,4,6,7,8,9];
    let mut index = 0;

    for val in arr_2.iter() {
        println!("val: {}", val);
    }
}

fn chapter_15() {
    let my_tuple: (u8, String, f64) = (2, "adrus".to_string(), 0.12341);

    println!("Name: {}", my_tuple.1);
    let(v1, v2, v3) = my_tuple;

    println!("Name: {}", v1);

}

fn chapter_16() {
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str("drus");
    for word in st1.split_whitespace() {
        println!("{}", word);
    }

    let st2 = st1.replace("A", "Another");
    println!("{}", st2);
}

fn chapter_17() {
    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort(); //ordenar
    v1.dedup(); // quitar duplicados
    for char in v1 {
        println!("{}", char); // loop de un vector
    }

    /* Usar un string para construir otro? Pasar una referencia para construir una nueva instancia de string? */
    let st4 = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);

    /* Convertir un string en un array de bytes */
    let byte_arr1 = st5.as_bytes();

    /* trimmear un string y asignar esa referencia a otra variable */
    let st6 = &st5[0..6];
    println!("st6: {}", st6);
    // st5 = "Otra cadena".to_string(); // Este assignment no está permitido porque st5 es "borrowed" por st6 y se usa despues en el println(). 

    /* el tipo string tambien tiene la función len() */
    println!("String length: {}", st6.len());

    /* Vaciar un string */
    st5.clear();

    /* combining strings */
    let st6: String = String::from("Just some");
    let st7: String = String::from(" words");
    let st8 = st6 + &st7;

    for char in st8.bytes() {
        println!("{}", char);
    }

    // println!("{}", st6); // sentencia ilegal. borrowed of moved value. Si hubiesemos usado .clone() en la asignacion de st8, esta sentencia si sería legal. 
    println!("{}", st7); // sentencia legal, porque en la asignación de st8 hemos pasado la referencia de st7

}

fn chapter_18() {
    /* Casting */
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    println!("{}", int3_u32);
}

fn chapter_19() {
    /* Enums y las funciones asociadas a los Enums */
    enum Day {
        Lunes,
        Martes,
        Miercoles,
        Jueves,
        Viernes,
        Sabado,
        Domingo
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Sabado | Day::Domingo => true,
                _ => false
            }
        }
    }

    let today:Day = Day::Lunes;

    match today {
        Day::Lunes => println!("Que buena, lunes!"),
        Day::Martes => println!("Que buena, martes!"),
        Day::Miercoles => println!("Que buena, miercoles!"),
        Day::Jueves => println!("Que buena, jueves!"),
        Day::Viernes => println!("Que buena, viernes!"),
        Day::Sabado => println!("Que buena, sabado!!!"),
        Day::Domingo => println!("Que buena, domingo!"),
    }

    println!("Is today the weekend? {}", today.is_weekend());

}

fn chapter_20() {
    /* Vectores */
    let vec1: Vec<i32> = Vec::new();

    let mut vec2 = vec![1,2,3,4];
    vec2.push(5);

    println!("1st: {}", vec2[0]);

    let second: &i32 = &vec2[1]; // Asignamos el segundo valor del vector a una variable

    match vec2.get(1) {
        Some(second) => println!("2nd: {}", second), // Este second no es el mismo que el de la linea superior; es una nueva variable que, creo, contiene vec2.get(1)
        None => println!("No second value"),
    }

    for i in &mut vec2 {
        *i *= 2;
    }
    for i in &vec2 {
        println!("{}", i)
    }
    println!("vector lrnght: {}", vec2.len());
    println!("Pop: {:?}", vec2.pop());
}

fn say_hello() {
    println!("hola");
}

fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x+y);
}

fn get_sum_2(x: i32, y: i32) -> i32 {
    x + y // este es el valor que se retorna, sin necesidad del keyword, y sin cerrar con punto y coma
}

fn get_sum_3(x: i32, y: i32) -> i32 {
    return x + y; // o puedes especificar el return keyword
}

fn get_2(x: i32) -> (i32, i32) {
    return (x+1, x+2); // o puedes especificar el return keyword
}
fn call_to_get_2() {
    /* Llamada a get_2() */
    let (val_1, val_2) = get_2(2);
    println!("{}, {}", val_1, val_2);
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;

    for &val in list.iter() {
        sum += &val;
    }

    return sum;
}
fn call_to_sum_list() {
    /* Llamada a sum_list */
    let num_list = vec![1,2,3,4,5];
    println!("Sum of list = {}", sum_list(&num_list));
}


use std::ops::Add; // esto nos permite usar sumas con Generics

fn calls_to_get_sum_generics() {
    println!("5 + 4 = {}", get_sum_generics(5, 4));
    println!("5.2 + 4.6 = {}", get_sum_generics(5.2, 4.6));
    // println!("5.2 + 4.6 = {}", get_sum_generics('a', 'b')); // Llamada ilegal. The trait Add is not implemented for char
}
fn get_sum_generics<T: Add<Output = T>>(x: T, y: T) -> T { 
    return x + y;
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* Ownership */
fn ownership() {
    /* Explicando el concepto de borrow */
    let str1 = String::from("world");
    let str2 = str1;
    // println!("Hello {}", str1); // Illegal. str1 ya no existe. Ahora str2 tiene el valor que antes tenía str1

    let str3 = String::from("world");
    let str4 = str3.clone(); 
    println!("Hello {}", str3); // Sentencia legal, porque clonamos el valor originalmente.
}

fn print_str(x: String) {
    println!("A string {}", x);
}
fn print_return_string(x: String) -> String {
    println!("A string {}", x);
    return x;
}
fn change_string(name: &mut String) { //reference to string
    name.push_str(" is happy");
    println!("Message: {}", name);
}

fn call_to_ownership_funcs() {
    let str1 = String::from("World");
    let str2 = str1.clone();

    print_str(str2);

    let str3 = print_return_string(str1);
    println!("str3 = {}", str3);
}

fn call_to_change_string() {
    let mut str1 = String::from("adrus");
    change_string(&mut str1);
}

/* Una conclusión importante relacionada con ownership es que solo se nos permite tener una versión mutable de un valor. */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;
fn hashmaps() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Spiderman", "Peter Parker");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("Flash", "Barry Allen");

    for(k,v) in heroes.iter() {
        println!("{}, {}", k, v);
    }

    println!("Lenght: {}", heroes.len());

    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }
}

fn structs() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("555 Main Street"),
        balance: 3425.22
    };

    bob.address = String::from("Nueva dirección de Bob");

    struct Rectable<T, U> { // struct and generics working together
        lenght: T,
        height: U,
    }

    let rect = Rectable {
        lenght: 4, height: 10.5
    };

    /* we can also use traits with strucs and generics */
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct ImprovedRectangle { length: f32, width: f32 }
    struct Circle { length: f32, width: f32 }

    impl Shape for ImprovedRectangle {
        fn new(length: f32, width: f32) -> ImprovedRectangle {
            return ImprovedRectangle { length, width };
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }
    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    let rec: ImprovedRectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);

    println!("rect area: {}", rec.area());
    println!("circle area: {}", circ.area());
}

mod restaurant;
use crate::restaurant::order_food;
fn using_modules() {
    order_food();
}

fn using_panic() {
    // panic!("Terrible error");

    let lil_arr = [1,2];
    println!("{}", lil_arr[2]); //panic
}

fn using_files() {
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file: {:?}", error),
    };

    write!(output, "Just some\nRandom words\n").expect("Failed to write to the file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", error),
            },
            _other_error => panic!("Problem opening file: {:?}", error),
        }
    };
}

fn looping_arrays() {
    let mut arr_it = [1,2,3,4];

    /* no entiendo bien las diferencias entre las distintas maneras de recorrer un array */
    for val in arr_it.iter() {
        println!("{}", val);
    }
    for mut val in arr_it.iter() {
        val = &5;
        println!("{}", val);
    }
    println!("{:?}", arr_it);


    let mut arr_it2 = [4,3,2,1];
    /* no entiendo bien las diferencias entre las distintas maneras de recorrer un array */
    for val in arr_it2.into_iter() {
        println!("{}", val);
    }
    println!("{:?}", arr_it2);


    let mut iter1 = arr_it.iter();
    println!("1st value: {:?}", iter1.next());

}

fn closures() { // anonymous funtions?
    // let var_name = |parameters| -> return_type {body}
    let can_vote = |age: i32| {
        age >= 18
    };
    println!("Can vote: {}", can_vote(90));

    /* las closures pueden acceder a variables del scope superior */
    let mut sampl1 = 5;
    let print_var = || println!("samp1 = {}", sampl1);
    print_var();

    /* puedes cambiar el valor de las variables si la funcion está declarada como mut */
    sampl1 = 10;
    let mut change_var = || sampl1 += 1;
    change_var();
    println!("sampl1 = {}", sampl1);
    sampl1 = 10;
    println!("sampl1 = {}", sampl1);
}

/* podemos declarar funciones dentro de otras funciones */
fn outer_function() {
    /* Usando generics podemos decirle a una función que acepte cualquier funcion que a su vez acepte 2 parámetro i32 y devuelve un i32 */
    fn use_function<T>(a: i32, b: i32, func: T) -> i32 
    where T: Fn(i32, i32) -> i32 {
        /* el cometido de nuestra función use_function será solo pasar los parámetros a cualquier otra función que se esté pasando por parámetro */
        func(a, b)
    }

    /* Me obliga a hacer explícito el tipo de datos de a y de b. A Derek no le obliga */
    let sum = |a:i32, b:i32| a + b;
    let product = |a:i32, b:i32| a * b;
    let division = |a:i32, b:i32| a / b;

    println!("5 + 4 = {}", use_function(5,4, sum));
    println!("5 * 4 = {}", use_function(5,4, product));
    println!("5 / 4 = {}", use_function(5,4, division));
}

fn smart_pointers() {
    // Box smart pointer
    let b_int1 = Box::new(10);
    println!("{}", b_int1);

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode { left: None, right: None, key }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self // return value
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self // return value
        }
    }

    let node1 = TreeNode::new(1).left(TreeNode::new(2)).right(TreeNode::new(3));

    /* how do we develop a printing function for our type TreeNode? */


}

use std::thread;
use std::time::Duration;
fn concurrency() {
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread: {}", i);
            thread::sleep(Duration::from_millis((1)));
        }
    });

    for i in 1..20 {
        println!("Main thread {}", i);
        thread::sleep(Duration::from_millis((1)));
    }

    // sin esta linea thread1 nisiquiera termina su ejecución, no se muy bien porqué
    thread1.join().unwrap(); 
}

fn banking_example() {
    /* El código siguiente está tomando bank, que pertenece a banking_example(), 
     * pero, como se llama desde un thread, podría "outlive" banking_example()
     * y podría generar todo tipo de problemas.   
    */

    pub struct Bank {
        balance: f32,
    }

    fn withdraw(the_bank: &mut Bank, amount: f32) {
        the_bank.balance -= amount;
    }
    let mut bank = Bank{ balance: 100.0 };
    withdraw(&mut bank, 5.0);
    println!("Balance: {}", bank.balance);

    fn customer(the_bank: &mut Bank) {
        withdraw(the_bank, 5.0);
    }

    /* Esta siguiente linea es la linea ilegal. El ejemplo bank_with_smart_pointers() soluciona este problema */
    // thread::spawn(|| {
    //     customer(&mut bank);
    // }).join().unwrap();
}

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{ Arc, Mutex };
fn bank_with_smart_pointers() {
    pub struct Bank {
        balance: f32,
    }

    fn widthdraw(the_bank: &Arc<Mutex<Bank>>, amount: f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!("Current balance: {} Withdrawal a smaller amount", bank_ref.balance);
        } else {
            bank_ref.balance -= amount;
            println!("Customer withdrew {}$. Current balance: {}", amount,  bank_ref.balance);
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>) {
        widthdraw(&the_bank, 5.00);
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank {balance: 50.00}));
    let handles = (0..15).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref);
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total: {}", bank.lock().unwrap().balance);

}

fn main() {
    bank_with_smart_pointers();
}