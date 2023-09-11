use std::ops::{Add,Sub,Mul,Div};
use std::cmp::Ordering;

#[derive(Debug,Copy,Clone)]

struct FiniteFieldElement{
    numero:u64,
    primo: u64,
}

impl FiniteFieldElement{

    fn new (numero:u64, primo: u64) -> FiniteFieldElement{
        if numero>=primo  {
            FiniteFieldElement {numero:0, primo:0}
        }else {
            FiniteFieldElement{numero:numero, primo:primo}
        }
    }

    fn pot(self,exp:u32)->FiniteFieldElement {
        let aux=self.numero.pow(exp)%self.primo;
        FiniteFieldElement{
            numero:aux,
            primo:self.primo
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

// se realiza la division aplicando el pequenio teorema de Fermat
// se aplica que a/b=afb^-1 ==>
// b^(p-1)= 1 --> b^-1 =b^(p-2)

impl Div for FiniteFieldElement{
    type Output = FiniteFieldElement;

    fn div(self, otro: FiniteFieldElement) -> FiniteFieldElement {
        // se contruye b^(p-2)
        let exp:u32= (self.primo - 2) as u32;
        let pow:FiniteFieldElement = otro.pot(exp);
        let resultado = self.numero * pow.numero;
        FiniteFieldElement { 
            numero: resultado % self.primo  , 
            primo: self.primo 
        }
    }
}

// sobrescribir la igualddad 
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

    let a=FiniteFieldElement::new(7,19);
    println!("elemento finito {:?}",a*a*a);

    //division 
    let a=FiniteFieldElement::new(2,19);
    let b=FiniteFieldElement::new(7,19);
    let c:FiniteFieldElement=a/b;

    println!("elemento finito {:?}",c);


}
