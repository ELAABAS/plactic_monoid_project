mod tab;
mod monico;
use std::time::Instant;
use tab::{Word, Tableau};
use std::str;

fn base64_ascii_to_word(w: &mut Word) {
	for s in w {
		match *s {
			43 => *s = 1,
			47..=57 => *s -= 45,
			65..=90 => *s -= 52,
			97..=122 => *s -= 58,
			_ => panic!("received non base64 char"),
		}
	}
}

fn word_to_base_64_ascii(w: &mut Word) {
	for s in  w {
		match *s {
			1 => *s = 43,
			2..=12 => *s += 45,
			13..=38 => *s += 52,
			39..=64 => *s += 58,
			_ => panic!("incorrect value"),
		}
	}
}

fn solve_brown_challenge() -> Word {
	assert_eq!(tab::N, 64);
	let t0 = Instant::now();

	let mut b: Word = b"gnxkOR2uN/j/sWwxNHcMKh1DaMaV4ifNUkJhjr9WWVAHVA5FNRdNYt1/bNGYuq5lZIjIiGtxjdVlT+shNNf5NnYWawpQPJZStxH376j3JQgqwLLy0o5dq4vLeUJrSyoNUGfFB+dQawvYYRTQH+tJZQiAusuD+VTNYkqBoVnl0Vt2CDKGNhNCdiYzf7O6IhgMJVmQxgmAGUPQwOinni6asO+sqodfogSB4B0Dg3UTUl5xnD0ALslyiSm3A7vO+8kOr8976RCTf19I+ZGWfihspGQUdcCwrcCmYRow31AEWMwbKnPLD41maUNHsOBVtJJU58RZcjubTZqnga1f13Bsb/lLn0rXg73vDhCEpbr+yUi6ZOYc+mZW+hB2Cvih6vJ3km3wxaMag86a2i+k1r9d0mcKTITJwjONvr1mDlpISCsmMwTbMcE7ddvbVjGw+TpP9xqQPaOTBMRkW2zP8zcc+8kAr1XClSeb3LKBriZYkHY80P7zaDJh3JJNNgwY/lpf".to_vec();
	base64_ascii_to_word(&mut b);
	let mut d: Word = b"xvupnymwzktxyjsuxipsuxgopstekorrxzcjloqvybhkmntxaeikmqwyYbgilpvxyzVZdhkouvvxzUXbfhnqstvxzTWaeglprsstySVZbfhmoorsxxxzRRVZacjmmnruuwwPPTXZbejkmnttuvOOQVXabdjllmnquvwMNPTUXYchkkllltuvxLMOOSVXZbgijjkqqtvwxKLMNOSWYYbhiiioopuvwJKKMMRUWXabffghjltuuxIJJKLPTUWZacdefiiiksvHHIJKOQRUWXXbbccghjrtvwxGGHHILLPSSUVVWZbegijosuwyFFGGHIKNOQTUUVXaaegiinqtxxxDEEFGGJMMOOSSTWYYbeggmnouuuxCDDDFFGJLLMNRRSTWWZffgiknstvvwyABBCDEFHIKLMNOQRUVYbbdhijooootuz9AABBCEFFHJKLNOQTUVZaadfiklllmpqswx89999BCDDDHHKKLMNSTTTYaacchhhjkoqtv788889BCCCFGIIJKKPRRRUUWZZbbffgjnooqqy67777888ABCFFFGHHMNNNQSVYYYabdfhikloooorx56666777999ABEFGGJJKLMPPQRUUYaddggiimnnoswwwy4455566678889BBDDDGHILLMOOQRSTUVXYdddllnnqrrss3333344455778888BBEGGGIKLNNNNNQUVVVZccfhikmmmrsww22222233344677779AABBDDGJJJJJKLMNTTUUWdffggijknpuw1111112222223445599AAAAABCCCDGHIJPQSTUUWYZbcghilmmpvvw0000000111111223333555788ABBCCDEIJLMNNQQRRabbbccdhhijkmrrsv//////////000000111333356666789ACGJJJKMMOOSTTVZZZaaaaglmpqrrvvwwxz+++++++++++++++++++++++++/11333788BBBCDHJJJJJNNNNNOOOOOPPPTTWXYYYbcefkklpwzz".to_vec();
	base64_ascii_to_word(&mut d);

	let t1 = Instant::now();
	let mut q: Word = monico::plactic_division_lifting(&b, &d);
	let d1 = t1.elapsed();

	let q_t = Tableau::from_word(&q);
	word_to_base_64_ascii(&mut q);
	let b_t = Tableau::from_word(&b);
	let d_t = Tableau::from_word(&d);
	assert_eq!(&q_t*&b_t, d_t);

	let d0 = t0.elapsed();

	println!("temps division : {:?}. Temps total : {:?}", d1, d0);

	return q;
}

fn main() {
	//std::env::set_var("RUST_BACKTRACE", "1");
	
	let q: Word = solve_brown_challenge();
	println!("Solution trouvée : ");
	println!("{}", str::from_utf8(&q).unwrap());

}