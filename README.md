# Hash

Darbo tikslas buvo sukurti hash funkciją be išorinės pagalbos.

## Reikalavimai paleisti

Norint naudoti funkciją reikia turėti suinstaliuota [rust](https://www.rust-lang.org/tools/install) ir sukompiliuoti failą naudojant komanda:

```rb
$ cargo run
```
## Naudojimas

Paleidus programą jūsų paklaus ar norite palyginti hash funkcijos greitį su sha256 hash metodu gautu iš rust sha256 [crate](https://crates.io/crates/sha256). Programa gali sukurti šabloninius failus, kuriuos naudos imti informaciją imti į hash funkciją. Vartotojas gali pasirinkti ar nori kurti failus ir kokius nori hashuoti.

Detalesniam tikrinimui programa sukuria 10000 skirtingų string porų ir jų hashintas poras išsaugo .txt faile, kuriuos skiriasi pirmu char. Šis failas yra naudojamas paskaičiuoti hashų vidutinį bitų skirtumą ir kiek pasikartojančių hashų funkcija sukuria.

Kuriant naujus failus 5 penkis kartus buvo gauti tokie rezultatai:

| | #1 failas | #2 failas | #3 failas | #4 failas | #5 failas |vid |
| ------------- | ------------- | ------------- | ------------- | ------------- | ------------- | -------------|
| Bitų skirtumas| 397 | 398 | 397 | 397 | 398 | 397,4 |
| Kolizijų kiekis| 64 | 62 | 72 | 52 | 77 | 65,4 |

Hash funckija tikrai nėra gera, nes, atrodo, kad funkcija yra lėta ir 1 bito skirtumas pakeičia galutini rezultatą tiktai apie 397 bitus.
