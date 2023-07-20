/* В цьому файлі зберігаються власні тести до алгоритму, основні тести по прикладах ТЗ знаходяться у lib.rs*/
use out_of_the_box::*;

#[test]
    fn get_vec_of_value() 
    {
        
        let query = Value 
        {
            available: vec![240, 720],
            allowed: vec![240, 360, 720, 1080],
            preferred: vec![240, 360],
        };

        let result = query.attempt();

        assert_eq!(vec![240, 720], result);

    }

    #[test]
    fn equal_preffered() 
    {
        
        let query = Value 
        {
            available: vec![240, 360, 720, 1080],
            allowed: vec![240, 360, 720],
            preferred: vec![360, 720],
        };

        let result = query.attempt();

        assert_eq!(vec![360, 720], result);

    }

    #[test]
    fn take_greater_than_preffered() 
    {
        
        let query = Value 
        {
            available: vec![240, 360, 720, 1080],
            allowed: vec![240, 360, 720],
            preferred: vec![480],
        };

        let result = query.attempt();

        assert_eq!(vec![720], result);

    }

    #[test]
    fn take_less_than_preffered() 
    {

        let query = Value 
        {
            available: vec![240, 360],
            allowed: vec![240, 360, 720],
            preferred: vec![480],
        };

        let result = query.attempt();

        assert_eq!(vec![360], result);

    }

    #[test]
    fn empty_vec() 
    {

        let query = Value 
        {
            available: vec![720],
            allowed: vec![240, 360, 1080],
            preferred: vec![240, 360],
        };

        let result = query.attempt();
        let empty_vec: Vec<i32> = vec![];

        assert_eq!(empty_vec, result);

    }

    #[test]
    fn no_repeats_vec() 
    {

        let query = Value 
        {
            available: vec![240, 360],
            allowed: vec![240, 360],
            preferred: vec![720, 1080],
        };

        let result = query.attempt();

        assert_eq!(vec![360], result);

    }

    #[test]
    fn vec_sort() 
    {

        let mut query = Value 
        {
            available: vec![360, 240, 360, 720],
            allowed: vec![1080, 720, 240, 720],
            preferred: vec![240, 480],
        };

        query.ready_to_use();

        assert_eq!(vec![240, 360, 720], query.available);

    }

    #[test]
    fn vec_sort_without_dublicate() 
    {

        let mut query = Value 
        {
            available: vec![360, 240, 360, 720],
            allowed: vec![1080, 720, 240, 720],
            preferred: vec![240, 480],
        };

        query.ready_to_use();

        let result = query.attempt();

        assert_eq!(vec![240, 720], result);

    }

    #[test]
    fn check_key_word_any() 
    {
        
        let mut allowed = vec!["360", "any"];
        let mut preferred = vec!["360", "720"];

        check_key_any(&mut allowed);
        check_key_any(&mut preferred);

        assert_eq!(vec!["240", "360", "480", "720", "1080"], allowed);
        assert_eq!(vec!["360", "720"], preferred);

    }

    #[test]
    fn check_i32_parse() 
    {

        let allowed = vec!["240", "360", "480", "720", "1080"];

        let allowed = i32_parse(allowed);

        assert_eq!(vec![240, 360, 480, 720, 1080], allowed);
        
    }