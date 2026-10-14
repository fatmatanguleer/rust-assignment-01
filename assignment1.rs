// PART 1 — FIX THE BORROW CHECKER

// (a) E0382: Changed `s: String` to `s: &str` to borrow instead of moving ownership.
fn print_len(s: &str) { 
    println!("{}", s.len()); 
}

fn fix_a() {
    let s = String::from("hello");
    print_len(&s);
    println!("{s}");
}

// (b) E0502: Moved `println!` before `v.push(4)` so the immutable borrow ends before the mutable borrow.
fn fix_b() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    println!("first = {first}");
    v.push(4);
}

// (f) E0505: Passed `&name` as `&str` to `shout` so ownership of `name` is not moved.
fn shout(s: &str) -> String { 
    s.to_uppercase() 
}

fn fix_f() {
    let name = String::from("ada");
    let r = &name;
    let loud = shout(&name);
    println!("{r} {loud}");
}

// (g) E0502: Collected matching items into a separate vector first to avoid simultaneous immutable iteration and mutable push.
fn fix_g() {
    let mut scores = vec![10, 20, 30];
    let to_add: Vec<i32> = scores.iter().copied().filter(|&s| s > 15).collect();
    scores.extend(to_add);
    println!("{scores:?}");
}

// (h) E0499: Evaluated `bump()` sequentially instead of creating two simultaneous mutable references.
struct Counter { value: i32 }
impl Counter {
    fn bump(&mut self) -> i32 {
        self.value += 1;
        self.value
    }
}

fn fix_h() {
    let mut c = Counter { value: 0 };
    let val_a = c.bump();
    let val_b = c.bump();
    println!("{} {}", val_a, val_b);
}

// (i) E0515: Returned an owned `String` instead of a reference to a local variable dropped at function end.
struct Wrapper { value: String }
fn unwrap_text(w: &Wrapper) -> &str {
    &w.value
}

fn broken_unwrap() -> String {
    let w = Wrapper { value: String::from("hi") };
    unwrap_text(&w).to_string()
}


// PART 2 — STRINGS AND SLICES

// 1. Returns the text up to the first space, or the whole string.
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(index) => &s[..index],
        None => s,
    }
}

// 2. Counts `aeiou`, ignoring case.
fn count_vowels(s: &str) -> usize {
    s.chars()
        .filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
}

// 3. Reverses words order using split_whitespace.
fn reverse_words(s: &str) -> String {
    s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}

// 4. Upper-cases the first character of the string.
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

// 5. Checks if palindrome ignoring case and non-alphanumeric characters.
fn is_palindrome(s: &str) -> bool {
    let cleaned: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    
    cleaned.iter().eq(cleaned.iter().rev())
}


// MAIN FUNCTION (TESTS)
fn main() {
    println!("--- Part 1 Tests ---");
    fix_a();
    fix_b();
    fix_f();
    fix_g();
    fix_h();
    println!("{}", broken_unwrap());

    println!("\n--- Part 2 Tests ---");
    println!("first_word: {}", first_word("hello world"));
    println!("count_vowels: {}", count_vowels("Hello World"));
    println!("reverse_words: {}", reverse_words("the quick brown fox"));
    println!("capitalize: {}", capitalize("rust"));
    println!("is_palindrome: {}", is_palindrome("A man, a plan, a canal: Panama"));
}
