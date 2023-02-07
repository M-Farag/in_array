fn main() {
    let items:[&str;5] = ["text_1","text_2","text_3","text_4","text_5"];
    let item:&str = "text_1";
    
    match in_array(item, &items) {
        true => println!("Item exist in the array"),
        false => println!("Item does not exist in the array")
    }
    
}


fn in_array(needle:&str,haystack:&[&str;5]) -> bool
{
    for &item in haystack {
        if needle == item {
            return true
        }
    }
    false
}