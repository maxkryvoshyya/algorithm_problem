/// Функція **attempt** приймає посилання на вектор та повертає вектор значень, які задовільняють умови запиту.
/// * `available` - вектор доступних значень.
/// * `allowed` - вектор дозволених значень.
/// * `preferred` - вектор бажаних значень.
/// 
/// - Алгоритм обирає із *доступних значень* так, щоб вони *задовільняли yci перерахованні умови* та повертає іх у вигляді вектора.
/// - Якщо алгоритм не може повернути *бажане значення*, тоді він намагається повернути *найближче значення, яке є не менше бажаного*, якщо і це не можливо тоді *найближче менше*.
/// - Алгоритм повертає лише ті значення, які є *дозволеними і доступними*.
/// - Алгоритм повертає *порожній вектор, якщо жодне із значень не підходить*. 
/// - Результат виконання алгоритму *не має повторів*.
/// - Всі вхідні дані *відсортовані у порядку зростання*.


///  # Приклад
///  
///  ```
///  let mut available = vec![240, 720];
///  let mut allowed = vec![240, 360, 720, 1080];
///  let mut preferred = vec![240, 360];
/// 
///  let result = out_of_the_box::attempt(&available, &allowed, &preferred);
/// 
///  assert_eq!(vec![240, 720], result);
///  ```
/// 

// Функція attempt приймає аргументами посилання на вектори значень, 
// та повертає вектор із відсортованими значеннями

pub fn attempt(available: &Vec<i32>, allowed: &Vec<i32>, preferred: &Vec<i32>) -> Vec<i32> {
    
    let mut selected = Vec::new(); // Змінна selected зберігає спільні значення available 
                                             // та allowed з якими будемо працювати далі
    for i in available {               
        for j in allowed {
            if i == j {
                selected.push(*i) //Якщо співпадає allowed з available значення записується в selected
            }
        }
    }

    let mut result = Vec::new(); // Змінна result зберігає спільні значення selected 
                                           // та preferred, а у випадку невдачі буде зберігатися більше 
                                           // значчення за бажене або нічого якщо таких значень немає 

    for i in preferred{
        for j in selected.iter() {
            if i == j {
                result.push(*j); // Якщо співпадає preferred з selected значення записується в result
                break;
            } 
        } break;      
    }
    if result.len() != preferred.len() { // Перевіряє чи було додане значення і якщо ні обирає більше
        for i in preferred {
            for j in selected.iter() {
                if i < j {
                    result.push(*j); // Обирається більше значення
                    break;
                } 
            } break;      
        }
    }
    if result.len() != preferred.len() {
        for i in preferred {
            for j in selected.iter().rev() { // Сортуємо вектор щоб отримати найперше менше значення
                if i > j {
                    result.push(*j); // Обирається менше значення
                    break;
                } 
            } break;      
        }
    }
    result
}

/// Функція **ready_to_use** приймає змінне посилання на вектор. Сортує значення від найменшого до найбільшого та видаляє дублікати.
///  # Приклад
///  
///  ```
///  let mut available = vec![360, 240, 360, 720];
///  let mut allowed = vec![1080, 720, 240, 720];
///  let mut preferred = vec![240, 480];
/// 
///  ready_to_use(&mut available, &mut allowed, &mut preferred);
/// 
///  assert_eq!(vec![240, 360, 720], available);
///  ```
/// 

pub fn ready_to_use (available: &mut Vec<i32>, allowed: &mut Vec<i32>, preferred: &mut Vec<i32>) {
    available.sort();
    available.dedup();
    allowed.sort();
    allowed.dedup();
    preferred.sort();
    preferred.dedup();
} 



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn get_vec_of_value() {
        let available = vec![240, 720];
        let allowed = vec![240, 360, 720, 1080];
        let preferred = vec![240, 360];

        let result = attempt(&available, &allowed, &preferred);

        assert_eq!(vec![240, 720], result);
    }

    #[test]
    fn equal_preffered() {
        let available = vec![240, 360, 720, 1080];
        let allowed = vec![240, 360, 720];
        let preferred = vec![360, 720];

        let result = attempt(&available, &allowed, &preferred);

        assert_eq!(vec![360, 720], result);
    }

    #[test]
    fn take_greater_than_preffered() {
        let available = vec![240, 360, 720, 1080];
        let allowed = vec![240, 360, 720];
        let preferred = vec![480];

        let result = attempt(&available, &allowed, &preferred);

        assert_eq!(vec![720], result);
    }

    #[test]
    fn take_less_than_preffered() {
        let available = vec![240, 360];
        let allowed = vec![240, 360, 720];
        let preferred = vec![480];

        let result = attempt(&available, &allowed, &preferred);

        assert_eq!(vec![360], result);
    }

    #[test]
    fn empty_vec() {
        let available = vec![720];
        let allowed = vec![240, 360, 1080];
        let preferred = vec![240, 360];

        let result = attempt(&available, &allowed, &preferred);
        let empty_vec: Vec<i32> = vec![];
        assert_eq!(empty_vec, result);
    }

    #[test]
    fn no_repeats_vec() {
        let available = vec![240, 360];
        let allowed = vec![240, 360];
        let preferred = vec![720, 1080];

        let result = attempt(&available, &allowed, &preferred);
        assert_eq!(vec![360], result);
    }

    #[test]
    fn vec_sort_without_dublicate() {
        let mut available = vec![360, 240, 360, 720];
        let mut allowed = vec![1080, 720, 240, 720];
        let mut preferred = vec![240, 480];

        ready_to_use(&mut available, &mut allowed, &mut preferred);
        let result = attempt(&available, &allowed, &preferred);
        assert_eq!(vec![240, 720], result);
    }

    #[test]
    fn vec_sort() {
        let mut available = vec![360, 240, 360, 720];
        let mut allowed = vec![1080, 720, 240, 720];
        let mut preferred = vec![240, 480];

        ready_to_use(&mut available, &mut allowed, &mut preferred);

        assert_eq!(vec![240, 360, 720], available);
    }
}