#![feature(plugin)]
#![plugin(cargo_snippet)]

pub mod procon;

// Stub of tests mod.

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_ok() {
        assert_eq!(7, 1 + 2 * 3);
    }
}

// Annotate snippet name
#[snippet = "mymath"]
#[snippet = "gcd"]
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Also works
#[snippet(name = "mymath")]
// Equivalent to #[snippet = "lcm"]
#[snippet]
pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[snippet]
// Include snippet
#[snippet(include = "gcd")]
pub fn gcd_list(list: &[u64]) -> u64 {
    list.iter().fold(list[0], |a, b| gcd(a, *b))
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(57, 3), 3);
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(3, 19), 57);
}
