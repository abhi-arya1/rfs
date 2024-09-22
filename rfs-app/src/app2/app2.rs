use crate::consts;

pub fn test_func() -> u32 { 
   consts::RETURN_VALUE as u32
}


pub fn app2_main() -> u32 {
    test_func()
}