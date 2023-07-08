fn main(){
    {
        let s = String::from("hello");  // s is valid from this point forwards
        // do stuff with s

    }                                   // this scope is now over, and s is no
                                        // longer valid
}