#[derive(Debug,PartialEq,Clone)]
struct Book {
    title: String,
    author: String,
    publication_year: u16,

    genre: Genre,
}
#[derive(Debug,PartialEq,Clone)]
enum Genre {
    Fiction,
    NonFiction,

    Mystery,
    SciFi,
}

fn create_book(
    title: String,
    author: String,
    publication_year: u16,
    genre: Genre,
) -> Result<Book, &'static str> {
    if 
    !title.is_empty() 
            &&
    !author.is_empty() {
        Ok(Book {
            title,
            author,
            publication_year,
            genre,
        })
    } else {
        Err("Fill all fields. Empty fields not allowed")
    }
}

fn find_book_by_genre(books: &Vec<Book>,genre: Genre) -> (usize,Option<&Book>) {
    for (i,book) in books.into_iter().enumerate() {
        if book.genre == genre {
            return (i,Some(&book));
        }
    }
    (0, None)
}
fn main() {
    let res = create_book(
        String::from("The C++ Programming Language"),
        String::from("Bjarne Stroustrup"),
        1990,
        Genre::NonFiction,
    )
    .unwrap();
    //println!("{res:?}");

    let res2 = create_book(
        String::from("The Go Programming Language"),
        String::from("Google"),
        1990,
        Genre::SciFi,
    ).unwrap();
    let mut books_container: Vec<Book> = vec![res,res2];
    let check = find_book_by_genre(&books_container, Genre::SciFi);
    println!("Found result: {}:{:?}", check.0, check.1);
}



mod tests {
    use super::*;

    #[cfg(test)]
    #[test]
    fn test_create_book_failure_empty_fields() {
        let result = create_book(
            String::from(""),
            String::from("Bjarne Stroustrup"),
            1990,
            Genre::NonFiction,
        );

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Fill all fields. Empty fields not allowed"
        );
    }
    #[test]
    fn test_find_book_by_genre() {
        let result = create_book(
            String::from("The Go Programming Language"),
            String::from("Google"),
            1990,
            Genre::SciFi,
        ).unwrap();
        let books_container: Vec<Book> = vec![result];
        let (index,result) = find_book_by_genre(&books_container, Genre::NonFiction);
        assert_eq!(index, 0);
        assert_eq!(result, None);
    }
    #[test]
    fn test_find_book_by_genre_found() {
        let book1 = Book {
            title: String::from("The C++ Programming Language"),
            author: String::from("Bjarne Stroustrup"),
            publication_year: 1990,
            genre: Genre::NonFiction,
        };

        let book2 = Book {
            title: String::from("The Go Programming Language"),
            author: String::from("Google"),
            publication_year: 1990,
            genre: Genre::SciFi,
        };

        let books_container: Vec<Book> = vec![book1.clone(), book2.clone()];

        let (index, result) = find_book_by_genre(&books_container, Genre::SciFi);

        assert_eq!(index, 1);
        assert_eq!(result, Some(&book2));
    }


}
