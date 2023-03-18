pub use crate::tab::*;
use rand::{Rng, SeedableRng};

/// distance entre 2 tableaux contenant le même nombre d'éléments
fn dist_tab_tab(u: &Tableau, v: &Tableau) -> usize {
	let mut c = 0;
	let mut u = u;
	let mut v = v;
	if v.len() > u.len() {
		let buf = u;
		u = v;
		v = buf;
	}
	for r in 0..v.len() {
		for s in 1..=N {
			c += u.count(r, s).abs_diff(v.count(r, s))
		}
	}
	for r in v.len()..u.len() {
		for s in 1..=N {
			c += u.count(r,s)
		}
	}

	return c/2;
}

/// distance entre un tableau et un mot de meme taille
fn dist_tab_word(u: &Tableau, v: &Word) -> usize {
	let v = Tableau::from_word(v);
	return dist_tab_tab(u,&v);
}

/// distance entre 2 mots **de même taille**
fn dist(u: &Word, v: &Word) -> usize {
	let u = Tableau::from_word(u);
	let v = Tableau::from_word(v);
	return dist_tab_tab(&u, &v);
}

#[allow(dead_code)]
/// distance entre 2 mots quelconques
fn dist_float(u: &Word, v: &Word) -> f64 {
	let u = Tableau::from_word(u);
	let v = Tableau::from_word(v);
	let mut c = 0;
	for r in 0..std::cmp::max(u.len(), v.len()) {
		for s in 1..=N {
			c += u.count(r, s).abs_diff(v.count(r, s));
		}
	}

	let c = c as f64;
	return c/2.;
}

/// renvoie le cardinal de l'intersection entre le support de w et {1,..,s}
/// avec supp(w) = ensemble des symboles distincts composants w
fn f(s: Symbol, w: &Word) -> i32 {
	let mut w = (*w).clone();
	w.sort();								// on trie le mot
	w.dedup();								// on élimine les doublons
	let i_max = w.len();
	let mut i = 0;					// variable pour le parcours de w
	let mut c = 0;						// compteur pour la taille de l'ensemble
	let mut symb: Symbol = 1;				// symbole courant dans {1,...,s}
	while symb <= s && i < i_max {
		if w[i] == symb {
			c += 1;
			i += 1;
			symb += 1;
			continue;
		}
		if symb < w[i] {
			symb += 1;
			continue;
		}
		if symb > w[i] {
			i += 1;
		}
	}

	return c;
}

#[allow(non_snake_case)]
/// fonction D de Monico (3.1). 
/// Majorant du nombre de \[x\] tq d(w,x)=1
fn D(w: &Word) -> usize {
	let mut c: i32 = 0;
	let mut fsw: i32;
	for s in 1..=N {
		fsw = f(s, w);
		c += (fsw-1)*std::cmp::min(fsw, w.iter().filter(|&n| *n == s).count() as i32);
	}

	return c as usize;
}

#[allow(dead_code)]
/// trouve un mot q0 tel qu'il existe une permutation sigma
/// telle que \[sigma(q0)\]\[b\] = \[c\]
pub fn find_q0(b: &Word, c: &Word) -> Word {
	let mut q0: Word = Vec::with_capacity(c.len()-b.len());
	let mut bs;
	let mut cs;
	for s in 1..=N {
		bs = b.iter().filter(|&n| *n == s).count();
		cs = c.iter().filter(|&n| *n == s).count();
		for _i in 0..cs-bs {
			q0.push(s);
		}
	}

	return q0;
}

/// applique en place une permutation de l'ensemble des perumations définit 
/// par Monico au mot w de taille n
fn permute<R>(w: &mut [Symbol], n: usize, rng: &mut R) where R: Rng + ?Sized {
	let i = rng.gen_range(0..n);
	let j = rng.gen_range(0..n);
	
	if i == j {
		// i et j doivent être distincts
		permute(w, n, rng);
	}
	else {
		let symb_i: Symbol = w[i];

		if i < j {
			for s in i..j {
				w[s] = w[s+1];
			}
		}
		else {
			for s in (j+1..=i).rev() {
				w[s] = w[s-1];
			}
		}

		w[j] = symb_i;
	}
}

/// Algorithme 1 de Monico. 
/// renvoie un mot q tel que \[q\]\[b\]=\[c\]
pub fn plactic_division(b: &Word, c: &Word, q0: Word) -> Word {
	// initialisation du prng
	let mut rng = rand_pcg::Pcg64Mcg::from_entropy();

	// inititalisations
	let n = q0.len();
	let mut m = 0;

	let mut q_g = q0.clone();

	let mut qb: Word = q0;
	qb.extend( b);

	#[allow(non_snake_case)]
	let M = 2 * D(&qb); 

	let mut sqb: Word = qb.clone();
	let mut delta;

	let mut dist_qb_c = dist(&qb,c);
	let mut delta_g = dist_qb_c;

	let mut x: f64;

	let c_tab = Tableau::from_word(c);

	// boucle
	while dist_qb_c > 0 {
		sqb[..n].clone_from_slice(&qb[..n]);
		permute(&mut sqb[..n], n, &mut rng);
		let sqb_tab = Tableau::from_word(&sqb);
		delta = dist_tab_tab(&sqb_tab, &c_tab);

		if dist_tab_word(&sqb_tab, &qb) == 1 {
			m += 1;
		}

		if delta < dist_qb_c {
			m = 0;
		}

		if delta <= dist_qb_c {
			qb[..n].clone_from_slice(&sqb[..n]);  		// q = sq
			dist_qb_c = delta;
		}

		if delta <= delta_g {
			delta_g = delta;
			q_g[..].clone_from_slice(&qb[..n]); 		// q_g = q
		}

		if delta > dist_qb_c && m > M {
			m = 0;
			qb[..n].clone_from_slice(&q_g[..]); 		// q = q_g
			
			permute(&mut qb[..n], n, &mut rng);

			loop {
				permute(&mut qb[..n], n, &mut rng);
				x = rng.gen();
				if x >= 1./2. {
					break;
				}
			}
			dist_qb_c = dist_tab_word(&c_tab, &qb);
		}
	}
	
	return qb[..n].to_vec();
}

/// Algorithme 2 de Monico. 
/// renvoie un mot q tel que \[q\]\[b\]=\[c\]
pub fn plactic_division_lifting(b: &Word, c: &Word) -> Word {
	let mut k = 1;
	let mut b_k = b.iter().filter(|&n| *n == 1).count();
	let mut c_k = c.iter().filter(|&n| *n == 1).count();
	let mut q: Word = vec![1; c_k-b_k];
	let mut q0: Word;
	let mut pi_k_b: Word;
	let mut pi_k_c: Word;

	while k < N {
		k += 1;

		pi_k_b = b.clone();
		pi_k_c = c.clone();

		pi_k_b.retain(|&s| s <= k);
		pi_k_c.retain(|&s| s <= k);

		b_k = pi_k_b.iter().filter(|&s| *s == k).count();
		c_k = pi_k_c.iter().filter(|&s| *s == k).count();
		q0 = vec![k; c_k-b_k];
		q0.append(&mut q);
		q = plactic_division(&pi_k_b, &pi_k_c, q0);
	}

	return q;
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn distance() {
		let mut rng = rand_pcg::Pcg64Mcg::from_entropy();


		let u: Word = vec![4,3,3,4,1,2,3,3];
		let v: Word = vec![2,1,3];

		assert_eq!(dist_float(&u, &v), 3.5);

		let mut s1: usize;
		let mut s2: usize;
		for _i in 0..10 {
			s1 = rng.gen_range(20..=100);
			s2 = rng.gen_range(20..=100);
			let u: Word = create_random_word(&mut rng, s1);
			let v: Word = create_random_word(&mut rng, s2);
			let u2: Word = u.clone();

			assert!(dist_float(&u, &v) >= 0f64);
			assert_eq!(dist_float(&u,&u2), 0f64);
			assert_eq!(dist_float(&u, &v), dist_float(&v, &u));
		}

		for _i in 0..10 {
			s1 = rng.gen_range(30..300);
			let u: Word = create_random_word(&mut rng, s1);
			let v: Word = create_random_word(&mut rng, s1);
			let u2: Word = u.clone();

			let u_tab = Tableau::from_word(&u);
			let v_tab = Tableau::from_word(&v);
			assert_eq!(dist_tab_tab(&u_tab, &v_tab), dist_tab_word(&v_tab, &u));
			assert_eq!(dist_tab_word(&u_tab, &v), dist(&u, &v));

			assert!(dist(&u, &v) >= 0 as usize);
			assert_eq!(dist(&u,&u2), 0);
			assert_eq!(dist(&u, &v), dist(&v, &u));
		}


		for _i in 0..10 {
			s1 = rng.gen_range(30..300);
			let u: Word = create_random_word(&mut rng, s1);
			let v: Word = create_random_word(&mut rng, s1);

			assert_eq!(dist(&u, &v) as f64, dist_float(&u, &v));
		}

	}

	#[test]
	fn division() {
		let mut rng = rand_pcg::Pcg64Mcg::from_entropy();

		let mut s_a: usize;
		let mut s_b: usize;
		for _i in 0..3 {
			s_a = rng.gen_range(3..15);
			s_b = rng.gen_range(3..15);
			let a: Word = create_random_word(&mut rng, s_a);
			let b: Word = create_random_word(&mut rng, s_b);

			let a_t = Tableau::from_word(&a);
			let b_t = Tableau::from_word(&b);
			let c_t = &a_t*&b_t;

			let c: Word = c_t.row_reading();

			let q0: Word = find_q0(&b, &c);

			let q: Word = plactic_division(&b, &c, q0);

			let q_t = Tableau::from_word(&q);

			assert_eq!(&q_t*&b_t, c_t);
		}
	}

	#[test]
	fn division_lifting() {
		let mut rng = rand_pcg::Pcg64Mcg::from_entropy();

		let mut s_a: usize;
		let mut s_b: usize;
		for _i in 0..3 {
			s_a = rng.gen_range(3..15);
			s_b = rng.gen_range(3..15);
			let a: Word = create_random_word(&mut rng, s_a);
			let b: Word = create_random_word(&mut rng, s_b);

			let a_t = Tableau::from_word(&a);
			let b_t = Tableau::from_word(&b);
			let c_t = &a_t*&b_t;

			let c: Word = c_t.row_reading();

			let q: Word = plactic_division_lifting(&b, &c);

			let q_t = Tableau::from_word(&q);

			assert_eq!(&q_t*&b_t, c_t);
		}
	}
}