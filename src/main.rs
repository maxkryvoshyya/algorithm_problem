fn main() {
    let mut available = vec![240, 360];
    let mut allowed = vec![240, 360];
    let mut preferred = vec![720, 1080];

    out_of_the_box::ready_to_use(&mut available, &mut allowed, &mut preferred);
    let result = out_of_the_box::attempt(&available, &allowed, &preferred);
    
    println!("available : {available:?}\nallowed : {allowed:?}\npreferred : {preferred:?}\nreturns : {result:?}");
}

