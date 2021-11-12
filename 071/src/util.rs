pub const PI_2: f32 = 2.0 * std::f32::consts::PI;
pub const PI_2_3: f32 = PI_2 / 3.0;

pub fn longest_contig_substr(s: &[usize]) -> (usize, usize) {
    let mut starting = 0;
    let mut streak = 0;
    let (mut best_starting, mut best_streak) = (0, 0);
    for i in 1..s.len() {
        if s[i - 1] + 1 == s[i] {
            streak += 1;
        } else {
            if streak > best_streak {
                best_starting = starting;
                best_streak = streak;
            }
            streak = 0;
            starting = i;
        }
    }
    if streak > best_streak {
        best_starting = starting;
        best_streak = streak;
    }

    (best_starting, best_streak + 1)
}
