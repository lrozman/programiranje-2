use std::ops::Add;
use std::ops::Mul;

#[derive(Debug)]

struct AritmeticnoZaporedje<T>{
    zacetni: T,
    razlika: T,
    trenutni_clen: T,
}

// Kaj bo pa kle razlika, i don't get it.

//btw {:?} v println-ju je z debugom, drgač pa loh, če ma stvar implementirat to.string

// In ne, nimajo vse metode smisla. Npr produkt, ena izmed njih.

//NAVODILA
// Za naloge, ki sledijo, boste potrebovali strukturo aritmetičnega zaporedja, ki ste ga definirali zadnjič in strukturo izrazov in binarnih operacij.
// Popravite definicijo aritmetičnega zaporedja od zadnjič, da bo delovala za poljubne tipe T. Ali imajo vse metode od zadnjič smisel? 
    // Nimajo
// Ugotovi, katerim značilnostim mora zadoščati tip T, da bodo imele metode od zadnjič smisel. Za tak tip T, popravi implementacijo metod od zadnjič. Zmnožek dveh aritmetičnih zaporedij definiramo tako, 
//da zmnožimo začetna člena in diferenci (zato da bo produkt dveh aritmetičnih zaporedij tudi aritmetično zaporedje).
    // Kateri Traiti? Add, Mul, nek unit ???

// NI ME BILO NA TEH VAJAH, ZATO NIMAM POJMA, KAJ BI MOGLA NAREST IN KAKO SE TO NARDI
use AritmeticnoZaporedje as AZ;

impl<T: Add<Output = T> + Clone> AZ<T>  {
    fn new(a0: T, d: T) -> Self {
        return Self {
            zacetni: a0.clone(),
            razlika: d,
            trenutni_clen: a0, } //ojoj, kle so zdej ceeeeeli problemi z referencami, ker to ni copy-able. Bom dala clone, da je več možnosti
    }

    fn next(&mut self) -> T {
        let clen = self.trenutni_clen.clone(); // A rabm js zdej kle reference? Pa a nej tud vračam reference?
        self.trenutni_clen = self.trenutni_clen.clone() + self.razlika.clone();
        return clen // sam to je kr en drek zdej ?
    }

    fn n_th(&self, n: u32) -> T {
        let mut clen = self.zacetni.clone(); // sam to je zihr cist narobe. Ne morm kr vsega cloneat. Sam sej drgač ne morm, če hočm met lastništvo ...
        for _ in 1..=n {
            clen = clen + self.razlika.clone()
        }; // Lahko bi ga samo direkt zračunala.
        return clen
    }

    fn reset(&mut self) -> () { // Al kaj?
        self.trenutni_clen = self.zacetni.clone();
    }

    fn current(&self) -> &T { // AL KAJ ??? Kako je tuki z lasntištvom in kako to narest?
        return &self.trenutni_clen;
    }

    fn sum(&self, n: u32) -> T {

        let mut vsota: T = self.zacetni.clone();
        for i in 2..=n {
            vsota = vsota + self.n_th(i-1)
        };
        return vsota
    } // Ker bi drgač rabla nevtralni element, grem začet s prvim členom rajš.

    fn vsota(&self, zap: &Self) -> Self { // Sej to je tko, ane? Sam sešteješ začetni pa razliko?
        return AZ::new(self.zacetni.clone() + zap.zacetni.clone(), self.razlika.clone() + zap.razlika.clone());
    }
// Pa js bom kr tkole raj nardila, ker je Mul kr restriktivno
}

impl<T: Mul<Output = T> + Add<Output = T> + Clone> AZ<T> { // Karkol za restriction sploh pomen
    fn produkt(&self, zap: &Self, i:u32) -> Self {
        return AZ::new(self.zacetni.clone() * zap.zacetni.clone(), self.razlika.clone() * zap.razlika.clone())
    }
} // Ampak to je vse jadno

//Pr splošnih se ne ukvarjat s potenco

//NAVODILA
// Implementirajte značilnost PartialEq za aritmetična zaporedja.
// Definirajte značilnost Zaporedje<T>, ki predstavlja poljubno zaporedje in ima metode name, start, k_th in contains.
// Definirajte: konstantno zaporedje, aritmetično zaporedje, geometrijsko zaporedje, zaporedje Fibonaccijevih števil.
    // ???????????? Kaj morm narest? A v mainu? Al nej bi bli to structi? Pa az že mam???
// Definirajte zamaknjeno_zaporedje, ki sprejme zaporedje in število n in vrne zaporedje, ki se začne z n-tim členom vhodnega zaporedja.
// Definirajte zaporedje Combined, ki sprejme aritmetični izraz (s spremenljivkami) in seznam zaporedij (s pravilnimi imeni) in vrne kombinirano zaporedje, 
//kjer je i-ti člen izračunan z uporabo izraza in vrednosti členov iz vhodnih zaporedij.
// Popravite Izraz tako, da bo konstanta v izrazu poljubnega tipa T.
// Katerim značilnostim mora zadoščati tip T, da bo imela metoda eval smisel? Kaj pa collect  in izpis?
// Za Izraz implementirajte značilnost ToString.
// Ustvarite nekaj aritmetičnih zaporedij, in testirajte operacije na njih.

impl<T: PartialEq> PartialEq for AZ<T> { // A lahko to zahtevam od tipa?
    fn eq(&self, other: &Self) -> bool {
        return (self.zacetni == other.zacetni) && (self.razlika == other.razlika)
    } // al kaj? Ja, to je tko.
}

trait Zaporedje<T> {
    fn name(&self) -> String;
    fn start(&self) -> &T;
    fn k_th(&self, k: u32) -> T;
    fn contains(&self) -> bool; //nimam pojma, kaj naj bi to vračal
}

struct KonstantnoZaporedje<T> {
    zacetni: T, // sej druzga ne rabiš, ne? Al rabim zard traita zgori ? Ne sej ne?
}

struct GeometrijskoZaporedje<T> {
    zacetni: T,
    kvocient: T,
    trenutni_clen: T
}

// A Fibonaccijevo ... ???
struct Fibonaccijevo {
    trenutni_clen: u32 //pač ostalo o tem zaporedju itak vemo ????????
}

fn zamaknjeno_zaporedje<T, Z: Zaporedje<T>>(zap: &Z, n: u32) -> Z {
    // sam sej kle nimamo metode new al neki???
    panic!("Ne vem, kaj moram narest??")
}

fn main() {
    let primer1 = AZ::new(1, 2);
    let primer2 = AZ::new(2, 3);
    let vsota = primer1.vsota(&primer2);
    println!("{:?}", vsota);
    println!("{:?}", primer1);
    // let primer3 = AZ::new("Lara", "a");
    // Sam tega s stringi pa zdej ne morm seštevat s tem add-om???? Zakaj pa ne? A je ta Add za + med integerji, sam sej ???
    // Al rabm ubistvu &str za tole. Jes, rabm &str. Aha ne, problem je, ker je add String + &str ... tkoda za stringe
    // bi rabla dva tipa v aritmeticnem: za zacetni clen in pa za razliko. Alpa bi rabla nevtralne elemente implementirat na roke.
    // Pol bi ubistvu loh sam dala Copy trait pa bi blo, da ne bi skos clone-ala kt bučman.
}