#[cfg(test)]
mod tests_simple_struct {
    use strum_macros::EnumIter;
use field_accessor::FieldAccessor;
    use strum_macros::AsRefStr;
    use strum_macros::EnumString;

    #[derive(FieldAccessor)]
    pub struct Dog {
        name: String,
        age: u32,
        life_expectancy: u32,
    }

    #[test]
    fn test_get_on_invalid_name() {
        let mut dog = Dog {
            name: "Taro".to_string(),
            age: 3,
            life_expectancy: 9,
        };
        let field_name = "name".to_string();
        let value_to_update = "Jiro".to_string();
        dog.set(&field_name, value_to_update).unwrap();
        let value_on_error;
        let fieldvalue: &String = match dog.get(&"invalid_name".to_string()) {
            Ok(value) => value,
            Err(_) => {
                value_on_error = "Ken".to_string();
                &value_on_error
            }
        };
        assert_eq!(fieldvalue, &"Ken".to_string());
    }

    #[test]
    fn test_get_mut() {
        let mut dog = Dog {
            name: "Taro".to_string(),
            age: 3,
            life_expectancy: 9,
        };
        let v: &mut String = dog.get_mut(&"name".to_string()).unwrap();
        *v = "Jiro".to_string();
        assert_eq!(dog.name, "Jiro".to_string());
    }

    #[test]
    fn test_set() {
        let mut dog = Dog {
            name: "Taro".to_string(),
            age: 3,
            life_expectancy: 9,
        };
        let field_name = "age".to_string();
        let value_to_update = 4u32;
        dog.set(&field_name, value_to_update).unwrap();
        let fieldvalue: &u32 = dog.get(&field_name).unwrap();
        assert_eq!(fieldvalue, &4u32);
    }

    #[test]
    fn test_get_error() {
        let dog = Dog {
            name: "Taro".to_string(),
            age: 3,
            life_expectancy: 9,
        };
        let field_name = "favorite_food".to_string();
        let fieldvalue: Result<&String, String> = dog.get(&field_name);
        assert!(fieldvalue.is_err());
    }

    #[test]
    fn test_iterate_with_enum() {
        let dog = Dog {
            name: "Taro".to_string(),
            age: 3,
            life_expectancy: 9,
        };
        let fields = vec![
            "name".to_string(),
            "age".to_string(),
            "life_expectancy".to_string(),
        ];
        let mut fieldvalues: Vec<DogFieldEnum> = vec![];
        for field_name in fields.into_iter() {
            fieldvalues.push(dog.getenum(&field_name).unwrap());
        }
        assert_eq!(fieldvalues[0], DogFieldEnum::name("Taro".to_string()));
        assert_eq!(fieldvalues[1], DogFieldEnum::age(3));
        assert_eq!(fieldvalues[2], DogFieldEnum::life_expectancy(9));
    }
}

#[cfg(test)]
mod test_mem {
    use strum_macros::EnumIter;
use field_accessor::FieldAccessor;
    use strum_macros::AsRefStr;
    use strum_macros::EnumString;

    #[derive(FieldAccessor)]
    pub struct Dog {
        name: String,
        age: u32,
        life_expectancy: u32,
        friends: Vec<String>,
    }

    #[test]
    fn test_take() {
        let mut dog = Dog {
            name: "Taro".to_string(),
            age: 3,
            life_expectancy: 9,
            friends: vec!["Mike".to_string(), "Nozomi".to_string()],
        };
        let field_name = "name".to_string();
        let v: String = dog.take(&field_name).unwrap();
        assert_eq!(v, "Taro".to_string());
        assert_eq!(dog.name, "".to_string());
    }

    #[test]
    fn test_replace() {
        let mut dog = Dog {
            name: "Taro".to_string(),
            age: 3,
            life_expectancy: 9,
            friends: vec!["Mike".to_string(), "Nozomi".to_string()],
        };
        let field_name = "name".to_string();
        let v: String = dog
            .replace(&field_name, "Taro_replaced".to_string())
            .unwrap();
        assert_eq!(v, "Taro".to_string());
        assert_eq!(dog.name, "Taro_replaced".to_string());
    }

    #[test]
    fn test_swap() {
        let mut dog = Dog {
            name: "Taro".to_string(),
            age: 3,
            life_expectancy: 9,
            friends: vec!["Mike".to_string(), "Nozomi".to_string()],
        };
        let field_name = "age".to_string();
        let field_name_for_swap = "life_expectancy".to_string();
        dog.swap(&field_name, &field_name_for_swap).unwrap();
        assert_eq!(dog.age, 9);
        assert_eq!(dog.life_expectancy, 3);
    }
}

#[cfg(test)]
mod tests_vector_type {
    use strum_macros::EnumIter;
use field_accessor::FieldAccessor;
    use strum_macros::AsRefStr;
    use strum_macros::EnumString;

    #[derive(FieldAccessor)]
    pub struct Dog {
        name: String,
        age: u32,
        life_expectancy: u32,
        friends: Vec<String>,
    }

    #[test]
    fn test_simple_vector_field() {
        let mut dog = Dog {
            name: "Taro".to_string(),
            age: 3,
            life_expectancy: 9,
            friends: vec!["Mike".to_string(), "Nozomi".to_string()],
        };
        let field_name = "friends".to_string();
        let value_to_update = vec!["Makoto".to_string(), "Maya".to_string()];
        dog.set(&field_name, value_to_update).unwrap();
        let fieldvalue: &Vec<String> = dog.get(&field_name).unwrap();
        assert_eq!(fieldvalue, &vec!["Makoto".to_string(), "Maya".to_string()]);
    }
}

#[cfg(test)]
mod tests_getstructinfo {
    use strum_macros::EnumIter;
use field_accessor::FieldAccessor;
    use strum_macros::AsRefStr;
    use strum_macros::EnumString;

    #[derive(FieldAccessor)]
    pub struct Dog {
        name: String,
        age: u32,
        life_expectancy: u32,
    }

    #[test]
    fn test_getstructinfo() {
        let dog = Dog {
            name: "Taro".to_string(),
            age: 3,
            life_expectancy: 9,
        };
        let info = dog.getstructinfo();
        let mut fieldvalues = vec![];
        for i in info.field_names.iter() {
            fieldvalues.push(dog.getenum(i).unwrap());
        }
        assert_eq!(fieldvalues[0], DogFieldEnum::name("Taro".to_string()));
        assert_eq!(fieldvalues[1], DogFieldEnum::age(3));
        assert_eq!(fieldvalues[2], DogFieldEnum::life_expectancy(9));
    }
}

#[cfg(test)]
mod tests_multiple_derive {
    use strum_macros::EnumIter;
use strum_macros::AsRefStr;
    use strum_macros::EnumString;

    #[test]
    fn test_multiple_derive() {
        use field_accessor::FieldAccessor;

        #[derive(FieldAccessor)]
        struct Test {
            pub name: String,
        }

        #[derive(FieldAccessor)]
        struct Test2 {
            pub name: String,
        }
    }
}

#[cfg(test)]
mod tests_nested_structs {
    use strum_macros::EnumIter;
use strum::IntoEnumIterator;
    use strum_macros::AsRefStr;
    use strum_macros::EnumString;

    #[test]
    fn test_nested_structs() {
        use field_accessor::FieldAccessor;

        // Todo Find an elegant way to use FieldAccessor without passing these traits to derive.
        #[derive(PartialEq, PartialOrd, Default, Clone, Debug)]
        pub struct UserData {
            some_field: String,
            some_value: i32,
        }

        #[derive(FieldAccessor)]
        pub struct User {
            name: String,
            data: UserData,
        }

        let user_data = UserData {
            some_field: "some value".to_string(),
            some_value: 123,
        };
        let my_user = User {
            name: "aGoodName".to_string(),
            data: user_data,
        };
        let field_name = "name".to_string();
        let name: &String = my_user.get(&field_name).unwrap();
        let userdata: &UserData = my_user.get(&"data".to_string()).unwrap();
        assert_eq!(*name, "aGoodName".to_string());
        assert_eq!(*userdata.some_field, "some value".to_string());
    }

    #[test]
    fn test_field_names() {
        use field_accessor::FieldAccessor;

        // Todo Find an elegant way to use FieldAccessor without passing these traits to derive.
        #[derive(PartialEq, PartialOrd, Default, Clone, Debug)]
        pub struct UserData {
            some_field: String,
            some_value: i32,
        }

        #[derive(FieldAccessor)]
        pub struct User {
            name: String,
            data: UserData,
        }

        let user_data = UserData {
            some_field: "some value".to_string(),
            some_value: 123,
        };
        let my_user = User {
            name: "aGoodName".to_string(),
            data: user_data,
        };
        let field_name = UserFields::name.as_ref().to_string();
        let name: &String = my_user.get(&field_name).unwrap();
        let userdata: &UserData = my_user.get(&UserFields::data.as_ref().to_string()).unwrap();
        assert_eq!(*name, "aGoodName".to_string());
        assert_eq!(*userdata.some_field, "some value".to_string());

        assert_eq!(UserFields::iter().collect::<Vec<_>>().len(), UserTypeList.len());
    }
}
