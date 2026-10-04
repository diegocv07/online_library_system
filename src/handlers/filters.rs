use crate::models::Book;

pub fn filter_by_availability(book_list: Vec<Book>, filter: Option<bool>) -> Vec<Book> {
    if let Some(available) = filter {
        book_list
            .into_iter()
            .filter(|b| b.available == available)
            .collect()
    } else {
        book_list
    }
}

pub fn filter_by_rating(book_list: Vec<Book>, filter: Option<i32>) -> Vec<Book> {
    if let Some(rating) = filter {
        book_list
            .into_iter()
            .filter(|b| b.book_rating == rating)
            .collect()
    } else {
        book_list
    }
}

pub fn filter_by_author(book_list: Vec<Book>, filter: Option<String>) -> Vec<Book> {
    if let Some(author) = filter
        && author.len() > 0
    {
        let author = author.to_ascii_lowercase().replace("+", " ");

        book_list
            .into_iter()
            .filter(|b| b.author.to_ascii_lowercase() == *author)
            .collect()
    } else {
        book_list
    }
}

pub fn filter_by_language(book_list: Vec<Book>, filter: Option<String>) -> Vec<Book> {
    if let Some(language) = filter
        && language.len() > 0
    {
        let language = language.to_ascii_lowercase().replace("+", " ");

        book_list
            .into_iter()
            .filter(|b| b.language.to_ascii_lowercase() == *language)
            .collect()
    } else {
        book_list
    }
}

pub fn filter_by_title(book_list: Vec<Book>, filter: Option<String>) -> Vec<Book> {
    if let Some(title) = filter
        && title.len() > 0
    {
        let word_filter: Vec<String> = title
            .to_ascii_lowercase()
            .split("+")
            .map(|word| word.to_owned())
            .collect();

        book_list
            .into_iter()
            .filter(|b| {
                let binding = b.title.to_ascii_lowercase();
                let title_words: usize = binding
                    .split(' ')
                    .filter(|&word| word_filter.contains(&word.to_owned().to_ascii_lowercase()))
                    .count();
                title_words == word_filter.len()
            })
            .collect()
    } else {
        book_list
    }
}

pub fn filter_by_tags(book_list: Vec<Book>, filter: Option<String>) -> Vec<Book> {
    if let Some(tags) = filter
        && tags.len() > 0
    {
        let tags: Vec<String> = tags
            .to_ascii_lowercase()
            .split(",")
            .map(|word| word.to_owned())
            .collect();

        book_list
            .into_iter()
            .filter(|b| {
                let matched_tags: usize = b
                    .tags
                    .iter()
                    .filter(|&tag| {
                        tag.is_some() && tags.contains(&tag.as_ref().unwrap().to_ascii_lowercase())
                    })
                    .count();
                matched_tags == tags.len()
            })
            .collect()
    } else {
        book_list
    }
}

pub fn filter_by_genres(book_list: Vec<Book>, filter: Option<String>) -> Vec<Book> {
    if let Some(genres) = filter
        && genres.len() > 0
    {
        let genres: Vec<String> = genres
            .to_ascii_lowercase()
            .split(",")
            .map(|word| word.to_owned())
            .collect();

        book_list
            .into_iter()
            .filter(|b| {
                let matched_genres: usize = b
                    .genres
                    .iter()
                    .filter(|&genre| {
                        genre.is_some()
                            && genres.contains(&genre.as_ref().unwrap().to_ascii_lowercase())
                    })
                    .count();
                matched_genres == genres.len()
            })
            .collect()
    } else {
        book_list
    }
}
