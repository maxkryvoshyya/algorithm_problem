fn main() {
    let available = vec![240, 720];
    let allowed = vec![240, 360, 720, 1080];
    let preferred = vec![240, 360];

    let result = out_of_the_box::attempt(&available, &allowed, &preferred);
    
    println!("available : {available:?}\nallowed : {allowed:?}\npreferred : {preferred:?}\nreturns : {result:?}");
}



// fn main() {
//     let mut result = Vec::new();
//     let mut available = vec![240, 360, 720, 1080];
//     let mut preferred = vec![240, 360];

//     for i in preferred.iter() {
//         for j in available.iter() {
//             if i == j {
//                 result.push(*j);
//                 break;
//             } else if j > i {
//                 result.push(*j);
//                 break;
//             } 
//         }      
//     }
//     println!("result : {result:?}");    
// }
