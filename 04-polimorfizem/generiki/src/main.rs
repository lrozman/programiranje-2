#[derive(Debug)]

struct AritmeticnoZaporedje<T>{
    zacetni: T,
    razlika: T,
    trenutni_clen: T,
}

// Kaj bo pa kle razlika, i don't get it.

//btw {:?} v println-ju je z debugom, drgač pa loh, če ma stvar implementirat to.string

// In ne, nimajo vse metode smisla. Npr produkt, ena izmed njih.

use AritmeticnoZaporedje as AZ;

impl<T>: AZ<T> { // Ni me blo na predavanjih, ne vem, kako se to nardi, aaaaa
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

        let mut vsota: i32 = 0;
        for i in 1..=n {
            vsota += self.n_th(i-1)
        };
        return vsota
    }

    fn vsota(&self, zap: &AZ) -> AZ { // Sej to je tko, ane? Sam sešteješ začetni pa razliko?
        return AZ::new(self.zacetni + zap.zacetni, self.razlika + zap.razlika);
    }

    fn produkt(&self, zap: &AZ, i:u32) -> i32 {
        return self.n_th(i) * zap.n_th(i)
    }
}


//Pr splošnih se ne ukvarjat s potenco