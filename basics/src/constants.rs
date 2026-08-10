#[macro_export]
macro_rules! constants_demo {
    () => {
        // an unchangeable variable/value
        const THRESHOLD: i32 = 10;
        println!("THRESHOLD: {}", THRESHOLD);

        // const variables cannot be reassigned!
        //THRESHOLD = 5;

        // static variables are valid during the whole program execution (and are per default constant)
        static LANGUAGE: &str = "Rust";
        println!("LANGUAGE: {}", LANGUAGE);

        // static variables cannot be reassigned per default!
        //LANGUAGE = "C++";

        // static variables can be made mutable but this is 'unsafe' since they are not protected from concurrent access!
        static mut LANGUAGE_CODE: &str = "English";
        unsafe {
            LANGUAGE_CODE = "English (American)";
            // since its unsafe it should not be used at all!
        }

        // to use static variables safley (with concurrent code) a mutex should be used
        use std::sync::Mutex;
        static LANGUAGE_CODE_MUTEX: Mutex<&str> = Mutex::new("English");

        // The star is dereferencing the mutex guard:
        // LANGUAGE_CODE_MUTEX.lock()                      -> lock the mutex so only one piece of code accesses the value
        // let guard = LANGUAGE_CODE_MUTEX.lock().unwrap() -> get the mutex guard
        // *LANGUAGE_CODE_MUTEX.lock().unwrap()            -> get/set the actual value stored by the guard
        println!("LANGUAGE_CODE_MUTEX: {}", *LANGUAGE_CODE_MUTEX.lock().unwrap());
        *LANGUAGE_CODE_MUTEX.lock().unwrap() = "English (American)";
        println!("LANGUAGE_CODE_MUTEX: {}", *LANGUAGE_CODE_MUTEX.lock().unwrap());
    };
}
