use algorithm_problem::{ check_key_any, i32_parse };

fn main() 
{

  let available= vec![ 240, 360, 720 ];
  let mut allowed = vec![ "360", "any" ];
  let mut preferred = vec![ "360", "720" ];

  check_key_any( &mut allowed ); /* <- Перевірка вектора на значення 'any' */
  check_key_any( &mut preferred );

  let allowed = i32_parse( allowed ); /* <- Конвертація Vec<&str> в Vec<i32> */
  let preferred = i32_parse( preferred );

  let mut query = algorithm_problem::Value /* <- Присвоюємо змінній структуру, з новими значеннями */
  { 

    available,
    allowed,
    preferred,

  };

  query.ready_to_use(); /* <- Сортування та видалення дублікатів */

  let res = query.attempt(); /* <- Обробка даних алгоритмом */

  query.print_results( res ); /* <- Друк результатів */
    
}

