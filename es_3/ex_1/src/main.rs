use clap::Parser;

const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";


/*#1 solution */
 
trait MySlug {
    fn is_slug(&self) -> bool;
    fn to_slug(&self) -> String;
}
/* 
impl MySlug for String {
    fn is_slug(&self) -> bool {
        let test = slugify(self);
        
        test.eq(self)
    }

    fn to_slug(&self) -> String {
        slugify(self)        
    }
}

impl MySlug for &str {
    fn is_slug(&self) -> bool {
        let test = slugify(self);

        test.eq(self)
    }

    fn to_slug(&self) -> String {
        slugify(self)
    }
}
*/

impl<T> MySlug for T 
where T: AsRef<str>{
    fn is_slug(&self) -> bool {
        let test = slugify(self.as_ref());
        
        test.eq(self.as_ref())
    }

    fn to_slug(&self) -> String {
        slugify(self.as_ref())        
    }
}



// with ieratators (not used, supplied for example)
fn _conv_alt(c: char) -> char {
    match c {
        'a'..='z' | '0'..='9' => c,
        _ => match SUBS_I.chars().position(|x|x == c ) {
            Some(i) => SUBS_O.chars().nth(i).unwrap(),
            None => '-',
        }
    }
}

// with for loop
fn  conv(c: char) -> char {
    match c {
        // use match syntax to match a range of characters
        'a'..='z' | '0'..='9' => c,
        _ => {
            // we can either ierate over the characters of SUBS_I or convert it to a vector of chars
            for (i, x) in SUBS_I.chars().enumerate() {
                if x == c {
                    // we can't index a string but we can index a slice of bytes
                    // in SUBS_O all chars are 1 byte long, therefore we can use as_bytes()
                    // and get the i-th byte as a char
                    return SUBS_O.as_bytes()[i] as char;
                }
            }
            '-'
        }
    }
}

fn slugify(s: &str) -> String {
    let mut slug = String::new();

    for c in s.chars() {
        // converting a char to lowercase may lead to one or more chars
        // we need to to loop on them
        // we could have used c.to_lowercase().chars() but it could be slightly less efficient 

        for c in c.to_lowercase() {
            let c = conv(c);
            if c != '-' {
                slug.push(c);
            } else if !slug.ends_with('-') {
                slug.push('-');
            }
        }
    }
    if slug.ends_with('-') && slug.len() > 1 {
        slug.pop();
    }
    slug
}



#[derive(Debug, Parser)]
struct Args {
    input: String,

    #[arg(short, long, default_value="false")]
    verbose: bool,

    #[arg(short, long)]
    repeat: Option<u32>
}


fn main() {

    let args: Args = Args::parse();

    let res =  slugify(&args.input);
    
    match args.repeat {
        Some(n) => {
            for i in 0..n {
                if args.verbose {
                    println!("[{}] {} => {}", i, args.input, res);
                } else { 
                    println!("{}", res);
                }
            }
        },
        None => if args.verbose {
            println!("{} => {}", args.input, res)
        } else {
            println!("{}", res)
        }
    }

    let s1 = String::from("Hello String");
    let s2 = "hello-slice";
    println!("{}", s1.is_slug()); // false
    println!("{}", s2.is_slug()); // true
    let s3: String = s1.to_slug();
    let s4: String = s2.to_slug();
    println!("s3:{} s4:{}", s3, s4); // stampa: s3:hello-string s4:hello-slice

}

#[cfg(test)]
mod tests {

    #[test]
    fn single_char_with_accent() {
        assert_eq!(super::slugify(&"ò"), "o");
    }

    #[test]
    fn single_char_without_accent() {
        assert_eq!(super::slugify(&"x"), "x");
    }

    #[test]
    fn single_invalid_char() {
        assert_eq!(super::slugify(&"/"), "-");
    }

    #[test]
    fn single_unknown_accent() {
        assert_eq!(super::slugify(&"ῶ"), "-");
    }

    #[test]
    fn multiple_words() {
        assert_eq!(super::slugify(&"Hello World"), "hello-world");
    }
    #[test]
    fn multiple_words_with_accents() {
        assert_eq!(super::slugify(&"Così è uno slug!"), "cosi-e-uno-slug");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(super::slugify(&""), "");
    }

    #[test]
    fn multiple_spaces() {
        assert_eq!(super::slugify(&"Hello   World!!"), "hello-world");
    }

    #[test]
    fn only_special_chars() {
        assert_eq!(super::slugify(&"!@#$%^&*()"),"-");
    }

    #[test]
    fn multiple_spaces_at_the_end() {
        assert_eq!(super::slugify(&"Hello  World!!  "), "hello-world");
    }

    #[test]
    fn multiple_invalid_chars_at_the_end() {
        assert_eq!(super::slugify(&"Hello  World/\\"), "hello-world");
    }
}
