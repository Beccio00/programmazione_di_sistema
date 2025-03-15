

use clap::Parser;

#[derive(Parser, Debug)]
struct Args{
    slug_in: String,

    #[arg(short, long, default_value_t = 1)]
    repeat: u8,

    #[arg(short, long)]
    verbose: bool,
}


fn slugify(s: &str) -> String {
    let mut slug = String::new();
    let mut prevous_dash = false;

    if s.chars().count() == 1 && conv(s.chars().next().unwrap()) == '-'{
        slug.push('-');
        return slug;
    }

    for c in s.chars(){
        let new_c = conv(c);
        
        if new_c.is_alphabetic() || c.is_digit(10){
            slug.push(conv(c));
            prevous_dash = false;
        } 
        else if new_c == '-' {
           
            if prevous_dash == false {
                slug.push('-');
            }
            prevous_dash = true;           
        }
    }
    
    if slug.ends_with('-') && slug.len() > 1 {
        slug.pop();
        
    }

    slug
}


fn conv(c: char) -> char {
    let lower_c = c.to_lowercase().next().unwrap_or(c); //becouse to_lowercase return the iterator
   
    let subs_i: Vec<char> = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż".chars().collect();
    let subs_o: Vec<char> = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz".chars().collect();

    if lower_c.is_ascii_alphanumeric() {
        return lower_c;
    }

    if let Some(pos) = subs_i.iter().position(|&x| x == lower_c){
        return subs_o[pos];
    }

    '-'  
}



fn run_program() {
    let args = Args::parse();

    println!("{}", slugify(&args.slug_in));
}


#[cfg(test)]
mod tests
{
    
    use super::*;
    
    #[test]
    fn test_conversion_accented_letter() {
        let c = 'à';
        assert_eq!('a', conv(c));
    }

    #[test]
    fn test_conversion_unaccented_letter() {
        let c = 'a';
        assert_eq!('a', conv(c));
    }

    #[test]
    fn test_unknow_letter() {
        let c = '£';
        assert_eq!('-', conv(c));
    }

    #[test]
    fn test_unknow_accented_letter() {
        let c = 'ῶ';
        assert_eq!('-', conv(c));
    }

    #[test]
    fn test_string_with_spaces() {
        let s = "hello world !";
        assert_eq!("hello-world", slugify(s));
    }
    #[test]
    fn test_string_with_accents() {
        let s = "héllő-wôrlđ-!";
        assert_eq!("hello-world", slugify(s));

    }

    #[test]
    fn test_empty_string() {
        let s: &str =  &String::new();
        assert_eq!("", slugify(s));
    }

    #[test]
    fn test_mul_spaces_string() {
        let s = "h e  l   l     o       w       o        r        l         d";
        assert_eq!("h-e-l-l-o-w-o-r-l-d", slugify(s));
    }

    #[test]
    fn test_serial_special_characters() {
        let s = "hello&%$$@@@@world!";
        assert_eq!("hello-world", slugify(s));
    }

    #[test]
    fn test_only_special_characters() {
        let s = "@#[*+§#%$£=|&;:._";
        assert_eq!("-", slugify(s));
    }

    #[test]
    fn  test_string_end_space() {
        let s = "hello world ! ";
        assert_eq!("hello-world", slugify(s));
    }

    #[test]
    fn test_string_end_characters() {
        let s = "hello-world !?°@!=";
        assert_eq!("hello-world", slugify(s));
    }

}

fn main() {
    run_program();
}


