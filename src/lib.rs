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
///  let available = vec![240, 720];
///  let allowed = vec![240, 360, 720, 1080];
///  let preferred = vec![240, 360];
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
                                           // значчення за баражене або нічого якщо таких значень немає 
    for i in preferred {
        for j in selected.iter() {
            if i == j {
                result.push(*j); // Якщо співпадає preferred з selected значення записується в result
                break;
            } else if j > i {
                result.push(*j); // Обирається більше значення, якщо немає збігу із баженим
                break;
            } 
        }
    }
    result
}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn work() {
        let available = vec![240, 720];
        let allowed = vec![240, 360, 720, 1080];
        let preferred = vec![240, 360];

        let result = attempt(&available, &allowed, &preferred);

        assert_eq!(vec![240, 720], result);
    }
}