pub type Symbol = u8;			// type d'une lettre de l'alphabet
pub const N: Symbol = 26; 		// nombre de symboles dans l'alphabet (alphabet = {1,...,N})
pub type Word = Vec<Symbol>;	// mot de l'alphabet

/// # Tableau
/// Représentation d'un tableau de Young semi-standard
#[derive(Clone, Debug)]
pub struct Tableau {
	tab: Vec<Vec<Symbol>>,
}

impl Tableau {

	/// renvoie un tableau vide
	pub fn new() -> Self {
		return Self {tab: Vec::new()};
	}

	pub fn from_vec(tab: Vec<Vec<Symbol>>) -> Self {
		let mut tbl = Self::new();
		tbl.tab = tab;
		return tbl
	}

	/// créé un tableau à partir d'un mot (algorithme de Robinson-Schensted)
	pub fn from_word(w: Word) -> Self {
		let mut tab = Self::new();
		for s in &w {
			tab.insert(*s);
		}

		return tab;
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
					symb = None; 	// on met symb à None pour signifier qu'on a terminé
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
fn word_to_string(w: &Word) -> String {
	let mut s = String::new();
		for symb in w {
			s.push_str(&*symb.to_string());
		}
	return s;
}