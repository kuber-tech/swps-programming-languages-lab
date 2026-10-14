//(a) -- E0382: Borrow of moved value

//The error appeared because the first function took ownership of the string.
//That means that the main function can't reuse the string after that.
//I fixed it by adding the "&" next to the string, which makes the function borrow the string.
//The same sign "&" next to "s" in the main function makes it pass a reference to the string.

fn print_len(s: &String) { 
    println!("{}", s.len()); }
fn task_a() {
    let s = String::from("hello");
    print_len(&s);
    println!("{s}");
}


//(b) -- E0502: Cannot borrow as mutable because it is also borrowed as immutable

//The error appear because we change the vector and only after try to print.
//I fixed it by printing first, then changing the vector.

fn task_b() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    println!("first = {first}");
    v.push(4);
    println!("vector = {v:?}");
}


//(f) E0505: Cannot move out of `name` because it is borrowed

//The "shout" command takes ownership of the string, so after we use "shout" in main
//we can't print "r" since it was already used.
//I fixed it by using the borrowed reference before moving "name" into "shout" and printing the "loud" at the end. 

fn shout(s: String) -> String { 
    s.to_uppercase() }
fn task_f() {
    let name = String::from("ada");
    let r = &name;
    println! ("{r}");
    let loud = shout(name);
    println!("{loud}");
}


// (g) E0502: Cannot borrow `scores` as mutable because it is also borrowed as immutable

//The error appears because we're trying to add new elements inside the loop
//while we're reading the elements by borrowing them.
//I fixed it by adding a new vector where we can temporarily put the values
//before adding all of them into scores.

fn task_g() {
    let mut scores = vec![10, 20, 30];
    let mut new = Vec::new();
    for s in &scores {
        if *s > 15 {
            new.push(*s);
        }
    }

    for s in new {
        scores.push(s);
    }

    println!("{scores:?}");
}


// (h) E0499: Cannot borrow `c` as mutable more than once at a time

//The error appears because we have two mutable borrows of the same value overlapping.
//I fixed it by using separate blocks for each mutable reference.

struct Counter { 
    value: i32 }

impl Counter {
    fn bump(&mut self) -> i32 {
        self.value += 1;
        self.value
    }
}
fn task_h() {
    let mut c = Counter { value: 0 };
    let first = {
        let a = &mut c;
        a.bump()
    };
    let second = {
        let b = &mut c;
        b.bump()
    };
    println!("{first} {second}");
}


// (i) E0515: Cannot return value referencing local variable

//The error appears because we create a local variable in broken_unwrap
//which stops existing after the function stops running.
//But unwrap_text returns a borrowed reference to text in w, which doesn't work.
//I fixed it by returning an owned string instead of a borrowed reference. 

struct Wrapper { 
    value: String, }

fn unwrap_text(w: &Wrapper) -> &str {
    &w.value
}

fn broken_unwrap() -> String {
    let w = Wrapper { 
        value: String::from("hi") 
    };
    unwrap_text(&w).to_string()
}
fn task_i(){
    let result = broken_unwrap();
    println!("{result}");
}



//first_word

fn first_word(s: &str) -> &str {
    for (i, c) in s.char_indices() {
        if c == ' ' {
            return &s[0..i];
        }
    }
    return s;
}

//count_vowels

fn count_vowels(s: &str) -> usize {
    let mut count = 0;
    for c in s.chars(){
        let letter = c.to_ascii_lowercase();
        if "aeiou".contains(letter){
            count += 1;
        }
    }
    count
}

//reverse_words

fn reverse_words(s: &str) -> String {
    let mut result = String::new();
    for word in s.split_whitespace().rev() {
        if result != "" {
            result.push(' ');
        }
        result.push_str(word);
    }
    return result;
}

//capitalize

fn capitalize(s: &str) -> String {
    let mut result = String::new();
    let mut first = true;
    for c in s.chars() {
        if first {
            for upper in c.to_uppercase() {
                result.push(upper);
            }

            first = false;
        } else {
            result.push(c);
        }
    }
    return result;
}

fn main() {
    println!("\nTask 1");
    task_a();
    println!("\nTask 2");
    task_b();
    println!("\nTask 3");
    task_f();
    println!("\nTask 4");
    task_g();
    println!("\nTask 5");
    task_h();
    println!("\nTask 6");
    task_i();
    println!("\nTask 7: first word");
    println!("{}", first_word("hello world"));
    println!("{}", first_word("single"));
    println!("\nTask 8: count vowels");
    println!("{}", count_vowels("AbcdE"));
    println!("{}", count_vowels("hello"));
    println!("\nTask 9: reverse words");
    println!("{}", reverse_words("the quick brown fox"));
    println!("\nTask 10: capitalize");
    println!("{}", capitalize("rust"));
    println!("{}", capitalize(""));
}












