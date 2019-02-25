fn arr_to_string(arr: &[i32]) -> String {
    let mut res = '['.to_string();
    for v in arr {
        res += &v.to_string();
        res.push(',');
    }
    res.pop();
    res.push(']');
    res
}

fn main() {
    let arr = arr_to_string(&[10, 20, 30]);
    let res = format!("hello {}", arr);
    println!("{}", res);
}
