pub type Value = i32;

pub struct ValueArray {
    pub count : usize,
    pub values : Vec<Value>
}

pub fn init_value_array() -> ValueArray {
    ValueArray { count: 0, values: Vec::with_capacity(10) }
}

pub fn write_value_array(array : &mut ValueArray, value: Value) -> usize {
    array.values.push(value);
    array.count += 1;
    array.count - 1
}

pub fn free_value_array(array : &mut ValueArray) {
    *array = init_value_array();
}

pub fn print_value(value: Value) {
    print!("{:?}", value);
}
