// pub mod exemple1;

pub mod exemple1 {

    pub fn interator() {
        let numbers = (3..10).inspect(|n| println!("n = {}", n));
        let sum = 0;

        let add = |n1: i32, n2: i32| n1 + n2;

        for number in numbers {
            add(sum, number);
        }

        println!("Sum of numbers is {}", sum)
    }
}
