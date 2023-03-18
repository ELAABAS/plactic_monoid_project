pub type Symbol = u8;			// type d'une lettre de l'alphabet
pub const N: Symbol = 64; 		// nombre de symboles dans l'alphabet (alphabet = {1,...,N})
pub type Word = Vec<Symbol>;	// mot de l'alphabet
use rand::prelude::*;

/// # Tableau
/// Représentation d'un tableau de Young semi-standard
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tableau {
	// les lignes sont stockées de bas en haut
	tab: Vec<Vec<Symbol>>,
}

impl Tableau {

	/// renvoie un tableau vide
	pub fn new() -> Self {
		return Self {tab: Vec::new()};
	}

	/// constructeur avec capacitée minimale pour le vec
	pub fn with_capacity(capacity: usize) -> Self {
		return Self {tab: Vec::with_capacity(capacity)}
	}

	/// construit un tableau à partir d'un vec
	pub fn from_vec(tab: Vec<Vec<Symbol>>) -> Self {
		let mut tbl = Self::new();
		tbl.tab = tab;
		return tbl
	}

	/// créé un tableau à partir d'un mot (algorithme de Robinson-Schensted)
	pub fn from_word(w: &Word) -> Self {
		let mut tab = Self::with_capacity(w.len());
		for s in w {
			tab.insert(*s);
		}
		return tab;
	}

	/// nombre de lignes d'un tableau
	pub fn len(&self) -> usize {
		return self.tab.len();
	}

	/// renvoie le nombre d'occurences de s dans la ligne r du tableau
	pub fn count(&self, r: usize, s: Symbol) -> usize {
		let mut c = 0;
		if let Some(line) = self.tab.get(r) {
			c = line.iter().filter(|&n| *n == s).count();
		}

		return c;
	}

	/// insère un élèment dans un tableau
	fn insert(&mut self, symb: Symbol) {
		let mut symb = Some(symb);

		if self.tab.is_empty() {
			self.tab.push(vec![symb.unwrap()])
		} else {
			let mut s : Symbol; 	// buffer
			let mut i : usize;

			for row in &mut self.tab {

				// une ligne n'est jamais vide car on la créé toujours 
				// avec un élément
				let l = row.last().unwrap();

				if *l <= symb.unwrap() {
					// cas où on peut insérer en fin de ligne : 
					// on insère et on arrête
					row.push(symb.unwrap());
					// on met symb à None pour signifier qu'on a terminé
					symb = None;

					break;

				} else {
					// sinon on insère le symbole de droite 
					// dans la ligne du dessus
					i = Self::search(row, symb.unwrap());
					s = row[i];
					row[i] = symb.unwrap();
					symb = Some(s);
				}
			}

			if symb.is_some() {
				// si on a pas terminé l'insertion 
				// on doit ajouter une ligne au tableau
				self.tab.push(vec![symb.unwrap()])
			}
		}
	}

	/// Effectue une recherche dichotomique dans une liste
	/// -entrée : 
	/// 	-row 	: une ligne de tableau
	/// 	-symb 	: un symbole de l'alphabet
	/// -sortie
	/// 	-l'indice du plus petit symbole strictement supérieur à celui recherché
	fn search(row: &Vec<Symbol>, symb: Symbol) -> usize {
		let mut a = 0;
		let mut b = row.len();
		let mut c;

		if symb < row[0] {
			// comme on renvoie b, on a un problème si le symbole
			// doit être inséré en 1ere position
			return 0;
		}

		while b-a > 1 {
			c = (a+b)/2;

			if symb < row[c] {
				b = c;
			} else {
				a = c;
			}
		}

		return b;
	}

	/// renvoie le mot correspondant à un tableau
	pub fn row_reading(&self) -> Word {
		let mut rr: Word = Vec::new();
		for row in self.tab.iter().rev() {
			for symb in row.iter() {
				rr.push(*symb);
			}
		}
		return  rr;
	}

}

impl std::ops::Mul for &Tableau {
	type Output = Tableau;

	/// multiplication de Knuth de 2 tableaux
	fn mul(self, rhs: Self) -> Self::Output {
		let mut mult = self.clone();
		let rr_rhs = rhs.row_reading();
		for s in &rr_rhs {
			mult.insert(*s);
		}

		return mult;
	}
}

use std::fmt;

impl fmt::Display for Tableau {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut s = String::new();
		for row in self.tab.iter().rev() {
			for symb in row {
				s.push_str(&*symb.to_string());
				s.push_str(", ");
			}
			s.pop();
			s.pop();
			s.push('\n');
		}
		write!(f, "{}", s)
	}
}

/// concatène les symboles d'un mot dans une String
#[allow(dead_code)]
pub fn word_to_string(w: &Word) -> String {
	let mut s = String::new();
		for symb in w {
			s.push_str(&*symb.to_string());
		}
	return s;
}

#[allow(dead_code)]
pub fn create_random_word<R>(rng: &mut R, size: usize) -> Word where R: Rng + ?Sized {
	let mut w: Word = Vec::with_capacity(size);
	let mut s: Symbol;
	for _i  in 0..size {
		s = rng.gen_range(1..=N);
		w.push(s);
	}

	return w;
}

#[cfg(test)]
mod test {
	use rand_pcg::Pcg64Mcg;
	use super::*;

	#[test]
	fn eq() {
		let tab1 = Tableau::from_vec(vec![vec![1,3], vec![2]]);
		let tab2 = Tableau::from_vec(vec![vec![1,3], vec![2]]);
		assert_eq!(tab1, tab2);
	}

	#[test]
	fn creation() {
		let tab = Tableau::from_vec(vec![vec![1,2,3,3], vec![3,3,4], vec![4]]);
		let w: Word = vec![4,3,3,4,1,2,3,3];
		assert_eq!(tab, Tableau::from_word(&w));
	}

	#[test]
	fn rr() {
		let tab = Tableau::from_vec(vec![vec![1,2,3,3], vec![3,3,4], vec![4]]);
		let w: Word = vec![4,3,3,4,1,2,3,3];
		assert_eq!(tab.row_reading(), w);
	}

	#[test]
	fn robinson() {
		let mut rng = Pcg64Mcg::from_entropy();
		let mut w: Word;
		let mut s: usize;
		let mut t: Tableau;
		for _i in 0..10 {
			s = rng.gen_range(10..200);
			w = create_random_word::<Pcg64Mcg>(&mut rng, s);
			t = Tableau::from_word(&w);
			w = t.row_reading();
			assert_eq!(t, Tableau::from_word(&w));
		}
	}

	#[test]
	fn multiplication() {
		let mut rng = Pcg64Mcg::from_entropy();
		let sa = rng.gen_range(30..300);
		let sb = rng.gen_range(30..300);
		let mut a_w = create_random_word(&mut rng, sa);
		let b_w = create_random_word(&mut rng, sb);

		let a = Tableau::from_word(&a_w);
		let b = Tableau::from_word(&b_w);

		a_w.extend(b_w);
		assert_eq!(&a*&b, Tableau::from_word(&a_w));
	}

	#[test]
	fn multiplication2() {
		let a: Word = vec![3,9,1];
		let b: Word = vec![8,3,9,2,5,7];
		let a_t = Tableau::from_word(&a);
		let b_t = Tableau::from_word(&b);

		let c = Tableau::from_vec(vec![vec![1,2,5,7], vec![3,3,9], vec![8], vec![9]]);

		assert_eq!(&a_t*&b_t, c);
	}

	#[test]
	fn multiplication3() {
		let tab1 = Tableau::from_vec(vec![vec![1,3], vec![2]]);
		let tab2 = Tableau::from_vec(Vec::new());

		assert_eq!(&tab1*&tab2, tab1);
	}

	#[test]
	fn count() {
		let tab = Tableau::from_vec(vec![vec![1,2,3,3], vec![3,3,4], vec![4]]);
		assert_eq!(tab.count(1, 3), 2);
		assert_eq!(tab.count(2, 2), 0);
		assert_eq!(tab.count(4,1), 0);
	}
}