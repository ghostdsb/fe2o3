#[allow(unused)]
pub mod protein_translation {
  use std::collections::HashMap;

  pub struct CodonsInfo<'a> {
    pair: HashMap<&'a str, &'a str>,
  }

  impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
      match self.pair.get(codon) {
        Some(name) => Some(name),
        None => None,
      }
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
      rna
        .chars()
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|chunk| self.name_for(chunk.iter().collect::<String>().as_str()))
        .take_while(|&codon| codon != Some("stop codon"))
        .collect()
    }
  }

  pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
      pair: pairs
        .iter()
        .fold(HashMap::new(), |mut codon_map, (codon, name)| {
          codon_map.insert(codon, name);
          codon_map
        }),
    }
  }
}
