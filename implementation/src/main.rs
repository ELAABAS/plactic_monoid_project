mod tab;
mod monico;
mod erousion;
fn main() {
    // Déclaration de tableaux 2D mutables "s1" et "s"

    let mut s1 = vec![vec![2],vec![1, 3]];
    let mut s = vec![vec![4], vec![3, 4], vec![2, 3, 3],vec![1, 1, 2, 3, 3]];
    // Appelle la fonction "erosion" avec les tableaux "s" et "s1" en entrée
    erousion::erosion(& mut s, & mut s1);
}
