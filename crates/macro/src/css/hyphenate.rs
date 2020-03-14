// Taken and modified from
// https://github.com/whatisinternet/Inflector/blob/e864d2c336ff28debc9e9344714e28b13e022bdd/src/cases/case/mod.rs
pub fn to_hyphen_case(input: &str) -> String {
    let mut first_character: bool = true;
    let mut result: String = String::with_capacity(input.len() * 2);
    for char_with_index in trim_right(input).char_indices() {
        if char_is_seperator(&char_with_index.1) {
            if !first_character {
                first_character = true;
                result.push('-');
            }
        } else if requires_seperator(char_with_index, first_character, &input) {
            first_character = false;
            result = snake_like_with_seperator(result, &char_with_index.1);
        } else {
            first_character = false;
            result = snake_like_no_seperator(result, &char_with_index.1);
        }
    }
    result
}

fn char_is_seperator(character: &char) -> bool {
    is_not_alphanumeric(*character)
}

fn trim_right(convertable_string: &str) -> &str {
    convertable_string.trim_end_matches(is_not_alphanumeric)
}

fn is_not_alphanumeric(character: char) -> bool {
    !character.is_alphanumeric()
}

fn char_is_uppercase(test_char: char) -> bool {
    test_char == test_char.to_ascii_uppercase()
}

fn next_or_previous_char_is_lowercase(convertable_string: &str, char_with_index: usize) -> bool {
    convertable_string
        .chars()
        .nth(char_with_index + 1)
        .unwrap_or('A')
        .is_lowercase()
        || convertable_string
            .chars()
            .nth(char_with_index - 1)
            .unwrap_or('A')
            .is_lowercase()
}

#[inline]
fn requires_seperator(
    char_with_index: (usize, char),
    first_character: bool,
    convertable_string: &str,
) -> bool {
    !first_character
        && char_is_uppercase(char_with_index.1)
        && next_or_previous_char_is_lowercase(convertable_string, char_with_index.0)
}

#[inline]
fn snake_like_no_seperator(mut accumlator: String, current_char: &char) -> String {
    accumlator.push(current_char.to_ascii_lowercase());
    accumlator
}

#[inline]
fn snake_like_with_seperator(mut accumlator: String, current_char: &char) -> String {
    accumlator.push('-');
    accumlator.push(current_char.to_ascii_lowercase());
    accumlator
}
