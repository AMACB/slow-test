#[cfg(test)]
mod test {
    use std::time::Instant;

    fn get_sum(lhs_data: &[u64], rhs_data: &[u64]) -> u64 {
        let mut sum = 0;
        for k in (0..(1 << 26)).step_by(4) {
            let lhs = lhs_data[k];
            let rhs = rhs_data[k];
            sum += lhs * rhs;
        }
        sum
    }

    #[test]
    fn what() {
        let data1: Vec<u64> = (0..(1 << 26)).collect();
        let data2 = data1.clone();
        for i in 0..16 {
            let begin = Instant::now();
            let sum = get_sum(&data1.as_slice(), &data2.as_slice());
            let end = Instant::now();
            eprintln!("[iteration {}] time = {:?}, sum = {}", i, end - begin, sum);
        }
    }
}
