use std::ops::{Add, Deref, Mul};

// struct AritmeticnoZaporedje<T> {
//     zacetni_clen: T,
//     divergenca: T
// }

// use std::process::Output;

// use AritmeticnoZaporedje as Az;

// impl<T> Az<T>
// where 
//     T: Copy
//     + Add<Output = T>
//     + Mul<Output = T>
//  {
//     fn new (a0: T, d: T) -> Az<T> { //ce ze tm pr impl napism <T> nerabm usakic pr funkciji pisat
//         return Az{
//             zacetni_clen: a0,
//             divergenca: d
//         };
//     }
//     fn n_ti_clen(&self, n: T) -> T {
//         return self.zacetni_clen + n * (self.divergenca);
//     }
//     fn vsota_zaporedji(&self, other: &Az<T>) -> Az<T> { //tuki ne dela ce je referenca na other?
//         let zaporedje = Az{
//             zacetni_clen: self.zacetni_clen + other.zacetni_clen,
//             divergenca: self.divergenca + other.divergenca
//         };
//         return zaporedje
//     }
    
//     //fn sum(self, n: i32) -> T { kaj tuki nardis s tipi (a use spremenis v T, al skombiniras)
//     //    let mut vsota = 0;
//     //    for i in 1..n {
//     //        vsota += self.n_ti_clen(i);
//     //    }
//     //    return vsota;
//     //}

//     fn mul (&self, other: Az<T>) -> Az<T> {
//         let zaporedje = Az{
//             zacetni_clen: self.zacetni_clen * other.zacetni_clen,
//             divergenca: self.divergenca * other.divergenca
//         };
//         return zaporedje
    
//     }
// }
// impl<T> PartialEq for Az<T>
// where
//     T : PartialEq 
// {
//     fn eq(&self, other: &Self) -> bool {
//        self.zacetni_clen == other.zacetni_clen && self.divergenca == other.divergenca 
//     }
// }
//------------------------------------------------Generiki-------------------------------------------------------------
// use std::any::type_name;
// use std::ops::{Sub, Rem};
// struct Zaporedje<T> {
//     zacetni_clen: T,
//     divergenca: T
// }
// impl<T> Zaporedje<T>
// where
//     T: Copy + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + Default
// {
//     fn name(&self) -> String {
//         format!("Zaporedje of type: {}", type_name::<T>())
//     }
//     fn start(&self) -> T {
//         return self.zacetni_clen
//     }
//     fn k_th(&self, k: u32) -> T {
//         if k == 0 {
//             return self.zacetni_clen;
//         }
//         self.divergenca + self.k_th(k - 1)
//     }
//     fn contains(&self, a_i: T) -> bool {
//         let diff = a_i - self.zacetni_clen;
//          self.zacetni_clen == a_i || diff % self.divergenca == T::default()  //ce divergenca deli (a_i - self.zacetni_clen)
//     }
// }
//---------------------------------------------------------Trait-i--------------------------------------------------------------------
trait Sequence<T> {
    fn name(&self) -> String;
    fn k_th(&self, k: usize) -> Option<T>; // Option rabimo ce je prazen
    fn contains(&self, item: T) -> bool;
    fn start(&self) -> T;
}

struct Constant<T> {
    c: T
}
 struct ConstantInteger {
    c : i64
 }

impl<T> Constant<T> {
    fn new(c : T) -> Constant<T> {
        Constant {c}
    }
}
impl ConstantInteger {
    fn new(c: i64) -> ConstantInteger {
        ConstantInteger{c}
    }
}

// impl<T> Sequence<T> for Constant<T> { //ful splosno za zaporedja zato nc ne dela ker potrebujemo ful enih dodatnih traitov
//     fn name(&self) -> String {
//         format!("Constant")
//     }
//     fn k_th(&self, k: usize) -> Option<T> {
//         return Some (self.c)
//     }
//     fn contains(&self, item: T) -> bool {
//         return item == self.c
//     }

// }

impl Sequence<i64> for Constant<i64> { //za zaporedja i64
    fn name(&self) -> String {
        format!("Constant")
    }
    fn k_th(&self, _: usize) -> Option<i64> {
        return Some (self.c);
    }
    fn contains(&self, item: i64) -> bool {
        return item == self.c;
    }
    fn start(&self) -> i64 {
        return self.c;
    }
}
struct AritmeticnoZaporedje<T> {
    zacetni_clen: T,
    divergenca: T
}
impl Sequence<i64> for AritmeticnoZaporedje<i64> {
    fn name(&self) -> String {
        format!("Stevilsko zaporedje")
    }
    fn start(&self) -> i64 {
        return self.zacetni_clen
    }
    fn k_th(&self, k: usize) -> Option<i64> {
        if k == 0 { //zaustavitveni pogoj
            return Some(self.zacetni_clen);
        }
        if self.zacetni_clen == 0 && self.divergenca == 0 { //prazno zaporedje
            return None
        }
        match self.k_th(k - 1) {
            Some(a) => Some(self.divergenca + a),
            None => None
        }
    }
    fn contains(&self, a_i: i64) -> bool {
        let diff = a_i - self.zacetni_clen;
         self.zacetni_clen == a_i || diff % self.divergenca == i64::default()  //ce divergenca deli (a_i - self.zacetni_clen)
    }
}
struct GeometrijskoZaporedje<T> {
    zacetni_clen: T,
    kvocient: T
}

impl Sequence<i64> for GeometrijskoZaporedje<i64> {

    fn name(&self) -> String {
        format!("Geometrijsko zaporedje")
    }
    fn start(&self) -> i64 {
        return self.zacetni_clen
    }
    fn k_th(&self, k: usize) -> Option<i64> {
        if self.zacetni_clen == 0 && self.kvocient == 0 {
            return None;
        }
        Some(self.zacetni_clen * self.kvocient.pow(k as u32)) //pow je ubistvu potenciranje
    }
    fn contains(&self, a_i: i64) -> bool {
        if self.zacetni_clen == 0 {
            return a_i == 0;
        }
        if a_i % self.zacetni_clen != 0 {
            return false;
        }
        let ratio = a_i / self.zacetni_clen;
        let mut n = ratio;
        while n > 1 { //to ne vem ce je prou, poglej se enkrat
            if n % self.kvocient != 0 {
                return false;
            }
            n /= self.kvocient;
        }
        true
    }
}
struct Fibonacci<T> {
    zacetni_clen1: T,
    zacetni_clen2: T
}
fn is_perfect_square(n: i64) -> bool {
    let sqrt_n = (n as f64).sqrt() as i64;
    sqrt_n * sqrt_n == n
}
impl Sequence<i64> for Fibonacci<i64> {
    fn name(&self) -> String {
        format!("Fibonaccijevo zaporedje")
    }
    fn start(&self) -> i64 {
        return self.zacetni_clen1
    }
    fn k_th(&self, k: usize) -> Option<i64> { //kaj morm tuki narest?
        // if k == 0 {return Some(self.zacetni_clen1)};
        // if k == 1 {return Some(self.zacetni_clen2)}
        // else {
        //     for i in 1 .. k {
        //         let next = self.zacetni_clen1 + self.zacetni_clen2;
        //         self.zacetni_clen1 = self.zacetni_clen2;
        //         self.zacetni_clen2 = next
        //         
        //     }
        // return Some(self.zacetni_clen2)
        // }
        return None 
    }
    fn contains(&self, a_i: i64) -> bool {
        let x1 = 5 * a_i * a_i + 4;
        let x2 = 5 * a_i * a_i - 4;
        is_perfect_square(x1) || is_perfect_square(x2)
    }
}
fn main() {
    let zaporedje = GeometrijskoZaporedje{zacetni_clen: 3, kvocient: 2};
    println!("{} clen zaporedja je {:?}", 10, zaporedje.k_th(10));
}
