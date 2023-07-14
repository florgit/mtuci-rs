/*
----> ЗАДАНИЕ 1 "Поиск слова в строке"

Вывести номер строки в котором встречается нужное слово и саму строку в формате:
2: строка...

 */

const SEARCH_TERM: &str = "picture";
const QUOTE: &str = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";


fn main() {
    println!("{}", find_term(SEARCH_TERM, QUOTE))
}

fn find_term(search_term: &str, quote: &str) -> String {
    let mut line_num = 1;
    let mut result_line = "";
    for line in quote.split('\n'){
        let word_index = line.find(search_term);
        if let Some(_) = word_index{
            result_line = line;
            return format!("{}: {}", line_num, result_line);
        }
        line_num += 1;
    }
    String::from("There is no such word in the quote")
}

// ----> TESTS
#[cfg(test)]
mod tests {
    use crate::find_term;
    use crate::{SEARCH_TERM, QUOTE};

    #[test]
    fn correct_line() {
        let answer = find_term(SEARCH_TERM, QUOTE);

        assert_eq!("2: dark square is a picture feverishly turned--in search of what?", answer)
    }
}
