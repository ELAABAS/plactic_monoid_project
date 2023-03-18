use crate::tab::*;

/// renvoie le cardinal de l'intersection entre le support de w et {1,..,s}
/// avec supp(w) = ensemble des symboles distincts composants w
fn f(s: Symbol, w: &Word) -> usize {
	let mut w = (*w).clone();
	w.sort();								// on trie le mot
	w.dedup();								// on élimine les doublons
	let mut i = 0;					// variable pour le parcours de w
	let mut c = 0;					// compteur pour la taille de l'ensemble
	let mut symb: Symbol = 1;				// symbole courant dans {1,...,s}
	while symb <= s {
		if w[i] == symb {
			c += 1;
			i += 1;
			symb += 1;
		}
		if symb < w[i] {
			symb += 1;
		}
		if symb > w[i] {
			i += 1;
		}
	}

	return c;
}

/// fonction D de Monico (3.1)
/// Majorant du nombre de [x] tq d(w,x)=1
fn d(w: &Word) -> usize {
	let mut c: usize = 0;
	let mut fsw: usize;
	for s in 1..=N {
		fsw = f(s, w);
		c += (fsw-1)*std::cmp::min(fsw, w.len());
	}

	return c;
}

/*
fn monico_division(c: &Word, b: &Word, q0: &Word) -> Word {
	let mut q = q0.clone();
	let n = q.len();
	let m = 0;
	
}
*/