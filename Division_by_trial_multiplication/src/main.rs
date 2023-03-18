use rand::Rng;

// la fonction T échange les valeurs des éléments w[1] et w[(i+1)%3]
fn T(i: usize, w: &mut [usize]) -> bool {
    w[1] ^= w[(i + 1) % 3];
    w[(i + 1) % 3] ^= w[1];
    w[1] ^= w[(i + 1) % 3];
    return true;
}

// la fonction knuth vérifie si la condition de Knuth est satisfaite pour i et w, puis appelle T si c'est le cas
fn knuth(i: usize, w: &mut [usize]) -> bool {
    return (w[2] < w[i - 1]) & (w[0] <= w[i]) && T(i, &mut w[0..]);
}

// la fonction robinson effectue une permutation de Robinson sur le tableau w en modifiant w directement
fn robinson(w: &mut [usize], mut j: usize) {
    let i = 0;
    if w[j] < w[j - 1] {
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

// la fonction transform_to_vec_of_vecs transforme un vecteur de nombres en un vecteur de vecteurs où chaque vecteur interne contient une séquence croissante de nombres
fn transform_to_vec_of_vecs(v: Vec<usize>) -> Vec<Vec<usize>> {
    let mut result = vec![];
    let mut i = 0;
    let mut row_len = result.len();
    while i < v.len() - 1 {
        let mut ligne = vec![];
        row_len = row_len + 1;
        while (i + 1) < v.len() && v[i] <= v[i + 1] {
            ligne.push(v[i]);
            i = i + 1;
        }
        ligne.push(v[i]);
        result.push(ligne);
        i += 1;
    }
    result
}

// la fonction multiplication calcule le produit de deux vecteurs et applique la permutation de Robinson pour chaque élément de la nouvelle colonne
fn multiplication(tab1: Vec<usize>, tab2: Vec<usize>) -> Vec<usize> {
    let mut result = tab1.into_boxed_slice().into_vec();
    result.extend_from_slice(&tab2);
    for n in 1..result.len() {
        robinson(&mut result[0..(n + 1)], n);
    }
    return result;
}
// Convertir une liste de nombres en un tableau semi-standard
fn to_semistandard_tableau(numbers:&mut Vec<usize>) -> Vec<Vec<usize>> {
    let mut semistandard_tableau: Vec<Vec<usize>> = Vec::new(); // Initialiser un tableau vide
    let mut j=1; // Initialiser un compteur
    let mut i=0; // Initialiser un compteur
    while i<numbers.len() // Tant que i est plus petit que la taille de la liste numbers
    {
        let mut slice: Vec<usize> = Vec::new(); // Initialiser un sous-tableau vide
        while i<numbers.len()-1 && numbers[i]<=numbers[i+1] // Tant que i est plus petit que la taille de la liste -1 et que numbers[i] est plus petit ou égal à numbers[i+1]
        {
            slice.push(numbers[i]); // Ajouter numbers[i] au sous-tableau slice
            i+=1; // Incrémenter i
        }
        slice.push(numbers[i]); // Ajouter numbers[i] au sous-tableau slice
        semistandard_tableau.push(slice); // Ajouter le sous-tableau slice au tableau semi-standard
        i+=1; // Incrémenter i
    }
    semistandard_tableau // Retourner le tableau semi-standard
}

// Générer un vecteur aléatoire qui ne contient pas les éléments d'un autre vecteur
fn crier_aleatoire(s1:Vec<Vec<usize>>, s:Vec<Vec<usize>>) -> Vec<usize> 
{
    let count = s1.iter().flatten().count(); // Compter le nombre d'éléments dans s1
    let count1 = s.iter().flatten().count(); // Compter le nombre d'éléments dans s
    let nombre_element = count1 - count; // Calculer le nombre d'éléments à ajouter
    let s1_vec: Vec<usize> = s1.concat(); // Concaténer tous les sous-tableaux de s1 en un seul vecteur
    let mut rng = rand::thread_rng(); // Initialiser un générateur de nombres aléatoires
    let max = s.iter().flatten().max().unwrap(); // Trouver le plus grand élément dans s
    let mut v: Vec<usize> = (0..nombre_element).map(|_| rng.gen_range(1..=*max)).collect(); // Générer un vecteur aléatoire qui contient nombre_element éléments compris entre 1 et max inclusivement
    v // Retourner le vecteur aléatoire
}


fn division_by_trial(s1: Vec<Vec<usize>>, s: Vec<Vec<usize>>) -> Vec<usize> {
    let mut b = crier_aleatoire(s1.clone(), s.clone()); // Appel à une fonction externe crier_aleatoire qui renvoie un vecteur b aléatoire.
    let s1_ref = &s1; // créer une référence à s1
    let s_ref = &s;
    let mut tab1: Vec<usize> = vec![]; // initialisation d'un vecteur vide de type usize
    let mut tab2: Vec<usize> = vec![]; // initialisation d'un vecteur vide de type usize
    for sub_vec in s1_ref.iter() { // boucle for pour parcourir les éléments du vecteur s1_ref
        tab1.extend(sub_vec.iter()); // étendre le vecteur tab1 avec chaque élément du vecteur sub_vec
    }
    for sub_vec in s_ref.iter() { // boucle for pour parcourir les éléments du vecteur s_ref
        tab2.extend(sub_vec.iter()); // étendre le vecteur tab2 avec chaque élément du vecteur sub_vec
    }
    
    while compare_vectors(&multiplication(b.clone(), tab1.clone()),& tab2.clone())==false || is_semistandard(to_semistandard_tableau(& mut b))==false
    { // boucle while pour tester si le vecteur b respecte deux conditions : multiplication de b avec tab1 égale à tab2, et b est semi-standard.
        b = crier_aleatoire(s1.clone(), s.clone()); // mettre à jour le vecteur b avec un nouveau vecteur aléatoire généré par crier_aleatoire
        let k=to_semistandard_tableau(& mut b); // créer un nouveau vecteur k qui contient le vecteur b converti en un tableau semi-standard
        let mut blist: Vec<usize> = vec![]; // initialisation d'un vecteur vide de type usize
        let k_ref = &k; // créer une référence au vecteur k
        for sub_vec in k_ref.iter() 
        { // boucle for pour parcourir les éléments du vecteur k_ref
            blist.extend(sub_vec.iter()); // étendre le vecteur blist avec chaque élément du vecteur sub_vec
        }
        b=blist; // mettre à jour le vecteur b avec le vecteur blist
    }
    b // retourne le vecteur b qui satisfait les deux conditions
}

fn compare_vectors(a: &Vec<usize>, b: &Vec<usize>) -> bool {
    if a.len() != b.len() { // comparer la longueur des deux vecteurs
        return false;
    }

    for (x, y) in a.iter().zip(b.iter()) { // boucle for pour parcourir les éléments des deux vecteurs en parallèle
        if x != y { // si un élément est différent dans les deux vecteurs, renvoyer faux
            return false;
        }
    }
    true // si la boucle for est terminée sans trouver de différence, renvoyer vrai
}
fn is_semistandard(tableau: Vec<Vec<usize>>) -> bool 
{
    for i in 0..tableau.len()-1
    {
        if tableau[i].len()>tableau[i+1].len()
        {
            return false;

        }
    }
    for i in 0..tableau.len()-1
    {
        if tableau[i][0]<tableau[i+1][0]
        {
            return false;

        }
    }

    return true;
}
fn main() {
    let s = vec![vec![4], vec![3, 3, 4], vec![1, 2, 3, 3]];
    let s1 = vec![vec![2], vec![1, 3]];
    let tab2 = vec![2,1, 3];
    let tab1 = vec![4,3,3,4,1,2,3,3];
    let s2 = vec![vec![4], vec![3,4],vec![2,3,3],vec![1,1 ,2, 3, 3]];

    println!("{:?}",to_semistandard_tableau(& mut division_by_trial(s1,s2)));
}