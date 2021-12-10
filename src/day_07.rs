use nom::IResult;

pub fn star_1(data: String) {
    let mut crabs = parse(&data);
    crabs.sort_unstable();

    // Given the fuel function f, we know that the midpoint of the list mid is a local minimum.
    // When the position is moved 1 to the left, all points on the left decrease by 1, and all
    // points on the right increase by one. Additionally, if there is a point at mid, it increases
    // by one. If there are an equal number of points to the left and right, that means that either
    // the point to the left has the same fuel value as the current point, or greater. The same is
    // true when we move 1 to the right. Therefore, if we start at the midpoint, moving can only
    // either keep the fuel value the same or make it go up.

    // We also know that there can be no maxima of the function. Assume that a point x is a maximum.
    // Then f(x) >= f(x - 1) and f(x) >= f(x + 1). We can substitute equations based on f(a) (where
    // left is the function that returns the number of points to the left of a, right is the
    // function of the points to the right, and center is 1 or 0 depending on if there is a point
    // at x):
    // f(x - 1) = f(x) - left(a) + center(a) + right(a)
    // f(x + 1) = f(x) + left(a) + center(a) - right(a)
    // This can be rewritten as the system of inequalities:
    // -left(a) + center(a) + right(a) <= 0
    // left(a) + center(a) - right(a) <= 0
    // Rearranging these, we have:
    // left(a) >= right(a) + center(a)
    // left(a) <= right(a) - center(a)
    // This is a contradiction (since left, right, and center can only return positive numbers);
    // therefore, no maxima can exist.

    // Without any maxima, there can only be one minimum, which makes the midpoint the global
    // minimum.

    let mut mid = crabs[crabs.len() / 2];

    // It's potentially possible for there to be multiple valid smallest fuel amounts; if the
    // middle two points are only 1 distance apart, they are both valid candidates. For simplicity,
    // we choose the point 1 after the point before the middle if there is no true midpoint.
    if crabs.len() % 2 != 0 {
        mid += 1;
    }

    let fuel = fuel_sum(&crabs[..], mid, |x| x);
    println!("{}", fuel);
}

pub fn star_2(data: String) {
    fn triangle(x: u64) -> u64 {
        (x * (x + 1)) / 2
    }

    let crabs = parse(&data);

    // The best position is the one that reduces the distance to all points as much as possible.
    // I do not have a proof of this yet.
    let mean = crabs.iter().sum::<u64>() / crabs.len() as u64;

    // I don't know why we can't just round mean here, but it doesn't work, so we have to check
    // both floor and ceiling.
    let left = fuel_sum(&crabs[..], mean, triangle);
    let right = fuel_sum(&crabs[..], mean + 1, triangle);
    let fuel = left.min(right);
    println!("{}", fuel);
}

fn fuel_sum<F>(crabs: &[u64], pos: u64, dist_to_fuel: F) -> u64
where
    F: Fn(u64) -> u64,
{
    fn abs_min(a: u64, b: u64) -> u64 {
        if a > b {
            a - b
        } else {
            b - a
        }
    }

    crabs
        .iter()
        .map(|&crab| abs_min(crab, pos))
        .map(dist_to_fuel)
        .sum()
}

fn parse(input: &str) -> Vec<u64> {
    super::utils::parse(crabs, input)
}

fn crabs(input: &str) -> IResult<&str, Vec<u64>> {
    use nom::{bytes::complete::tag, character::complete::u64 as u64_, multi::separated_list0};

    separated_list0(tag(","), u64_)(input)
}
