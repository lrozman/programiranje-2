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



fn main() {
    let mut z1 = GZ::new(1, 2);
    println!("{}", z1.next());
    println!("{}", z1.next());
    println!("{}", z1.n_th(16));
    z1.reset();
    println!("{}", z1.current());
    println!("{}", z1.sum(5));
    let mut z2 = z1.produkt(&z1);
    println!("{}", z2.n_th(4));
}



// Pr AST si morš potenco dodat.
