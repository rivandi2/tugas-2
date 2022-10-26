use chrono::prelude::*;
use chrono::DateTime;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io;

#[derive(Debug, Clone)]
enum Genre {
    Action,
    Comedy,
    Romance,
    Horror,
    Drama
}

impl Genre {
    fn from_str(genre: &str) -> Result<Genre, String> {
        match genre {
            "Action" => Ok(Genre::Action),
            "Comedy" => Ok(Genre::Comedy),
            "Romance" => Ok(Genre::Romance),
            "Horror" => Ok(Genre::Horror),
            "Drama" => Ok(Genre::Drama),
            _ => Err("Genre not found!".to_string())
        }
    }
}

#[derive(Debug, Clone)]
struct Movie {
    name: String,
    genre: Genre,
    duration: i32,
    time_rented: DateTime<Local>
}

struct Movies {
    inner: HashMap<String, Movie>
}
impl Movies {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, movie: Movie) {
        self.inner.insert(movie.name.clone(), movie);
    }

    fn get_all(&self) -> Vec<&Movie> {
        let mut movies = vec![];
        for movie in self.inner.values() {
            movies.push(movie)
        }
        movies
    }

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    fn update(&mut self, name: &str, duration: i32) -> bool {
        match self.inner.get_mut(name) {
            Some(movie) => {
                movie.duration = duration;
                movie.time_rented = Local::now();
                true
            },
            None => false
        }
    }
}

fn get_input() -> Option<String> {
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).unwrap();

    let input = temp.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_duration() -> Option<i32> {
    loop {
        print!("Duration (days): ");
        std::io::stdout().flush().unwrap();

        let input = match get_input() {
            Some(input) => input,
            None => return None
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result<i32, _> = input.parse();
        match parsed_input {
            Ok(duration) => return Some(duration),
            Err(_) => println!("Please enter a number!")
        }
    }
}

fn get_genre() -> Option<Genre> {
    loop {
        print!("Genre (Action, Comedy, Romance, Horror, Drama): ");
        std::io::stdout().flush().unwrap();

        let input = match get_input() {
            Some(input) => input,
            None => return None
        };
        if &input == "" {
            return None;
        }
        if &input == "Action" || &input == "Comedy" || &input == "Romance" || &input == "Drama" || &input == "Horror"{
            let genre = Genre::from_str(input.trim()).unwrap();
            return Some(genre)
        } else {
            println!("Please enter a genre!")
        }
        
    }
}

fn rent_movie_menu(movies: &mut Movies) {
    print!("Movie name: ");
    std::io::stdout().flush().unwrap();

    let name = match get_input() {
        Some(input) => input,
        None => return
    };

    let genre = match get_genre() {
        Some(genre) => genre,
        None => return
    };

    let duration = match get_duration() {
        Some(duration) => duration,
        None => return
    };

    let time_rented = Local::now();

    let movie = Movie { name, genre, duration, time_rented };
    movies.add(movie);
    println!("Movie rented!");
}

fn remove_movie_menu(movies: &mut Movies) {
    print!("Enter movie to remove: ");
    std::io::stdout().flush().unwrap();

    let name = match get_input() {
        Some(input) => input,
        None => return
    };
    if movies.remove(&name) {
        println!("Movie removed!")
    } else {
        println!("Movie not found!")
    }
}

fn update_movie_menu(movies: &mut Movies) {
    print!("Enter movie to update: ");
    std::io::stdout().flush().unwrap();

    let name = match get_input() {
        Some(input) => input,
        None => return
    };
    let duration  = match get_duration() {
        Some(duration) => duration,
        None => return
    };
    if movies.update(&name, duration) {
        println!("Movie Rent Updated!")
    } else {
        println!("Movie not found!")
    }
}

fn view_rent_menu(movies: &Movies) {
    for movie in movies.get_all() {
        println!("{:?}", movie);
    }
}

fn menu() {
    fn show() {
        println!("");
        println!("Movie Rental");
        println!("=============");
        println!("1. Rent Movie");
        println!("2. View Movie Rent");
        println!("3. Remove Movie Rental");
        println!("4. Update Movie Rental");
        println!("5. Exit");
        print!("Input: ");
        std::io::stdout().flush().unwrap();
    }

    let mut movies = Movies::new();

    loop {
        show();
        let input = match get_input() {
            Some(input) => input,
            None => return
        };
        match input.as_str() {
            "1" => rent_movie_menu(&mut movies),
            "2" => view_rent_menu(&movies),
            "3" => remove_movie_menu(&mut movies),
            "4" => update_movie_menu(&mut movies),
            "5" => break,
            _ => println!("Please input between 1-5!")
        }
    }
}


fn main() {
    menu()
}
