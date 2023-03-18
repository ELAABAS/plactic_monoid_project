extern crate rand;
use rand::Rng;
fn t(i: usize, w: &mut [usize]) -> bool {
    w[1] ^= w[(i + 1) % 3];
    w[(i + 1) % 3] ^= w[1];
    w[1] ^= w[(i + 1) % 3];
    true
}

fn knuth(i: usize, w: &mut [usize]) -> bool {
    w[2] < w[i - 1] && w[0] <= w[i] && t(i, &mut w[0..])
}

fn robinson(w: &mut [usize], mut j: usize) {
    if j > 1 && w[j] < w[j - 1] {
        while j > 1 {
            for i in 1..3 {
                while j >= 2 && knuth(i, &mut w[j - 2..]) {
                    j = j - 1;
                }
            }
            j = j - 1;
        }
    }
}

fn affichage(s: &mut Vec<usize>) {
    // Boucle à travers les lignes du tableau "s"
    let mut result = Vec::new();
    let mut i = 0;
    while i < s.len() {
        let row_len = result.len() + 1;
        if i + row_len <= s.len() {
            let row = s[i..i+row_len].to_vec();
            result.push(row);
        }
        i += row_len;
    }
    println!("{:?}", result);
}
/*
fn main() 
{
    let tab1 = vec![vec![4], vec![3, 3, 4], vec![1, 2, 3, 3]];
    let tab2 = vec![vec![2], vec![1, 3]];
    let mut result = vec![];
    for sub_vec in tab1.iter().chain(tab2.iter()) {
        for item in sub_vec.iter() {
            result.push(*item);
        }
    }

    for n in 1..result.len() {
        robinson(&mut result[0..(n + 1)], n);
    }
    affichage(&mut result);

}
*/
fn main() 
{
    let mut s1 = vec![vec![2],vec![1, 3]];
    let mut s = vec![vec![4], vec![3, 4], vec![2, 3, 3],vec![1, 1, 2, 3, 3]];
    let count = s1.iter().flatten().count();
    let count1 = s.iter().flatten().count();
    let nombre_element=count1-count;
    let mut rng = rand::thread_rng();
    let v: Vec<usize> = (0..8).map(|_| rng.gen_range(1..=10)).collect();    println!("{:?}", v);
}

