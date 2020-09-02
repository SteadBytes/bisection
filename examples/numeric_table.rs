use bisection::bisect;

fn grade(score: &i32, breakpoints: &[i32], grades: &[char]) -> char {
    let i = bisect(breakpoints, score);
    grades[i]
}

fn main() {
    let breakpoints = [60, 70, 80, 90];
    let grades: Vec<_> = "FDCBA".chars().collect();
    let scores = [33, 99, 77, 70, 89, 90, 100];

    let results = scores.iter().map(|s| grade(s, &breakpoints, &grades));

    for (s, g) in scores.iter().zip(results) {
        println!("{} -> {}", s, g)
    }
}
