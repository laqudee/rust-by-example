pub mod deep;
pub mod mode;

pub mod my_mod {
    fn private_function() {
        println!("called my_mod::private_function");
    }

    pub fn function() {
        println!("called my_mod::function");
    }

    pub fn indirect_access() {
        print!("called my_mod::indirect_access(), that\n ");
        private_function();
    }

    // 模块套模块
    pub mod nested {
        pub fn function() {
            println!("called my_mod::nested::function()");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called my_mod::nested::private_function()");
        }

        // 使用pub(in path)语法定义的函数只在给定的路径中可见
        // path 必须是父模块parent module或祖先模块ancestor module
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called my_mod::nested::public_function_in_my_mod(), that\n ");
            public_function_in_nested()
        }

        // 使用pub(self)语法定义的函数则只在当前模块中可见
        pub(self) fn public_function_in_nested() {
            println!("called my_mod::nested::public_function_in_nested");
        }

        // pub(super) 语法定义的函数则只在当前模块中可见
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called my_mod::call_public_function_in_my_mod(), that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate)使得函数只在当前crate中可见
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }

    // 嵌套模块的可见性遵循相同的规则
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called my_mod::private_nested::function()");
        }
    }
}


pub fn my_mod_test() {
  my_mod::public_function_in_crate();
}

