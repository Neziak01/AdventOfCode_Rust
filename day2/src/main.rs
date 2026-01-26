 use std::env;
 use std::fs;
// fn main() {
//    let args:Vec<String> = env::args().collect();
   
//    let file_path = &args[1];  

//    let content = fs::read_to_string(file_path).expect("it should be able to read file");

//    println!("here is the content {content}");

//    let lines: Vec<&str> = content.lines().collect();
//    let mut cpt: i16 = 0;
//    for line in lines {
//       let elements_list: Vec<&str> = line.split(" ").collect(); 
      
//       let mut direction_precedent: Option<bool> = None; // Pour stocker la direction précédente
//       let mut ligne_valide = true; // Pour vérifier si toute la ligne est valide
      
//       for (index, elem) in elements_list.iter().enumerate() {
//          if index < elements_list.len() - 1 {
            
//             let current: i8 = elem.parse().unwrap_or(0);
//             let next: i8 = elements_list[index + 1].parse().unwrap_or(0);
//             let croissant: bool = current < next;
//             let ecart: i8 = current - next; 
//             let ecart_abso: i8 = ecart.abs();
            
//             // Vérifier que l'écart est entre 1 et 3 inclus
//             if ecart_abso < 1 || ecart_abso > 3 {
//                ligne_valide = false;
//                break;
//             }
            
//             if direction_precedent.is_none() {
//                direction_precedent = Some(croissant);
//             } 
//             else if direction_precedent != Some(croissant) {
//                // La direction change, la ligne n'est pas valide
//                ligne_valide = false;
//                break;
//             }
//          }
//       }
      
//       // Si la ligne est valide, incrémenter le compteur
//       if ligne_valide {
//          cpt += 1;
//          println!("Ligne valide: {}", line);
//       }
//    }
//    println!("Nombre de rapports sûrs: {cpt}");

//   //7 6 4 2 1
// }




fn main() {
   let args:Vec<String> = env::args().collect();
   
   let file_path = &args[1];  

   let content = fs::read_to_string(file_path).expect("it should be able to read file");

   println!("here is the content {content}");

   let mut cpt: i16 = 0;
   for line in content.lines() {
      let elements_list: Vec<i8> = line.split_whitespace().filter_map(|n| n.parse().ok()).collect(); 
      
      // Test 1: Vérifier si la ligne est valide telle quelle
      if est_ligne_valide(&elements_list) {
         cpt += 1;
         println!("Ligne valide sans modification: {}", line);
         continue;
      }
      
      // Test 2: Vérifier si retirer un seul élément peut la rendre valide
      let mut peut_etre_sauvee = false;
      for i in 0..elements_list.len() {
         let mut nouvelle_liste = elements_list.clone();
         nouvelle_liste.remove(i);
         
         if est_ligne_valide(&nouvelle_liste) {
            peut_etre_sauvee = true;
            println!("Ligne sauvée en retirant l'élément à l'index {}: {}", i, line);
            break;
         }
      }
      
      if peut_etre_sauvee {
         cpt += 1;
      }
   }  
   
   println!("Nombre de rapports sûrs (avec Problem Dampener): {cpt}");
}

// Fonction pour vérifier si une ligne est valide
fn est_ligne_valide(elements: &[i8]) -> bool {
   if elements.len() < 2 {
      return true; // Une ligne avec moins de 2 éléments est considérée valide
   }
   
   let mut direction_precedent: Option<bool> = None;
   
   for pair in elements.windows(2) {
      let (current, next) = (pair[0], pair[1]);
      let croissant: bool = current < next;
      let ecart: i8 = (current - next).abs();
      
      // Vérifier que l'écart est entre 1 et 3 inclus
      if ecart < 1 || ecart > 3 {
         return false;
      }
      
      match direction_precedent { 
         Some(direction) => {
            if direction != croissant {
               return false;
            }
         }
         None => direction_precedent = Some(croissant),
      }
   }
   
   true
}
