mod iamnew{
    pub struct Pizza{
        
        pub dough: String,
        pub cheese:i32

        
    }
    
    impl Pizza{
        pub fn lunch(toppings : &str)->Pizza{
            Pizza{
                dough:String::from(toppings),
                cheese:5
            }
        }
    }
    }