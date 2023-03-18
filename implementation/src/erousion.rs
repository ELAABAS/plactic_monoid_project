pub fn delete_peaks(tab1 : & mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    // sauv est un entier qui enregistre la valeur de l'élément à supprimer de la table tab1
    let mut sauv = tab1[i][j];
    // suppression de la valeur de l'élément dans la table tab1
    tab1[i].remove(j);
    // si la ligne i est inférieure à la longueur totale de la table moins 1, on insère la valeur sauv dans la ligne suivante
    if i < tab1.len() - 1 {
        tab1[i + 1].push(sauv);
    }
    // boucle à travers les lignes restantes de la table tab1
    for k in i + 1..tab1.len() {
        // trie les colonnes de chaque ligne
        for v in tab1.iter_mut() {
            v.sort();
        }

        let mut j = 0;
        // boucle à travers les colonnes de chaque ligne
        while j < tab1[k].len() {
            // si la valeur sauv est trouvée dans la colonne actuelle
            if sauv == tab1[k][j] {
                // si j est supérieur à 0, on supprime l'élément précédent et on l'assigne à sauv
                if j > 0 {
                    sauv = tab1[k][j - 1];
                    tab1[k].remove(j - 1);
                }
                // sinon, on supprime l'élément suivant et on l'assigne à sauv
                if j == 0 {
                    sauv = tab1[k][j + 1];
                    tab1[k].remove(j + 1);
                }
                // si k est inférieure à la longueur totale de la table moins 1, on insère la valeur sauv dans la ligne suivante
                if k < tab1.len() - 1 {
                    tab1[k + 1].push(sauv);
                }
            }
            j += 1;
        }
    }
    // retourne la valeur sauv
    return sauv;
}

// Fonction pour effectuer l'opération d'érosion
pub fn erosion(tab1 : & mut Vec<Vec<i32>>, tab2 : & mut Vec<Vec<i32>>) {
    // Transformation de la table tab2 en une table unidimensionnelle
    let p: Vec<i32> = tab2.into_iter().flatten().map(|x| *x).collect(); 
    // Boucle à travers les éléments de la table p de la fin à la début
    for i in (0..=p.len()-1).rev() {
        for mut k in 0..tab1.len() {
            // Copie de la table tab1
            let mut copied_table = tab1.to_vec();
            // Vérification si l'élément actuel de la table tab1 est vide
            if tab1[k].len() == 0 {
                k = k + 1;
            } else {
                // Suppression de la valeur maximale dans la table copiée
                let mut deleted_element = delete_peaks(& mut copied_table, k, tab1[k].len()-1);
                println!("p(i)-->{}  deleted_element-->{}  k-->{}  tab1[k].len()-1-->{}", p[i], deleted_element, k, tab1[k].len()-1);
                // Si l'élément supprimé correspond à la valeur p[i], on effectue la suppression dans la table originale
                if deleted_element == p[i] {
                    deleted_element = delete_peaks(tab1, k, tab1[k].len()-1);
                    // Affichage de la table modifiée
                    affichage(tab1);
                    break;
                }
            }
        }
    }
}
// La fonction "affichage" prend en entrée un tableau 2D mutable "s" de nombres entiers "i32"
pub fn affichage(s : & mut Vec<Vec<i32>>) {
    // Boucle à travers les lignes du tableau "s"
    for row in s.iter() {
    // Convertit chaque élément de la ligne en une chaîne de caractères, puis les joint ensemble avec des espaces
    let row_string = row.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    // Affiche la ligne convertie en chaîne de caractères
    println!("{}", row_string);
}
}
    
