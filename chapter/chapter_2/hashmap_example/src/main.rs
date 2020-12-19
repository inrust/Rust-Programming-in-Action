use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, i32> = HashMap::new();

    let zhangsan1 = map.insert("zhangsan", 97);
    map.insert("lisi", 86);

    println!("{:?}", zhangsan1);
    println!("{:?}", map);

    let zhangsan2 = map.insert("zhangsan", 79);
    println!("{:?}", zhangsan2);
    println!("{:?}", map);

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.entry("zhangsan").or_insert(97);
    map.entry("lisi").or_insert(86);
    println!("{:?}", map);

    map.entry("zhangsan").or_insert(79);
    println!("{:?}", map);

    map.insert("zhangsan", 97);
    map.insert("lisi", 86);
    map.insert("wangwu", 55);
    println!("{:?}", map);

    for (_, val) in map.iter_mut() {
        *val += 2;
    }
    println!("{:?}", map);

    let result = map.remove("wangwu");
    println!("{:?}", map);
    println!("{:?}", result);

    println!("zhangsan: {}", map["zhangsan"]);
    // println!("wangwu: {}", map["wangwu"]);

    println!("zhangsan: {:?}", map.get("zhangsan"));
    println!("wangwu: {:?}", map.get("wangwu"));
}
