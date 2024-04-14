#[derive(Debug)] // utilizado para imprimir o conteúdo da struct
pub struct Accumulator {
    sum: i32,
}

// impl Accumulator { // isso é chamado de assosieated function
//     fn new(init: i32) -> Accumulator {
//         Accumulator { sum: init }
//     }
// }

#[derive(Debug)] // utilizado para imprimir o conteúdo da struct
pub struct AccumulatorImutable {
    sum: i32,
}

impl Accumulator {
    // isso é chamado de assosieated function
    pub fn new(sum: i32) -> Self {
        // quando o nome da struct é o mesmo do tipo, pode ser utilizado Self
        Self { sum } // quando o parametro tem o mesmo nome da propriedade, pode ser utilizado apenas o nome do parametro
    }

    pub fn get(&self) -> i32 {
        self.sum
    }

    pub fn add(&mut self, incriment_value: i32) {
        self.sum += incriment_value
    }
}

impl AccumulatorImutable {
    // isso é chamado de assosieated function
    fn new(sum: i32) -> Self {
        // quando o nome da struct é o mesmo do tipo, pode ser utilizado Self
        Self { sum } // quando o parametro tem o mesmo nome da propriedade, pode ser utilizado apenas o nome do parametro
    }

    fn get(self) -> i32 {
        self.sum
    }

    fn add(self, incriment_value: i32) -> Self {
        Self {
            sum: self.sum + incriment_value
        }
    }
}