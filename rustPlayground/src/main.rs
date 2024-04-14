use std::borrow::Borrow;

mod exemples;
mod accumulator;

#[warn(non_snake_case)]
fn main() {
    exemples::interators::interator();
    // let values = [1, 2, 3, 4, 5];

    // let mut sum = 0;

    // for value in values {
    //     sum = add(sum, value);
    // }

    // // for index in 0..values.len() {
    // //     sum += values[index];
    // // }
    // // 0..values.len() é uma forma de obter o intervalo de 0 até o tamanho do array values
    // // poderia ser escrito 0..5, mas é melhor usar values.len() para evitar erros

    // println!("Sum of values is {}", sum);

    // // Vamos utilizar o slice para somar apenas os 2 primeiros elementos

    // let mut sum_of_two_frist_element = 0;
    // for value in &values[0..2] {
    //     sum_of_two_frist_element = add(sum_of_two_frist_element, *value);
    // }

    // println!("Sum of two first elements is {}", sum_of_two_frist_element);

    // // Vamos agora trabalhar com vector

    // let mut values_vec = vec![1, 2];
    // values_vec.push(3);
    // values_vec.push(4);

    // // Closures são funções anônimas que podem capturar variáveis do escopo em que são definidas
    // // São similares a lambdas em outras linguagens

    // let sum_closure = |number1: i32, number2: i32| number1 + number2;
    // let my_closures_sum = sum_closure(10, 20);
    // println!("Sum using closures: {}", my_closures_sum);

    // let sum_range_closure = |from: i32, to: i32| -> i32 {
    //     let mut sum = 0;
    //     for value in from..to {
    //         sum += value;
    //     }
    //     sum // também pode ser escrito como "return sum;"
    // };

    // let my_range_sum = sum_range_closure(1, 5);
    // println!("Sum using range closures: {}", my_range_sum);

    // interator();
    // interator2();
    // interator3();
    // struct_example();
    // struct_example2();
    // struct_example3();
}

// fn main() {
//     println!("Hello, world!");

//     let a = 10; // também pode ser escrito como let a: i32 = 10;
//     let b = 20;
//     let sum = add(a, b);

//     // sum += b; // não é possível, pois sum é uma variável imutável
//     // Rust é uma linguagem de programação com variáveis imutáveis por padrão
//     // Para tornar uma variável mutável, é necessário adicionar a palavra-chave mut
//     // let mut sum = add(a, b);
//     // sum += b;

//     println!("Sum of {} and {} is {}", a, b, sum);
// }
