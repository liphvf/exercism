use crate::accumulator::*; // Import the accumulator module

pub mod interators {

    pub fn interator() {

        println!("Iterator Exemple");

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
}

fn add(number1: i32, number2: i32) -> i32 {
    return number1 + number2;
    // também pode ser escrito como:
    // number1 + number2
    // o rust entende que a última expressão é o retorno da função
    // Rust is an expression-based language
}

fn struct_example() {
    // let accumulator = Accumulator { sum: 0 };
    let accumulator = Accumulator::new(0);

    println!("{:?}", accumulator);
}

// fn struct_example2() {
//     let mut accumulator = Accumulator::new(0);

//     for n in 3..10 {
//         accumulator = accumulator.add(n);
//     }

//     println!("{}", accumulator.get());
// }

fn struct_example3() {
    let mut accumulator = Accumulator::new(0);

    for n in 3..10 {
        accumulator.add(n);
        println!("acc {}", accumulator.get())
    }

    // println!("{}", accumulator.get());
}
