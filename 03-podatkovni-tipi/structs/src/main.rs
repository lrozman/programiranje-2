#[derive(Debug)]

struct AritmeticnoZaporedje {
    zacetni: i32,
    razlika: i32,
    trenutni_clen: i32,
} // Potrebujemo začetni čen in razliko, ker lahko iz tega izračunamo vse nadaljnje člene.
// Potrebujemo konstruktor. N-ti člen. ? Ne razumem, kaj naloga zahteva od mene,
// pa tudi če naj ne bi gledali naprej, je preveč vidno vse, kar piše pod 3.
// Js ne dojamem, kako nej bi to implementiral. Nič od tega.
// A potrebujemo tud izpis?

//Ukaz `(a.next();a.next())` naj tako za zaporedje z začetkom 1 in korakom 1 vrne `(1,2)`.
// Baje bi kle mogla bitvejica, da je par.
// Seprav zapordje hočjo, da skos ve, kje si.

use AritmeticnoZaporedje as AZ;

impl AZ {
    fn new(a0: i32, d: i32) -> AZ {
        return AZ {
            zacetni: a0,
            razlika: d,
            trenutni_clen: a0, }
    }

    fn next(&mut self) -> i32 {
        let clen = self.trenutni_clen;
        self.trenutni_clen += self.razlika;
        return clen
    }

    fn n_th(&self, n: u32) -> i32 {
        let mut clen = self.zacetni;
        for _ in 1..=n {
            clen += self.razlika
        }; // Lahko bi ga samo direkt zračunala.
        return clen
    }

    fn reset(&mut self) -> () { // Al kaj?
        self.trenutni_clen = self.zacetni;
    }

    fn current(&self) -> i32 {
        return self.trenutni_clen;
    }

    fn sum(&self, n: u32) -> i32 {
//        if n == 0 {
//            return 0;
//        } else {
//            return ((n as i32) * (self.zacetni + self.n_th(n-1))) / 2
//        };
//    }
    // Piše, da ne smeš uporabiti formule, ups.
        let mut vsota: i32 = 0;
        for i in 1..=n {
            vsota += self.n_th(i-1) // da za n=0 vrne nič, ampak začetni je pa indeks 0
        };
        return vsota
    }

    fn vsota(&self, zap: &AZ) -> AZ { // Sej to je tko, ane? Sam sešteješ začetni pa razliko?
        return AZ::new(self.zacetni + zap.zacetni, self.razlika + zap.razlika);
    }

    // produkt ?? A je to mišljeno členoma množenje? Za aritmetično se to sliši, da bo ratalo grdo, ker bo treba vsakič množit?
    // Ker ne dobiš ven aritmetičnega nujno, če se ne motim ??? Recimo dobiš zaporedje kvadratov, kar pa ni aritmetično ...
    // Torej ne morem napisati funkcije, da vrne AZ, ker pač v splošnem to en bo aritmetično zaporedje.
    // Lahko bi napisala nekaj, kar mi vrača i-ti člen novega zaporedja. Ampak iii.
    // Hoče, da sprejme še i, in vrne i-ti člen.
    fn produkt(&self, zap: &AZ, i:u32) -> i32 {
        return self.n_th(i) * zap.n_th(i)
    }
} // A lahko vračaš Self? Jaka je reku, da je to enkje v rust booku vidu.

struct GeometrijskoZaporedje {
    zacetni: i32,
    kvocient: i32,
    trenutni_clen: i32,
}

use GeometrijskoZaporedje as GZ;

impl GZ {
    fn new(a0: i32, q: i32) -> GZ {
        return GZ {
            zacetni: a0,
            kvocient: q,
            trenutni_clen: a0, }
    }

    fn next(&mut self) -> i32 {
        let clen = self.trenutni_clen;
        self.trenutni_clen = self.trenutni_clen * self.kvocient;
        return clen
    }

    fn n_th(&self, n: u32) -> i32 {
        let mut clen = self.zacetni;
        for _ in 1..=n {
            clen = clen * self.kvocient
        };
        return clen
    }

    fn reset(&mut self) -> () {
        self.trenutni_clen = self.zacetni;
    }

    fn current(&self) -> i32 {
        return self.trenutni_clen;
    }

    fn sum(&self, n: u32) -> i32 {
        let mut vsota: i32 = 0;
        for i in 1..=n {
            vsota += self.n_th(i-1) // da za n=0 vrne nič, ampak začetni je pa indeks 0
        };
        return vsota
    }

    fn vsota(&self, zap: &GZ, i: u32) -> i32 { // Tukaj je problem pr vsoti in ni pri produktu.
        return self.n_th(i) + zap.n_th(i)
    }


    fn produkt(&self, zap: &GZ) -> GZ {
        return GZ::new(self.zacetni * zap.zacetni, self.kvocient * zap.kvocient) // A je tko?
    }
} // A lahko vračaš Self? Jaka je reku, da je to enkje v rust booku vidu.

// struct Par {
// first: u8,
// second: u8,
//}
// use Par as P
// impl P {
//   fn new_pair(f: u8, s:u8) -> P{
//      return P {...}}}
// fn main() { let par2 = P::new_pair(2, 5)
// println!("{}", par2) ne bo delal, ker par2 nima implementirane metode display. Loh pa nardimo {:?}, ampak nima debuga, tkoda spet ne dela
// kako lahko to popravte? K mate struct, mu lahko recete, da vzame debug:}
// #[derive(Debug)]
// implemetirat, da vrne prvega: fn fst() -> u8
// Že če karkol maš, rabš referenco, k drgač zgbi lastništvo.
// AMpak ubistvu hpčeš metode, da loh verižš, pa da ni tko funkcija isto. Tkoda fn fst(self), sam spet je prevzel lastištvo, ker smo ga notr podal.
// Tkoda rabš met &self

//__________________________________________________________________________________________________________________________________
//Matematične izraze (in tudi programe) enostavno predstavimo z drevesi, ki jih tipično imenujemo abstraktno sintaktično drevo (AST).

// Najprej definirajte sledeči strukturi:
// ```rust
// enum BinOperacija {
//     Plus,
//     Minus,
//     Times,
// }

// enum Izraz {
//     Konstanta(u32),
//     Operacija(Box<Izraz>, BinOperacija, Box<Izraz>),
// }
// ```
// 1. Poskusite napisati tip za `Izraz` brez uporabe `Box` in skupaj s prevajalnikom razmislite, zakaj to ne deluje.
// 1. Razmislite, ali je potrebno v zapis izraza dodati tudi oklepaje, ali je dovolj že to, da drevo pravilno predstavlja izraz.
// 1. Zapišite primer izrazov za:
//     - `1 + (2 * 3)`
//     - `(1 + 2) * 3`
//     - `1 + 2 + 3`
//     - `5**2 + 3**2`
//     - `5 * 5 + 4**2`
// 1. Implementirajte metodo `eval`, ki izračuna vrednost izraza.
// 1. Implementirajte metodo `collect`, ki vrne število konstant v izrazu.
// 1. Implementirajte za izpisovanje `izpis`, ki vrne izraz v obliki `(a + b) * c`.
// Poskrbite, da boste pravilno izpisali oklepaje, vendar se ne obremenjujte, če izpište kakšen dodaten oklepaj.
// 1. Napišite nekaj primernih testov za metode `eval`, `collect` in `izpis`.
//----------------------------------------------------------------------------------------------------------------------------------

enum BinOperacija {
    Plus,
    Minus,
    Times,
    Potenca,
}

enum Izraz {
    Konstanta(u32),
    Operacija(Box<Izraz>, BinOperacija, Box<Izraz>),
}
//"recursive without indirection"- to napiše. In "insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle"
// Ampak ne vem, kaj to pomen, iskreno.

//Fora dreves je, da ne rabš oklepajev. Z drevesom loh predstavš vsak izraz. 2 * (4 + 5)
// Bo drevp iz zvezdice v 2 in v +, k gre pa v 4 in v 5
// Oklepaji so nekak ta poddrevesa. Ne morte sploh z dvojko pa štirko hkrati nč nardit.

impl Izraz {
    fn eval(&self) -> i32 {
        match self {
            Self::Operacija(i1, BinOperacija::Plus, i2) => i1.eval() + i2.eval(), // Zakaj ga te boxi nč ne motjo? Ne razumem, kaj se dogaja.
            Self::Operacija(i1, BinOperacija::Minus, i2) => i1.eval() - i2.eval(),
            Self::Operacija(i1, BinOperacija::Times, i2) => i1.eval() * i2.eval(),
            Self::Operacija(i1, BinOperacija::Potenca, i2) => i1.eval().pow(i2.eval().try_into().unwrap()), // Oni bi rajš, da jo sami definirajo, js bom dala power
            Self::Konstanta(k) => (*k).try_into().unwrap(),
        }
    }

    fn collect(&self) -> u32 {
        fn pomozna(izraz: &Izraz) -> u32 {
            match izraz {
                Izraz::Operacija(i1, _, i2) => pomozna(i1) + pomozna(i2),
                Izraz::Konstanta(_) => 1,
            }
        }
        return pomozna(self);
    }

    fn izpis(&self) -> String {
        match self {
            Self::Operacija(i1, BinOperacija::Plus, i2) => String::from("(") + &i1.izpis() + &String::from(" + ") + &i2.izpis() + &String::from(")"), // Zakaj ga te boxi nč ne motjo? Ne razumem, kaj se dogaja.
            Self::Operacija(i1, BinOperacija::Minus, i2) => String::from("(") + &i1.izpis() + &String::from(" - ") + &i2.izpis() + &String::from(")"),
            Self::Operacija(i1, BinOperacija::Times, i2) => String::from("(") + &i1.izpis() + &String::from(" * ") + &i2.izpis() + &String::from(")"),
            Self::Operacija(i1, BinOperacija::Potenca, i2) => String::from("(") + &i1.izpis() + &String::from("**") + &i2.izpis() + &String::from(")"), // Oni bi rajš, da jo sami definirajo, js bom dala power
            Self::Konstanta(k) => (*k).to_string(),
        }        // Piše, naj se ne obrememenjujem, če izpišem kakšenm dodaten oklepaj
    }
}

// PISANJE TESTOV !!!!!!!!!!!!!!!!!!

fn main() {
    //let mut z1 = GZ::new(1, 2);
    //println!("{}", z1.next());
    //println!("{}", z1.next());
    //println!("{}", z1.n_th(16));
    //z1.reset();
    //println!("{}", z1.current());
    //println!("{}", z1.sum(5));
    //let mut z2 = z1.produkt(&z1);
    //println!("{}", z2.n_th(4));
    let primer1: Izraz = Izraz::Operacija(
        Box::new(Izraz::Konstanta(1)), 
        BinOperacija::Plus, 
        Box::new(Izraz::Operacija(
            Box::new(Izraz::Konstanta(2)), 
            BinOperacija::Times, 
            Box::new(Izraz::Konstanta(3))))
    );
    let primer4: Izraz = Izraz::Operacija(
        Box::new(Izraz::Operacija(
            Box::new(Izraz::Konstanta(5)), BinOperacija::Potenca, Box::new(Izraz::Konstanta(2)))),
        BinOperacija::Plus,
        Box::new(Izraz::Operacija(
            Box::new(Izraz::Konstanta(3)), BinOperacija::Potenca, Box::new(Izraz::Konstanta(2))
        ))
    );
    let rezultat = primer4.eval();
    println!("{rezultat}");
    let collection = primer4.collect();
    println!("{collection}");
    let niz = primer1.izpis();
    println!("{}", niz);
}



// Pr AST si morš potenco dodat.
