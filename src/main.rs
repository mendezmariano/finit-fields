use core::panic;
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
            panic!("FiniteFielElement error : {:?}","Primo no puede ser 0" )
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
        if otro.primo!= self.primo{
            panic!("FiniteFielElement error : {:?}","los numeros deben pertenercer al mismo Finite FIeld " )
        }
        FiniteFieldElement { 
            numero: (self.numero + otro.numero)% self.primo , 
            primo: self.primo 
        }
    }
}

impl Sub for FiniteFieldElement{
    type Output = FiniteFieldElement;

    fn sub(self, otro: FiniteFieldElement) -> FiniteFieldElement {
        if otro.primo!= self.primo{
            panic!("FiniteFielElement error : {:?}","los numeros deben pertenercer al mismo Finite FIeld " )
        }
        FiniteFieldElement { 
            numero: (self.numero - otro.numero)% self.primo , 
            primo: self.primo 
        }
    }
}


impl Mul for FiniteFieldElement{
    type Output = FiniteFieldElement;

    fn mul(self, otro: FiniteFieldElement) -> FiniteFieldElement {
        if otro.primo!= self.primo{
            panic!("FiniteFielElement error : {:?}","los numeros deben pertenercer al mismo Finite FIeld " )
        }
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
        if otro.primo!= self.primo{
            panic!("FiniteFielElement error : {:?}","los numeros deben pertenercer al mismo Finite FIeld " )
        }
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
    
    let a=FiniteFieldElement::new(8,19);
    let b=FiniteFieldElement::new(17,19);
    let c:FiniteFieldElement=a*b;

    println!("elemento finito {:?}",c);

    let a=FiniteFieldElement::new(17,19);
    let b=FiniteFieldElement::new(8,19);
    let c:FiniteFieldElement=a-b;

    println!("elemento finito {:?}",c);





    let a=FiniteFieldElement::new(7,19);
    println!("elemento finito {:?}",a*a*a);

    //division 
    let a=FiniteFieldElement::new(2,19);
    let b=FiniteFieldElement::new(7,19);
    let c:FiniteFieldElement=a/b;

    println!("elemento finito {:?}",c);


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_when_add_two_ffd_a_new_ffe_with_addition_is_created() {
        let a = FiniteFieldElement::new(7, 13);
        let b = FiniteFieldElement::new(12, 13);
        let c:FiniteFieldElement=a+b;
        assert_eq!(FiniteFieldElement{numero:6,primo:13}, c);
       
    }

    #[test]
    fn test_when_sub_two_ffd_a_new_ffe_with_substraction_is_created() {
        let a = FiniteFieldElement::new(17, 19);
        let b = FiniteFieldElement::new(8, 19);
        let c:FiniteFieldElement=a-b;
        assert_eq!(FiniteFieldElement{numero:9,primo:19}, c);
       
    }

    #[test]
    fn test_when_mult_two_ffd_a_new_ffe_with_mult_is_created() {
        let a = FiniteFieldElement::new(8, 19);
        let b = FiniteFieldElement::new(17, 19);
        let c:FiniteFieldElement=a*b;
        assert_eq!(FiniteFieldElement{numero:3,primo:19}, c);
       
    }


    #[test]
    fn test_when_div_two_ffd_a_new_ffe_with_div_is_created() {
        let a = FiniteFieldElement::new(2, 19);
        let b = FiniteFieldElement::new(7, 19);
        let c:FiniteFieldElement=a/b;
        assert_eq!(FiniteFieldElement{numero:3,primo:19}, c);
       
    }




    // #[test]
    // fn test_when_sub_two_ffd_with_diff_primos_an_error_occurs() {
    //     let a = FiniteFieldElement::new(12, 19);
    //     let b = FiniteFieldElement::new(7, 13);
        
    //     assert_eq!(FiniteFieldElement{numero:9,primo:13}, c);
       
    // }
}