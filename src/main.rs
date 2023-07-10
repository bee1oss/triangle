#[derive(Debug)]
struct Triangle {
    cat1: f64,
    cat2: f64,
}
//methods struct Triangle
impl Triangle {
    fn find_hyp(&self) -> f64 {
        (self.cat1 * self.cat1 + self.cat2 * self.cat2).sqrt()
    }
    fn find_area(&self)->f64{
        0.5*self.cat1*self.cat2
    }
    fn is_eq(&self,ar:f64)->bool{
        self.find_area()<ar
    }
    fn create_isc(cat1:f64,cat2:f64)->Triangle{
        Triangle{
            cat1: cat1,
            cat2: cat2
        }
    }
}

fn main() {
    let tr1 = Triangle {
        cat1: 6.0,
        cat2: 8.0,
    };
    let tr2 = Triangle{
        cat1: 3.0,
        cat2: 4.0,
    };


    let hyp = tr1.find_hyp();
    let area = tr1.find_area();
    /*let isc_tr=Triangle::create_isc(5.0,5.0);
    let hyp = isc_tr.find_hyp();
    let area = isc_tr.find_area();

    println!("Isc:{:#?}", isc_tr);*/
    let result = tr2.is_eq(tr1.find_area());
    println!("Hyp:{}", hyp);
    println!("Area:{}", area);
    //println!("Result:{}", result);
    check(result);

}

fn check(is_what:bool){
    if is_what==true {
        println!("Sigiyor!!");
    }
    else {
        println!("Sigmiyor!!");
    }
}