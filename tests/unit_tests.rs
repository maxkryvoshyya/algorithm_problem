use algorithm_problem::*;

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

    assert_eq!( vec![ 360 ], result ); 

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