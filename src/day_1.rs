// You probably know the "like" system from Facebook and other pages. People can "like" blog posts, pictures or other items. We want to create the text that should be displayed next to such an item.

// Implement the function which takes an array containing the names of people that like an item. It must return the display text as shown in the examples:

// ```
// []                                -->  "no one likes this"
// ["Peter"]                         -->  "Peter likes this"
// ["Jacob", "Alex"]                 -->  "Jacob and Alex like this"
// ["Max", "John", "Mark"]           -->  "Max, John and Mark like this"
// ["Alex", "Jacob", "Mark", "Max"]  -->  "Alex, Jacob and 2 others like this"
// ```
// Note: For 4 or more names, the number in "and 2 others" simply increases.
#[allow(dead_code)]
pub fn likes(names: &[&str]) -> String {
    let num_names = names.len();
    if num_names == 0 {
        String::from("no one likes this")
    } else if num_names == 1 {
        format!("{} likes this", names[0])
    } else if num_names == 2 {
        format!("{} and {} like this", names[0], names[1])
    } else if num_names == 3 {
        format!("{}, {} and {} like this", names[0], names[1], names[2])
    } else {
        format!(
            "{}, {} and {} others like this",
            names[0],
            names[1],
            num_names - 2
        )
    }
}

#[cfg(test)]
mod tests {
    use super::likes;

    #[test]
    fn example_tests() {
        assert_eq!(likes(&[]), "no one likes this");
        assert_eq!(likes(&["Peter"]), "Peter likes this");
        assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
        assert_eq!(
            likes(&["Max", "John", "Mark"]),
            "Max, John and Mark like this"
        );
        assert_eq!(
            likes(&["Alex", "Jacob", "Mark", "Max"]),
            "Alex, Jacob and 2 others like this"
        );
    }
}
