#[cfg(test)]
mod tests {
    use simple_compose::*;

    fn sum_duration(ds: &Vec<Duration>) -> f64 {
        ds.iter().fold(0.0f64, |acc, x| acc + x)
    }

    fn do_a_measure_test(beat: u8) {
        let ds = duration_utils::generate_one_measure(beat);
        println!("{:?}", ds);
        assert_eq!(sum_duration(&ds), beat as f64);
    }

    #[test]
    fn test_duration_1() {
        assert_eq!(Duration::Whole + Duration::Half, 1.5);
        assert_eq!(<Duration as Into<f32>>::into(Duration::Half), 0.5);
        assert_eq!(Duration::from(0.5), Duration::Half);
    }

    #[test]
    fn test_measure_1() {
        do_a_measure_test(4);
        do_a_measure_test(4);
        do_a_measure_test(4);
        do_a_measure_test(4);
        do_a_measure_test(4);
        do_a_measure_test(4);
        do_a_measure_test(4);
        do_a_measure_test(4);
        do_a_measure_test(4);
        do_a_measure_test(4);
        do_a_measure_test(4);
    }
}