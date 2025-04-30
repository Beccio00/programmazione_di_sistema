// WARNING: 
// - the lifetimes are not set correctly, you have to set them to make it compile
// - you have also to implemment missing functions and fix the code
// - *** see test test functions in the code for usage examples 

use std::{fs::{read, File}, io};
use std::fs;
use regex::Regex;

// (1) LineEditor: implement functionality
pub struct LineEditor {
    lines: Vec<String>,
}

impl LineEditor {
    pub fn new(s: String) -> Self {
        let first_lines = s.lines()     
            .map(|line| line.trim_end())     // removes trailing spaces and '\n'
            .map(|line| line.to_string())  
            .collect();

        Self { lines: first_lines }
    }

    // create a new LineEditor from a file
    pub fn from_file(file_name: &str) -> Result<Self, io::Error> {
        let text_file = fs::read_to_string(file_name)?;
        
        Ok(Self::new(text_file))
    }

    pub fn all_lines(&self) -> Vec<&str> {
        self.lines.iter().map(|l| l.as_str()).collect()
    }

    pub fn replace(&mut self, line: usize, start: usize, end: usize, subst: &str) {
        if let Some(l) = self.lines.get_mutz(line) {
            let modified_line = format!("{}{}{}", &l[0..start], subst, &l[end..]);
            *l = modified_line;
        }    
    }
}



// (2) Match contains the information about the match. Fix the lifetimes
// repl will contain the replacement.
// It is an Option because it may be not set yet or it may be skipped 
struct Match<'a> {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub text: &'a str,
    pub repl: Option<String>,
}

// use the crate "regex" to find the pattern and its method find_iter for iterating over the matches
// modify if necessary, this is just an example for using a regex to find a pattern
fn find_example<'a>(lines: &'a Vec<&'a str>, pattern: &'a str) -> Vec<Match<'a>> {
    let mut matches = Vec::new();
    let re = regex::Regex::new(pattern).unwrap();
    for (line_idx, line) in lines.iter().enumerate() {
        for mat in re.find_iter(line) {
            matches.push(Match {
                line: line_idx,
                start: mat.start(),
                end: mat.end(),
                text: &line[mat.start()..mat.end()],
                repl: None,
            });
        }
    }
    matches
}

// (3) Fix the lifetimes of the FindReplace struct
// (4) implement the Finder struct
struct FindReplace <'a> {
    lines: Vec<&'a str>,
    pattern: String,
    matches: Vec<Match<'a>>,
}

impl<'a> FindReplace<'a>{
    
    pub fn new(lines:  Vec<&'a str> , pattern: &str) -> Self {
        let mut finder = Self { 
            lines: lines, 
            pattern: pattern.to_string(),
            matches: Vec::new(),
        };
        finder.find_matches();

        finder
    }

    fn find_matches(&mut self) {
        let re = regex::Regex::new(&self.pattern).unwrap();
        for (line_idx, line) in self.lines.iter().enumerate() {
            for mat in re.find_iter(line) {
                self.matches.push(Match {
                    line: line_idx,
                    start: mat.start(),
                    end: mat.end(),
                    text: &line[mat.start()..mat.end()],
                    repl: None,
                });
            }
        }
    }

    // return all the matches
    pub fn matches(&self) -> &Vec<Match> {
        &self.matches
    }

    // apply a function to all matches and allow to accept them and set the repl
    // useful for promptig the user for a replacement
    pub fn apply(&mut self, fun: impl Fn(&mut Match) -> bool) {
        self.matches.iter_mut()
            .for_each(|match_item| {
                let accept = fun(match_item);

                if !accept {
                    match_item.repl = None;
                }
            });
    }
}


//(5) how FindReplace should work together with the LineEditor in order
// to replace the matches in the text
#[test]
fn test_find_replace() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = FindReplace::new(lines, "ll");

    // find all the matches and accept them 
    finder.apply(|m| {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
        m.repl = Some("some repl".to_string());
        true
    });

    // now let's replace the matches
    // // why this loop won't work? Because editor it was borrowed to lines
    // for  m in finder.matches() {
    //     editor.replace(m.line, m.start, m.end, "This is the substituition");
    // }    

    // alternate method: why this one works? 

    let mut subs = Vec::new();
    for m in finder.matches() {
        if let Some(repl) = &m.repl {
            subs.push((m.line, m.start, m.end, repl.clone()));
        }
    }


    for (line, start, end, subst) in subs {
       editor.replace(line, start, end, &subst);
    }

}


// (6) sometimes it's very expensive to find all the matches at once before applying 
// the changes
// we can implement a lazy finder that finds just the next match and returns it
// each call to next() will return the next match
// this is a naive implementation of an Iterarator

#[derive(Debug, Clone, Copy)]
struct FinderPos {
    pub line: usize,
    pub offset: usize,
}

struct LazyFinder <'a> {
    lines: Vec<&'a str>,
    pattern: String,
    pos: Option<FinderPos>,
}

impl <'a> LazyFinder <'a> {
    pub fn new(lines: Vec<&'a str>, pattern: &str) -> Self {
        Self { lines: lines, pattern: pattern.to_string(), pos: None }
    }

    pub fn next(&mut self) -> Option<Match<'a>> {
        // remember:
        // return None if there are no more matches
        // return Some(Match) if there is a match
        // each time save the position of the match for the next call
        let re = Regex::new(&self.pattern).unwrap();
        let mut start_line = 0;
        let mut offset = 0;

        if let Some(pos) = self.pos {
            start_line = pos.line;
            offset = pos.offset;
        }

        for line_idx in start_line..self.lines.len() {
            let line = self.lines[line_idx];

            let search_start = if line_idx == start_line { offset } else { 0 };
            if search_start >= line.len() {
                continue;
            }

            let sub_line = &line[search_start..];
            if let Some(mat) = re.find(sub_line) {
                let start = mat.start() + search_start;
                let end = mat.end() + search_start;

                // Save new position for the next call
                self.pos = Some(FinderPos {
                    line: line_idx,
                    offset: end,
                });

                return Some(Match {
                    line: line_idx,
                    start,
                    end,
                    text: &line[start..end],
                    repl: None,
                });
            }
        }
        None
    }
}

// (7) example of how to use the LazyFinder
#[test]
fn test_lazy_finder() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = LazyFinder::new(lines, "ll");

    // find all the matches and accept them 
    while let Some(m) = finder.next() {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
    }
}


// (8) now you have everything you need to implement the real Iterator

struct FindIter <'a>{
    lines: Vec<&'a str>,
    pattern: String,
    finder: LazyFinder<'a>
}

impl <'a> FindIter <'a> {
    pub fn new(lines: Vec<&'a str>, pattern: &str) -> Self {
        Self { lines: lines.clone(), pattern: pattern.to_string(), finder: LazyFinder::new(lines, pattern) }
    }
}

impl <'a> Iterator for  FindIter <'a> {
    type Item = Match<'a>; // <== we inform the Iterator that we return a Match

    fn next(&mut self) -> Option<Self::Item> {
        self.finder.next()
    }
}

// (9) test the find iterator
#[test]
fn test_find_iter() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = FindIter::new(lines, "ll");

    // find all the matches and accept them 
    for m in finder {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
    
    }
}

