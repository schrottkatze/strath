use lazy_static::lazy_static;
use fancy_regex::Regex;

pub fn preprocess(raw_code: String) -> String {
    lazy_static! {
        static ref COMMENT_RE: Regex = Regex::new(r"(\]\].*?\[\[|&&.*\n|;.*\n|``(.|\n)*?´´)").unwrap();
    }

    let code = COMMENT_RE.replace_all(&raw_code, "");
    let code = remove_empty_lines(code.to_string());
    code
}

pub fn remove_empty_lines(text: String) -> String {
    let text: Vec<&str> = text.split('\n').collect();

    let mut out = String::new();
    for line in text {
        if !line.trim().is_empty() {
            out += line;
            out.push('\n');
        }
    }

    out
}

#[test]
fn test_preprocessor() {
    let code = preprocess("
&& line comment test


¡<= ]] inline comment example [[ /\"inline comments work\"\\!

``
   big ass block comment!
   ye boiiiii
´´

&& declare variable out as string
¡}}string{{ out!

&& take input and take it into var
¡=> /out\\!

&& log out
¡<= /out\\!

        ".to_string());

    println!("{:?}", code);

    assert_eq!(code, "\
¡<=  /\"inline comments work\"\\!
¡}}string{{ out!
¡=> /out\\!
¡<= /out\\!\n");
}
