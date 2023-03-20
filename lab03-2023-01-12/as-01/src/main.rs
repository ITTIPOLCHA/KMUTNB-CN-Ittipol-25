fn main() {
    //let text = "this cat this bat this rat";
    println!("this cat this bat this rat");
    let _result = words("this cat this bat this rat");
}

fn words(text: &str) -> i32{
    let text = text.to_string();
    let mut x = 0;
    let mut z = 0;
    let mut y = 0;
    let mut st = 0;
    let mut en = 0;
    let mut word = Vec::new();
    for a in text.chars() {
        if a == 't' {
            if x % 2 == 0 {
                st = y;
            }
            else {
                en = y+1;
                let s = &text[st..en];
                word.push(s);
            }
            x += 1;
        }
        else if  a == ' '{
            if z % 2 == 0 {
                en = y;
                let s = &text[st..en];
                word.push(s);
                st = y+1;
            }
            z += 1;
        }
        y += 1;
    }
    println!("{:?}",word);
    let _result = unique(word);
    _result
}

fn unique(_word: Vec<&str>) -> i32{
    let mut w = Vec::new();
    w.push(_word[0]);
    for a in &_word {
        if a != &_word[0] {
            w.push(a);
        }

    }
    println!("{:?}",w);
    let _result = count(w);
    _result
}

fn count(w: Vec<&str>) -> i32{
    let mut _result = 0;
    for _b in w {
        _result += 1;
    }
    println!("{}",_result);
    _result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        println!("test : 4");
        assert_eq!(words("this cat this bat this rat"), 4);
    }
}