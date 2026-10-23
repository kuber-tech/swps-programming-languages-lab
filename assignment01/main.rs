//Task 1 (fn double_in_place)

fn double_in_place(values: &mut [i32]){
    for number in values {
        *number = *number * 2;
    }
}

//Task 2 (fn largest)

fn largest(values: &[i32]) -> Option<i32> {
    if values.len() == 0 {
        return None;
    }
let mut biggest = values[0];
for number in values {
    if *number > biggest {
        biggest = *number;
    }
}
Some(biggest)
}

//

struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    top_left: Point,
    width: f64,
    height: f64,
}

impl Rectangle {

//Task 3 (fn area)

fn area(&self) -> f64 {
    self.width * self.height
}

//Task 4 (fn contains)

fn contains(&self, p: Point) -> bool {
    p.x >= self.top_left.x
        && p.x <= self.top_left.x + self.width
        && p.y >= self.top_left.y
        && p.y <= self.top_left.y + self.height
}

//Task 5 (fn can_hold)

fn can_hold(&self, other: &Rectangle) -> bool {
    self.width >= other.width && self.height >= other.height
}

//Task 6 (fn scale)

fn scale(&mut self, factor: f64) {
    self.width = self.width * factor;
    self.height = self.height * factor;
}
}

//

struct Book {
    title: String,
    author: String,
    year: u16,
}

impl Book{

//Task 7 (Book::new)

fn new(title: &str, author: &str, year: u16) -> Self {
    Book {
        title: title.to_string(),
        author: author.to_string(),
        year: year,
    }
}
}

//Task 8 (Library::new)

struct Library {
    books: Vec<Book>,
}

impl Library{

fn new() -> Self {
    Library {books: Vec::new()}
}

//Task 9 (fn add)

fn add(&mut self, book: Book) {
    self.books.push(book);
}

//Task 10 (fn titles)

fn titles(&self) -> Vec<&str> {
    let mut result = Vec::new();
    
    for book in &self.books{
        result.push(book.title.as_str());
    }
    result
}
}



fn main(){
    let mut numbers = vec![1, 2, 3];
    double_in_place(&mut numbers);
    println!("Doubled: {:?}", numbers);
    println!("Largest: {:?}", largest(&numbers));
    println!("Empty: {:?}", largest(&[]));

    let mut rect = Rectangle {
        top_left: Point {x: 0.0, y: 0.0},
        width: 6.0,
        height: 7.0,
    };
    let small = Rectangle {
        top_left: Point {x: 10.0, y: 20.0},
        width: 3.0,
        height: 2.0,
    };
    println!("Area: {}", rect.area());
    println!("Contains point: {}", rect.contains(Point{x: 2.0, y: 2.0}));
    println!("Can hold: {}", rect.can_hold(&small));
    rect.scale(2.0);
    println!("Scaled width: {}, height: {}", rect.width, rect.height);

    let mut library = Library::new();
    let first_book = Book::new("Dune", "Frank Herbert", 1965);
    let second_book = Book::new("The Hunger Games", "Suzanne Collins", 2008);
    library.add(first_book);
    library.add(second_book);
    println!("Titles: {:?}", library.titles());
}