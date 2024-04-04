use std::borrow::Borrow;

#[warn(non_snake_case)]
fn main() {
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
    struct_example3();
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

fn add(number1: i32, number2: i32) -> i32 {
    return number1 + number2;
    // também pode ser escrito como:
    // number1 + number2
    // o rust entende que a última expressão é o retorno da função
    // Rust is an expression-based language
}

fn interator() {
    let numbers = (3..10).inspect(|n| println!("n = {}", n));
    let sum = 0;

    let add = |n1: i32, n2: i32| n1 + n2;

    for number in numbers {
        add(sum, number);
    }

    println!("Sum of numbers is {}", sum)
}

fn interator2() {
    let numbers = (1..13)
        .inspect(|n| println!("before filter n = {}", n))
        .filter(|x| x % 2 == 0)
        .inspect(|n| println!("after filter n = {}", n));
    let sum = 0;

    let add = |n1: i32, n2: i32| n1 + n2;

    for number in numbers {
        add(sum, number);
    }

    println!("Sum of numbers is {}", sum)
}

fn interator3() {
    // Sum all odd numbers from 1 to 13
    // let numbers = (1..13).filter(|x| x % 2 == 0).fold(0, |acc, x| acc + x);
    // let numbers = (1..13).filter(|x| x % 2 == 0).sum::<i32>();
    let numbers: i32 = (1..13).filter(|x| x % 2 == 0).sum();

    println!("Sum of numbers is {}", numbers)
}

fn struct_example() {
    // let accumulator = Accumulator { sum: 0 };
    let accumulator = Accumulator::new(0);

    println!("{:?}", accumulator);
}

fn struct_example2() {
    let mut accumulator = Accumulator::new(0);

    for n in 3..10 {
        accumulator = accumulator.add(n);
    }


    println!("{}", accumulator.get());
}

fn struct_example3() {
    let mut accumulator = Accumulator::new(0);

    for n in 3..10 {
        accumulator = accumulator.add(n);
        println!("acc {}", accumulator.get())
    }


    // println!("{}", accumulator.get());
}

#[derive(Debug)] // utilizado para imprimir o conteúdo da struct
struct Accumulator {
    sum: i32,
}

// impl Accumulator { // isso é chamado de assosieated function
//     fn new(init: i32) -> Accumulator {
//         Accumulator { sum: init }
//     }
// }

// impl AccumulatorImutable {
//     // isso é chamado de assosieated function
//     fn new(sum: i32) -> Self {
//         // quando o nome da struct é o mesmo do tipo, pode ser utilizado Self
//         Self { sum } // quando o parametro tem o mesmo nome da propriedade, pode ser utilizado apenas o nome do parametro
//     }

//     fn get(self) -> i32 {
//         self.sum
//     }

//     fn add(self, incriment_value: i32) -> Self {
//         Self {
//             sum: self.sum + incriment_value
//         }
//     }
// }

impl Accumulator {
    // isso é chamado de assosieated function
    fn new(sum: i32) -> Self {
        // quando o nome da struct é o mesmo do tipo, pode ser utilizado Self
        Self { sum } // quando o parametro tem o mesmo nome da propriedade, pode ser utilizado apenas o nome do parametro
    }

    fn get(&self) -> i32 {
        self.sum
    }

    fn add(self, incriment_value: i32) -> Self {
        Self {
            sum: self.sum + incriment_value
        }
    }
}
