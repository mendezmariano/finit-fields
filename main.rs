use std::ops::{Add,Sub,Mul,Div};
use std::cmp::Ordering;

#[derive(Debug)]

struct FiniteFieldElement{
    numero:u32,
    primo: u32,
}

impl FiniteFieldElement{

    fn new (numero:u32, primo: u32) -> FiniteFieldElement{
        if numero>=primo  {
            FiniteFieldElement {numero:0, primo:0}
        }else {
            FiniteFieldElement{numero:numero, primo:primo}
        }
    }
}

impl Add for FiniteFieldElement {
    type Output = FiniteFieldElement;

    fn add(self, otro: FiniteFieldElement) -> FiniteFieldElement {
        FiniteFieldElement { 
            numero: (self.numero + otro.numero)% self.primo , 
            primo: self.primo 
        }
    }
}

impl Sub for FiniteFieldElement{
    type Output = FiniteFieldElement;

    fn sub(self, otro: FiniteFieldElement) -> FiniteFieldElement {
        FiniteFieldElement { 
            numero: (self.numero - otro.numero)% self.primo , 
            primo: self.primo 
        }
    }
}


impl Mul for FiniteFieldElement{
    type Output = FiniteFieldElement;

    fn mul(self, otro: FiniteFieldElement) -> FiniteFieldElement {
        FiniteFieldElement { 
            numero: (self.numero * otro.numero)% self.primo , 
            primo: self.primo 
        }
    }

}

impl Div for FiniteFieldElement{
    type Output = FiniteFieldElement;

    fn div(self, otro: FiniteFieldElement) -> FiniteFieldElement {
        FiniteFieldElement { 
            numero: (self.numero + otro.numero)% self.primo , 
            primo: self.primo 
        }
    }
}

impl PartialEq for FiniteFieldElement {
    fn eq(&self, otro: &Self) -> bool {
        self.numero == otro.numero && self.primo== otro.primo
    }
}


fn main(){

    let a=FiniteFieldElement::new(7,13);
    let b=FiniteFieldElement::new(12,13);
    let c:FiniteFieldElement=a+b;

    println!("elemento finito {:?}",c);
    assert_eq!(FiniteFieldElement{numero:6,primo:13}, c);
    let a=FiniteFieldElement::new(8,19);
    let b=FiniteFieldElement::new(17,19);
    let c:FiniteFieldElement=a*b;

    println!("elemento finito {:?}",c);
}
