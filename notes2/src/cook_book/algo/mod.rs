use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u8, 0-100: {}", rng.gen_range(0..=100));

    // 从一组字母数字字符创建随机密码
    let rand_string: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let mut arr = vec![19.1, 2.1, 3.2, 4.3, 5.4];
    arr.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    assert!(arr == vec![2.1, 3.2, 4.3, 5.4, 19.1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
