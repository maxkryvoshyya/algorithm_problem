
//! - Алгоритм обирає із *доступних значень* так, щоб вони *задовільняли yci перерахованні умови* та повертає іх у вигляді вектора.
//! - Якщо алгоритм не може повернути *бажане значення*, тоді він намагається повернути *найближче значення, яке є не менше бажаного*, якщо і це не можливо тоді *найближче менше*.
//! - Алгоритм повертає лише ті значення, які є *дозволеними і доступними*.
//! - Алгоритм повертає *порожній вектор, якщо жодне із значень не підходить*.
//! - Результат виконання алгоритму *не має повторів*.
//! - Всі вхідні дані *відсортовані у порядку зростання*.
//!
//! - Також масиви *'allowed'* та *'preferred'* можуть мати один елемент *"any"*.
//!   "any" в масиві дозволених значень означає, що дозволеними є всі значення.
//!   "any" в масиві бажаних значень означає, що бажаним є будь-яке значення.

///
/// * `available` - вектор доступних значень.
/// * `allowed` - вектор дозволених значень.
/// * `preferred` - вектор бажаних значень.
///
#[ derive( Debug ) ]
pub struct Value 
{

	pub available: Vec< i32 >,
	pub allowed: Vec< i32 >,
	pub preferred: Vec< i32 >,

}

impl Value 
{

	/// Функція **attempt** приймає посилання self, реалізує алгоритм
	/// та повертає вектор значень, які задовільняють умови запиту.
	///
	///  # Приклад
	///  
	///  ```
	/// let query = Value {
	///    available: vec![240, 720],
	///    allowed: vec![240, 360, 720, 1080],
	///    preferred: vec![240, 360],
	/// };
	///
	/// let result = query.attempt();
	///
	/// assert_eq!(vec![240, 720], result);
	///  ```
	///
	pub fn attempt( &self ) -> Vec< i32 > 
	{

		let mut selected = Vec::new(); /* <- Змінна selected зберігає спільні значення available та allowed з якими будемо працювати далі  */

		for i in &self.available 
		{
				
			for j in &self.allowed 
			{
					
				if i == j /* <- Якщо співпадає allowed з available значення записується в selected */
				{

					selected.push( *i );

				}

			}
				
		}

		let mut result = Vec::new(); /* <- Змінна result зберігає спільні значення selected та preferred, а у випадку невдачі буде зберігатися більше значчення за бажене або нічого якщо таких значень немає  */
		let mut preferred_exhaustive = self.preferred.clone();  /* <- Змінна preferred_exhaustive клонує значення preferred. У разі успішного результату алгоритму буде видаляти використане значення */

		for i in preferred_exhaustive.iter() 
		{

			for j in selected.iter() 
			{

				if i == j /* <- Якщо співпадає preferred з selected значення записується в result */
				{ 

					result.push( *j );
					break;

				}

			}

		}

		for i in result.iter() 
		{

			for ( ej, j ) in preferred_exhaustive.iter().enumerate() 
			{

				if i == j /* <- Видаляємо використаний елемент preferred_exhaustive */
				{ 

					preferred_exhaustive.remove( ej );
					break;

				}

			}

		}

		if result.len() != self.preferred.len() /* <- Перевіряє чи заповнене result бажаними значеннями */
		{   

			for i in preferred_exhaustive.iter() 
			{

				for j in selected.iter() 
				{

					if i < j 
					{

						result.push( *j ); /* <- Обирається більше значення */
						break;

					}

				}

			}

		}

		result.dedup(); /* <- Видаляємо дублікати так як два менших значення можуть посилатися на спільне більше */

		for i in result.iter() 
		{

			for ( ej, j ) in preferred_exhaustive.iter().enumerate() 
			{

				if i > j /* <- Видаляємо використаний елемент preferred_exhaustive */
				{

					preferred_exhaustive.remove( ej ); 
					break;

				}

			}

		}

		if result.len() != self.preferred.len() 
		{
				
			for i in preferred_exhaustive.iter() 
			{

				for j in selected.iter().rev() /* <- Сортуємо вектор щоб отримати найперше менше значення */
				{
								
					if i > j 
					{

						result.push( *j ); /* <- Обирається менше значення */
						break;

					}

				}

			}

		}

		result.dedup(); /* <- Видаляємо дублікати так як два більших значення можуть посилатися на спільне менше */
		result.sort();  /* <- Сортуємо у порядку зростання */

		result

	}

	/// Функція **ready_to_use** приймає змінне посилання на self.
	/// Сортує значення від найменшого до найбільшого та видаляє дублікати.
	///
	///  # Приклад
	///  
	///  ```
	///  let mut query = Value {
	///    available: vec![360, 240, 360, 720],
	///    allowed: vec![1080, 720, 240, 720],
	///    preferred: vec![240, 480],
	///  };
	///
	///  query.ready_to_use();
	///
	///  assert_eq!(vec![240, 360, 720], query.available);
	///  ```
	///
	pub fn ready_to_use( &mut self ) 
	{

		self.available.sort(); /* <- Сортування значень від найменшого */
		self.available.dedup(); /* <- Видалення дублікатів */
		self.allowed.sort();
		self.allowed.dedup();
		self.preferred.sort();
		self.preferred.dedup();

	}

	/// Функція **print_results** приймає посилання self і результат функції **attempt** та друкує результати.
	///
	///  # Приклад
	///  
	///  ```
	///  let query = Value {
	///     available: vec![240, 360, 720, 1080],
	///     allowed: vec![240, 360, 720],
	///     preferred: vec![360, 720],
	///  };
	///
	///  let result = query.attempt();
	///
	///  query.print_results();
	///  ```
	///
	pub fn print_results( &self, result: Vec< i32 > ) 
	{

		println!("available : {:?}\nallowed : {:?}\npreferred : {:?}\nreturns : {:?}", self.available, self.allowed, self.preferred, result);

	}

}

/// Функція **check_key_any** шукає ключове слово 'any'. Якщо воно відсутнє вектор залишається без змін.
/// У разі співпадіння, вектор очищається і переписується новими даними.
///
///  # Приклад
///  
///  ```
///  let mut allowed = vec!["360", "any"];
///  let mut preferred = vec!["360", "720"];
///
///  check_key_any(&mut allowed);
///  check_key_any(&mut preferred);
///
///  assert_eq!(vec!["240", "360", "480", "720", "1080"], allowed);
///  assert_eq!(vec!["360", "720"], preferred);
///  ```
///
pub fn check_key_any( item: &mut Vec< &str > ) 
{

	let any = [ "240", "360", "480", "720", "1080" ]; /* <- Масив з всіма можливими значеннями 'any' */
	
	if item.iter().any( | &i | i == "any" ) /* <- Проходимося по векторі і якщо знаходимо any, очищаємо масив і переписуємо значеннями з масиву. У разі відсутності any залишаємо вектор незмінним */
	{
			
		item.clear();

		for i in any 
		{

			item.push( i );

		}

	}

}

/// Функція **i32_parse** приймає Vec<&str> та повертає Vec<&i32>.
///
///  # Приклад
///  
///  ```
///  let allowed = vec!["240", "360", "480", "720", "1080"];
///
///  let allowed = i32_parse(allowed);
///
///  assert_eq!(vec![240, 360, 480, 720, 1080], allowed);
///  ```
///
pub fn i32_parse( vec: Vec< &str > ) -> Vec< i32 > /* <- Беремо значення вектора з рядковим літералом у володіння та повертаємо новий вектор i32 */
{
    
	let mut new_vec: Vec< i32 > = Vec::new();

	for i in vec 
	{

		let val: i32 = i.trim().parse().expect( "Error parsing!" );
		new_vec.push( val );

	}

	new_vec

}

#[ cfg( test ) ]
mod test 
{

	use super::*;

	#[ test ]
	fn query_1() 
	{

		let mut query = Value 
		{

			available: vec![ 240, 360, 720 ],
			allowed: vec![ 360, 720 ],
			preferred: vec![ 1080 ],

		};

		query.ready_to_use();

		let result = query.attempt();

		assert_eq!( vec![ 720 ], result ); /* <- У прикладі ТЗ вказано [360], що є помилкою. Алгоритм повинен повертати найближче менше до бажаного. У даному випадку це [720] */ 

	}

	#[ test ]
	fn query_2() 
	{

		let mut query = Value 
		{

			available: vec![ 240, 720 ],
			allowed: vec![ 360, 720 ],
			preferred: vec![ 1080 ],

		};

		query.ready_to_use();

		let result = query.attempt();

		assert_eq!( vec![ 720 ], result );

	}

	#[ test ]
	fn query_3() 
	{

		let mut query = Value 
		{

			available: vec![ 240 ],
			allowed: vec![ 360, 720 ],
			preferred: vec![ 1080 ],

		};

		query.ready_to_use();

		let result = query.attempt();

		let empty_vector: Vec< i32 > = Vec::new();

		assert_eq!( empty_vector, result );

	}

	#[ test ]
	fn query_4() 
	{

		let mut query = Value 
		{

			available: vec![ 240, 360, 720 ],
			allowed: vec![ 240, 360, 720, 1080 ],
			preferred: vec![ 240, 360 ],

		};

		query.ready_to_use();

		let result = query.attempt();

		assert_eq!( vec![ 240, 360 ], result );

	}

	#[ test ]
	fn query_5() 
	{

		let mut query = Value 
		{

			available: vec![ 240, 720 ],
			allowed: vec![ 240, 360, 720, 1080 ],
			preferred: vec![ 240, 360 ],

		};

		query.ready_to_use();

		let result = query.attempt();

		assert_eq!( vec![ 240, 720 ], result );

	}

	#[ test ]
	fn query_6() 
	{

		let mut query = Value 
		{

			available: vec![ 240, 720 ],
			allowed: vec![ 240, 360, 1080 ],
			preferred: vec![ 240, 360 ],

		};

		query.ready_to_use();

		let result = query.attempt();

		assert_eq!( vec![ 240 ], result );

	}

	#[ test ]
	fn query_7() 
	{

		let mut query = Value 
		{

			available: vec![ 720 ],
			allowed: vec![ 240, 360, 1080 ],
			preferred: vec![ 240, 360 ],

		};

		query.ready_to_use();

		let result = query.attempt();
		let empty_vector: Vec< i32 > = Vec::new();

		assert_eq!( empty_vector, result );

	}

	#[ test ]
	fn query_8() 
	{

		let mut query = Value 
		{

			available: vec![ 240, 360 ],
			allowed: vec![ 240, 360 ],
			preferred: vec![ 720, 1080 ], /* <- В ТЗ не вказано як повинен працювати алгоритм якщо один preferred вже захопив нижче число, і ще один preferred повинен повинен брати те саме число чи нижче за перше */

		};

		query.ready_to_use();

		let result = query.attempt();

		assert_eq!( vec![ 360 ], result ); /* <- [ 240, 360 ] - можливий результат */

	}

	#[ test ]
	fn query_9() 
	{

		let available = vec![ 240, 360, 720 ];
		let mut allowed = vec![ "360", "any" ];
		let mut preferred = vec![ "360", "720" ];

		check_key_any( &mut allowed );
		check_key_any( &mut preferred );

		let allowed = i32_parse( allowed );
		let preferred = i32_parse( preferred );

		let mut query = Value 
		{

			available,
			allowed,
			preferred,

		};

		query.ready_to_use();

		let result = query.attempt();

		assert_eq!( vec![ 360, 720 ], result );

	}

	#[ test ]
	fn query_10() 
	{

		let available = vec![ 240, 360, 720 ];
		let mut allowed = vec![ "240", "360", "720" ];
		let mut preferred = vec![ "any", "720" ];

		check_key_any( &mut allowed );
		check_key_any( &mut preferred );

		let allowed = i32_parse( allowed );
		let preferred = i32_parse( preferred );

		let mut query = Value 
		{

			available,
			allowed,
			preferred,

		};

		query.ready_to_use();

		let result = query.attempt();

		assert_eq!( vec![ 240, 360, 720 ], result );

	}

	#[ test ]
	fn query_11() 
	{

		let available = vec![ 240, 360, 720 ];
		let mut allowed = vec![ "360", "1080" ];
		let mut preferred = vec![ "any", "720" ];

		check_key_any( &mut allowed );
		check_key_any( &mut preferred );

		let allowed = i32_parse( allowed );
		let preferred = i32_parse( preferred );

		let mut query = Value 
		{

			available,
			allowed,
			preferred,

		};

		query.ready_to_use();

		let result = query.attempt();

		assert_eq!( vec![ 360 ], result );

	}

	#[ test ]
	fn query_12() 
	{

		let available = vec![ 240, 360, 720 ];
		let mut allowed = vec![ "1080" ];
		let mut preferred = vec![ "any", "720" ];

		check_key_any( &mut allowed );
		check_key_any( &mut preferred );

		let allowed = i32_parse( allowed );
		let preferred = i32_parse( preferred );

		let mut query = Value 
		{

			available,
			allowed,
			preferred,
			
		};

		query.ready_to_use();

		let result = query.attempt();
		let empty_vector: Vec< i32 > = Vec::new();

		assert_eq!( empty_vector, result );

	}

}
