pub mod math {
    fn euclidean_gcd(mut a: u128, mut b: u128) -> u128 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    pub fn lcm(a: u128, b: u128) -> u128 {
        if a == 0 || b == 0 {
            0
        } else {
            (a / euclidean_gcd(a, b)) * b
        }
    }

    fn lcm_of_set(numbers: &[u128]) -> u128 {
        let mut current_lcm = numbers[0];

        for &num in &numbers[1..] {
            current_lcm = lcm(current_lcm, num);
        }

        current_lcm
    }

}


