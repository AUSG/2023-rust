
struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // 아래는 Book 구조체의 생성자입니다.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// 이 구현메서드는 {} 포맷으로 출력할 수 있게 도와줍니다.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

impl Library {
    fn new() -> Library {
        Library {
            books: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.len() == 0
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn print_books(&self) {
        for book in &self.books {
            println!("{book}");
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        let mut oldest: Option<&Book> = None;
        for book in &self.books {
            match oldest {
                Some(oldest_book) => {
                    if oldest_book.year > book.year {
                        oldest = Some(&book);
                    }
                }
                None => oldest = Some(&book),
            }
        }
        oldest
    }
}

// 아래 소스 주석을 제거하고 누락된 메서드를 구현하세요
// 미구현 메서드도 존재하므로 메서드 시그니처를 업데이트 하세요(self 포함)
fn main() {
    let mut library = Library::new();

    println!("Our library is empty: {}", library.is_empty());

    let favorite_book = Book::new("Lord of the Rings", 1954);
    println!("Our favorite book {favorite_book} should go in the library");
    library.add_book(favorite_book);
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    library.print_books();

    match library.oldest_book() {
       Some(book) => println!("My oldest book is {book}"),
       None => println!("My library is empty!"),
    }

    println!("Our library has {} books", library.len());
    for book in library.books {
        println!("{book}");
    }
}
