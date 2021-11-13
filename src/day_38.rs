pub mod rna_transcription {

  #[derive(Debug, PartialEq)]
  pub struct Dna(String);

  #[derive(Debug, PartialEq)]
  pub struct Rna(String);

  impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
      match dna
        .chars()
        .enumerate()
        .take_while(|(i, n)| valid_nucleotides("dna", *i, *n).is_ok())
        .count()
      {
        len if len == dna.len() => Ok(Dna(String::from(dna))),
        len => Err(len),
      }
    }

    pub fn into_rna(self) -> Rna {
      Rna(self.0.chars().map(|n| get_rna_complement(n)).collect())
    }
  }

  impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
      match rna
        .chars()
        .enumerate()
        .take_while(|(i, n)| valid_nucleotides("rna", *i, *n).is_ok())
        .count()
      {
        len if len == rna.len() => Ok(Rna(String::from(rna))),
        len => Err(len),
      }
    }
  }

  fn valid_nucleotides(strand: &str, index: usize, n: char) -> Result<usize, usize> {
    match (strand, n) {
      ("dna", 'A' | 'T' | 'G' | 'C') => Ok(index),
      ("rna", 'A' | 'U' | 'G' | 'C') => Ok(index),
      _ => Err(index),
    }
  }

  fn get_rna_complement(n: char) -> char {
    match n {
      'A' => 'U',
      'T' => 'A',
      'G' => 'C',
      'C' => 'G',
      _ => 'X',
    }
  }
}
