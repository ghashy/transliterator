use rand::Rng;
use unicode_segmentation::UnicodeSegmentation;

fn transliterate(text: &str) -> String {
    let mut result = String::new();
    let lower = text.to_lowercase();
    let mut chars = text.graphemes(true).zip(lower.graphemes(true));

    while let Some((c, lower)) = chars.next() {
        let is_upper = c.chars().next().unwrap().is_uppercase();
        match lower {
            "й" => result.push(if is_upper { 'Y' } else { 'y' }),
            "й" => result.push(if is_upper { 'Y' } else { 'y' }),
            "ц" => result.push_str(if is_upper { "TS" } else { "ts" }),
            "у" => result.push(if is_upper { 'U' } else { 'u' }),
            "к" => result.push(if is_upper { 'K' } else { 'k' }),
            "е" => result.push(if is_upper { 'E' } else { 'e' }),
            "н" => result.push(if is_upper { 'N' } else { 'n' }),
            "г" => result.push(if is_upper { 'G' } else { 'g' }),
            "ш" => result.push_str(if is_upper { "SH" } else { "sh" }),
            "щ" => result.push_str(if is_upper { "SCH" } else { "sch" }),
            "з" => result.push(if is_upper { 'Z' } else { 'z' }),
            "х" => result.push(if is_upper { 'H' } else { 'h' }),
            "ъ" => result.push(if is_upper { '\'' } else { '\'' }),
            "ф" => result.push(if is_upper { 'F' } else { 'f' }),
            "ы" => result.push(if is_upper { 'Y' } else { 'y' }),
            "в" => result.push(if is_upper { 'V' } else { 'v' }),
            "а" => result.push(if is_upper { 'A' } else { 'a' }),
            "п" => result.push(if is_upper { 'P' } else { 'p' }),
            "р" => result.push(if is_upper { 'R' } else { 'r' }),
            "о" => result.push(if is_upper { 'O' } else { 'o' }),
            "л" => result.push(if is_upper { 'L' } else { 'l' }),
            "д" => result.push(if is_upper { 'D' } else { 'd' }),
            "ж" => result.push_str(if is_upper { "ZH" } else { "zh" }),
            "э" => result.push(if is_upper { 'E' } else { 'e' }),
            "я" => result.push_str(if is_upper { "YA" } else { "ya" }),
            "ч" => result.push_str(if is_upper { "CH" } else { "ch" }),
            "с" => result.push(if is_upper { 'S' } else { 's' }),
            "м" => result.push(if is_upper { 'M' } else { 'm' }),
            "и" => result.push(if is_upper { 'I' } else { 'i' }),
            "т" => result.push(if is_upper { 'T' } else { 't' }),
            "ь" => result.push(if is_upper { '\'' } else { '\'' }),
            "б" => result.push(if is_upper { 'B' } else { 'b' }),
            "ю" => result.push_str(if is_upper { "YU" } else { "yu" }),
            _ => {
                if c.len() == 1 {
                    let ch = c.chars().next().unwrap();
                    if ch.is_ascii() {
                        result.push(ch);
                    } else {
                        result.push('_');
                    }
                }
            }
        }
    }

    result
}

fn main() {
    let result = transliterate("Миша - Май demo rec 2.mp3");
    println!("{result}");
}

fn random_latin_letter() -> char {
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..52);
    letters.chars().nth(index).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transliterate_basic() {
        assert_eq!(transliterate("Миша"), "Misha");
        assert_eq!(
            transliterate("Миша - Май demo rec 2.mp3"),
            "Misha - May demo rec 2.mp3"
        );
    }

    #[test]
    fn test_transliterate_uppercase() {
        assert_eq!(transliterate("МиША"), "MiSHA");
        assert_eq!(
            transliterate("МИША - МАЙ DEMO REC 2.MP3"),
            "MISHA - MAY DEMO REC 2.MP3"
        );
    }

    #[test]
    fn test_transliterate_special_characters() {
        assert_eq!(transliterate("Миша Я"), "Misha YA");
        assert_eq!(transliterate("Миша!"), "Misha!");
    }

    #[test]
    fn test_transliterate_non_cyrillic() {
        assert_eq!(transliterate("Hello Миша World"), "Hello Misha World");
    }

    #[test]
    fn test_transliterate_mixed_casing() {
        assert_eq!(
            transliterate("МиША - МаЙ demo rec 2.Mp3"),
            "MiSHA - MaY demo rec 2.Mp3"
        );
    }
}
