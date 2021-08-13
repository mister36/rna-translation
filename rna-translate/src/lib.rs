use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

pub struct RNA<'a> {
    string: &'a str,
}

impl<'a> RNA<'a> {
    pub fn from(rna_string: &str) -> RNA {
        if rna_string.graphemes(true).count() % 3 != 0 {
            panic!("String length must be a multiple of 3");
        }

        RNA {string: rna_string}
    }

    fn codon_mapper() -> HashMap<Vec<&'a str>, &'a str> {
        let mut mapper: HashMap<_, _> = HashMap::new();
        mapper.insert(vec!["AUG"], "Methionine");
        mapper.insert(vec!["UUU", "UUC"], "Phenylalanine");
        mapper.insert(vec!["UUA", "UUG", "CUU", "CUC", "CUA", "CUG"], "Leucine");
        mapper.insert(vec!["UCU", "UCC", "UCA", "UCG"], "Serine");
        mapper.insert(vec!["UAU", "UAC"], "Tyrosine");
        mapper.insert(vec!["UGU", "UGC"], "Cysteine");
        mapper.insert(vec!["UGG"], "Tryptophan");
        mapper.insert(vec!["UAA", "UAG", "UGA"], "STOP");
        mapper
    }

    pub fn translate(&self) -> Vec<String> {
        let mut codons: Vec<String> = Vec::new(); 
        let mut proteins: Vec<String> = Vec::new();

         // converts "UAAUAGUGA" -> ["UAA", "UAG", "UGA"]
        for i in 0..self.string.graphemes(true).count() / 3 {
            codons.push(self.string[i * 3..i * 3 + 3].to_string());
        }
        let c_mapper = RNA::codon_mapper();

        'outer: for codon in &codons {
            for codon_options in c_mapper.keys()  {
                if codon_options.contains(&codon.as_str()) {
                    if c_mapper.get(codon_options) == Some(&"STOP") {
                        break 'outer;
                    } else {
                        if let Some(protein) = c_mapper.get(codon_options) {
                            proteins.push(protein.to_string());
                        } 
                    }
                    
                } 
            }
        }
        
        proteins
        
    }
}

