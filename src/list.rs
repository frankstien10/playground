use std::sync::{Arc, Mutex};

struct List<T>{
    data: T,
    ptr: Arc<Mutex<&List<T>>>

}

pub fn making_list<T>(input : Vec<T>)->List{

}
