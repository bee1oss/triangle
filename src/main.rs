
struct Triangle {
    cat1: f64,
    cat2: f64,
}

impl Triangle {
    fn find_hyp(&self) -> f64 {
        (self.cat1 * self.cat1 + self.cat2 * self.cat2).sqrt()
    }
    fn 
}

fn main() {
    let tr1 = Triangle {
        cat1: 6.0,
        cat2: 8.0,
    };

    let hyp = tr1.find_hyp();

    println!("{}", hyp);
}
