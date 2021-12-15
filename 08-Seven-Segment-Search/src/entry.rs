pub struct Entry {
    pub wirings: Vec<String>,
    pub output: Vec<String>,
}

impl Entry {
    pub fn from_str(string: &str) -> Self {
        let input: Vec<&str> = string.split("|").collect();
        let wirings = input
            .get(0)
            .expect("Could not parse wirings")
            .trim()
            .split(" ")
            .map(|string| string.to_owned())
            .collect();
        let output = input
            .get(1)
            .expect("Could not parse outputs")
            .trim()
            .split(" ")
            .map(|string| string.to_owned())
            .collect();

        Entry { wirings, output }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_entry_from_string() {
        let input =
            "bgcfda gecbda abdgf aedfbg eda efcbd ae agfe bdefagc fbeda | ae egdafb ea fcdeb";

        let entry = Entry::from_str(input);

        assert_eq!(10, entry.wirings.len());
        assert_eq!(4, entry.output.len());
    }
}
