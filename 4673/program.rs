fn get_self_number(num: u16) -> u16 {
    let mut result: u16 = num;
    let nums = format!("{}", num);
    let numsp = nums
        .split("")
        .collect::<Vec<&str>>();
    for n in numsp.iter() {
        result += match n.parse::<u16>() {
            Ok(p) => p,
            Err(_) => 0,
        }
    }
    return result;
}
// for문 3번 쓰는게 상당히 이상해 보이지만 일단...
fn main () {
    let mut self_number: Vec<u16> = Vec::new();
    for i in 1..9999 {
        let t = get_self_number(i);
        self_number.push(t);
    }
    let mut man = (1..9999).collect::<Vec<u16>>();
    for n in self_number.iter() {
        man.retain(|x| x != n);
    }
    for n in man {
        println!("{}", n);
    }
}
