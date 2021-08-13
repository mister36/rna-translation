use rna_translate::RNA;
fn main() {
    let rna = RNA::from("UGGUAUUUAUUGUGAUCU");
    println!("{:?}", rna.translate());
}
// Tryptophan, Tyrosine, Leucine, Leucine, STOP