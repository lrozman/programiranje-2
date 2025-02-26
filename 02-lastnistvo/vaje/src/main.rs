use std::time::{Duration, Instant};

fn time_it<F: FnOnce() -> R, R>(f: F) -> Duration {
    let start = Instant::now();
    f();
    start.elapsed()
}

fn on_stack() {
    // Narišite shemo spreminjanja sklada in kopice
    // Za vsako vrstico napiši, kolikokrat se v pomnilniku pojavi 13?
    let mut a = [13; 100]; // to je array, k ma sto trinajstk notr. To gre na stack, ker je array, k je fiksne dolžine, se ne bo nč spreminjal po velikosti.
    let mut b = a; // Kdaj stvari lahko prevzemajo lastništvo? Kadar živijo na kopici "lastim si ta prostor na kopici"
    // Če bi ta ta prvi seznam živel na kopici, bi blo tko. Ampak ta seznam ne živi na kopici. Mamo še dve možnosti:
    // Loh kaže na isto kokr a, loh se pa kopira.
    // Kle se kopira. Ne more kazat na isto, kokr a, ker je to kontra s filozofijo od Rusta - sam en loh na neki kaže (že tko pa ne more bit pointerja tja, ker ni na kopici)
    let q = String::from("13"); //Loh bi rekl tud, Box::new(13), ker pač žvi na kopici. Zdej mamo 201 trinajstk na pomnilniku
    println!("{}", q); //izpiše q, ampak kaj se tuki zgodi, a se nardi kopija, a zgubi lastništvo al ... 
    // On tuki ve, da ti hočš referenco dobit ven iz tega, tkoda ti sam pogleda v škatlo notr, q ma še vedno lastništvo
    // Drgač bi mogl pisat *q (ker je na tabli q Box), da pogleda v škatlo, ampak rust to sam ve, ker je pametn. 
    // Tuki torej q ne zgubi lastništva, čeprou smo a uporabl v funkciji.
    let r = q; // Nobene nove trinajstke, q zgubi lastništvo, r ga zdej ma, 201 trinajstk
    let p = &r; // Ta stvar nardi referenco. Samo "sposodš" si vrednost r-ja. Nekak kokr v pythonu.
    // Referenco pa spet odpakiraš z *p, loh pa **p, da še iz boxa odpakiraš ven, če je bla referenca na box.
    // Nobene nove trinajstke, p sam kaže na r, r je še vedno lastnik te škatle, p si jo je sposodu
    a[0] = 1; // Ena trinajstka je zginla, mamo jih še 200. b se ni spremenu, ker se je b itak kopiru.
    {
        let c = &b;
        println!("{}", c[0]); // klele 13 dobimo, ker je b isti.
    } // Pa se mi  zdi, da c-ja zuni tega ni ??
    println!("{}", b[0]); // 13 printamo
    println!("{}", a[0]); // 1 printamo
    println!("{}", p); // print odpakira box, tkoda izpiše 13
    println!("{}", r); // isto 13
    // println!("{}", q); // Razloži, zakaj to ne deluje
    // Ker q nima več lastništva, ni ga več.
    // ene fore z mutable:
    // Če maš let mut p = 13, in pol let mut q = &mut p, pol loh neki spreminjša. Pač ne morš spreminjat, če si si sposodu neki nespremenljivga, se mi zdi?
    // Neki neki, rust v nekih primerih ne pusti, da si sposojeno vrednost še enrkat sposodš, takrat loh to rešš, čedaš to v svoj scope {}, zuni ne živi več in si gut.
    // seprav, let mut p = neki, in po let q = &mut p, let r = &mut p, ne morš, ler si si dvakrat mutable sposodu in ne pusti.
}

/// Napišite funkcijo `swap`, ki zamenja vrednosti dveh celoštevilskih spremenljivk.
//fn swap(x: i32, y: i32) -> (i32, i32) {
//    return (y, x); // Tud brez returna dela, ker je zadn stavek, vrne.
//}

fn swap(x: &mut i32, y: &mut i32) {
    let c = *x;
    *x = *y; // Na levi strani enačaja pač gre tja neki neki, na desni gre pa pač tja pogledat al neki?
    *y = c;
}

fn test_swap() {
    // V spremenljivko `a` shranite vrednost 13, v spremenljivko `b` pa vrednost 42.
    let mut a = 13;
    let mut b = 42;
    println!("a: {}, b: {}", a, b);
    // Izpiše `a: 13, b: 42`.

    // Naredite swap s pomočjo pomožne funkcije `swap`.
    //let (a, b) = swap(a, b);
    // Ta stvar ni spremenila sploh a-jev pa b-jev, sam prevzel so lastništvo, oziroma še to ne, ker so številke, k so na stacku, je sam skopiru
    // Kle smo na novo definiral a pa b, tkoda tle nismo nardil tega, kar smo hotl.
    // Ta prav swap ničesar ne bo vrnu
    swap(&mut a, &mut b); // Sam sposodl si jih bomo

    println!("a: {}, b: {}", a, b);
    // Izpiše `a: 42, b: 13`.
}

/// Popravite zakomentiran del spodnje funkcije, da bo deloval
fn str_own() {
    let x = String::from("Hello world"); // Na kopici se ustvari ta string
    let y = &x; // loh bi dal x.clone()
    println!("{}, {}", x, y);
}

/// Popravite brez uporabe funkcije `clone`
/// Namig: sklad in kopiranje na skladu - kodo lahko spremenite
fn str_own2() {
    //let x = (1, 2, (), String::from("Hello world"));  // Tko je blo najprej
    // če bi dal mut x, bi loh nardil x.3.push_str("lalal") in spremeni ta string, čeprou je v tuplu.
    // Problem je v tem stringu, ker ma ta tuple lastništvo nad njim.
    // Lahko nardimo tkole:
    let x = (1, 2, (), "Hello world"); // To pa zdej ni spremenljivo, kt en array characterjev,
    // to živi na stacku, in če nardimo y = neki, kar je na stacku, bo naredu kopijo
    let y = x; // loh bi dal referenco, sam mi bi radi brez clonea prsill y, da bo kopija x-a
    println!("{:?}, {:?}", x, y); // baje ta :? neki če obstaja ta vrednost neki neki ??
}

/// Popravite spodnji dve funkciji, da bosta delovali

fn wrong() {
    let s = String::from("Hello World");
    print_str(&s); // loh damo notr s.clone()
    println!("{}", s);
}

fn print_str(s: &String) { // Pol smo še kle dodal &String, in to printa sam sposojene vrednosti
    println!("{}", s)
}

/// ------------------------------------------------------------------------------------------------
/// Popravite spodnjo funkcijo, da bo delovala
fn fn1() {
    let s = String::from("Hello ");
    let mut s1 = s; // Dal smo, da mora bit mutable s1, za s nam je vseen itak
    s1.push_str("World!");
    println!("Success!");
}

/// ------------------------------------------------------------------------------------------------
/// Popravite spodnjo funkcijo, da bo delovala

fn fn2() {
    //let mut x = Box::new(5);
//
    //// // Popravite zgolj tukaj vmes
    //let mut y = &mut x;
    //// //
    //**y = 4; // To sem js ugotavljala, kaj se zgodi s temi referencami
    let x = Box::new(5);
    // Popravite zgolj tukaj vmes
    let mut y = Box::new(42);
    //
    *y = 4;

    assert_eq!(*x, 5); // Če ne bo enako, se bo sesul

    println!("Success!");
}

/// ------------------------------------------------------------------------------------------------

fn fn3() {
    let t = (
        String::from("hello"),
        String::from("world"),
        String::from("!"),
    );

    let _s = t.1; // To je prestavljena vrednost. Podčrtaj je pa, da rust ne joka, da nisi uporabu

    // Izpišite čim večji del t-ja.
    println!("{}{}", t.0, t.2);
    // Celga t-ja sploh ne morš izpisat, ker je delno prestavljen.
}

/// ------------------------------------------------------------------------------------------------

fn fn4() {
    let x = 5;
    // Izpišite naslov spremenljivke x
    println!("{:p}", &x);
}

/// ------------------------------------------------------------------------------------------------

fn fn5() {
    let x = 13;
    let y = &x;

    // Popravite spodnjo vrstico, da bo bo enakost držala
    // assert_eq!(13, y);
    assert_eq!(13, *y);
}

/// ------------------------------------------------------------------------------------------------

/// Popravite spodnjo funkcijo, funkcija `helper` se mora poklicati čim bolj učinkovito.
fn fn6() {
    let mut s = String::from("hello, ");

    helper(&s); //referenco smo mogl dat notr

    println!("Success!");
}

// Te funkcije ne spreminjajte
fn helper(s: &String) {}

/// ------------------------------------------------------------------------------------------------

/// Popravite spodnjo funkcijo, funkcija `helper2` se mora poklicati čim bolj učinkovito.
fn fn7() {
    let mut s = String::from("hello, ");

    helper2(&mut s); // hoče mutable referenco na string

    println!("Success!");
}
// Te funkcije ne spreminjajte
fn helper2(s: &mut String) {
    s.push_str("world")
}

/// ------------------------------------------------------------------------------------------------

/// Pojasnite, zakaj spodnja koda ne deluje
fn fn8() {
    // let mut s = String::from("hello, ");

    // let p = &mut s; // p je tipa mutable, tkoda do tega p.push_str je gut, je tipa &mut. Al kaj ??

    // p.push_str("world");  // To dela, ker push_str vzame &mut self, sprejme self, kot mutable referenco
    // Alpa je pametn, pa zna velirkat zvezdice sam postavljat.
    // Pa btw, spremenil se je tud s, ker p sam kaže tja.
    // Če damo te dve zgornji vrstici v {}, pa spodi pač s namest p-ja, bo delal.

    // println!("Success! {}", p);
    // println!("Success! {}", s); // Tuki je problem, ker si ne morš sposodt s kot immutable, ker smo si ga že prej kot mut sposodl
    // p.push_str("!");
}

/// ------------------------------------------------------------------------------------------------
/// Pojasnite, zakaj spodnja koda ne deluje in jo popravite
/// Pojasnite tudi, zakaj je popravek ok

fn fn9() {
    let mut s = String::from("hello");

    let r1 = &s; //&mut s;
    let r2 = &s; // &mut s; // dvakrat smo si mutable sposodl, no go-
    // Pa obakrat si ga mormo ne-mutable, ker si ga por printu še enkrat sposodl, tkoda si ga prej ne mormo mutable
    // Če pa hočmo zares met, da je to mutable, nardimo scope za prvega, pa notr print!("{r1}") in pol spodi posebi r2.

    println!("{}, {}", r1, r2);

    println!("Success!");
}

/// ------------------------------------------------------------------------------------------------
fn fn10() {
    // // Popravite spodnjo vrstico
    // let s = String::from("hello, ");
    let mut s = String::from("hello, ");

    helper3(&mut s);

    println!("Success!");
}

fn helper3(s: &mut String) {}

/// ------------------------------------------------------------------------------------------------

fn main() {
    fn10();
}
