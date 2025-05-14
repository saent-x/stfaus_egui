use std::collections::HashSet;
use rand::{distr::Alphanumeric, rng, Rng};

use crate::models::Song;

pub fn generate_rand_id() -> String {
    let rand_string: String = rng().sample_iter(&Alphanumeric).take(5).map(char::from).collect();

    rand_string
}

pub fn remove_duplicates(arr: &mut Vec<Song>) -> usize {
    let mut hs = HashSet::new();
    let initial_len = arr.len();
    
    println!("arr len b4: {:?}", arr.len()); // to be removed
    
    arr.retain(|v| {
        let flag = hs.contains(&(v.title.clone(), v.album.clone()));
        hs.insert((v.title.clone(), v.album.clone()));
        
        // log duplicated: to be removed
        if flag { println!("duplicate: {} - {}", v.title, v.album);}
        
        !flag
    });
    
    println!("hs len: {:?}", hs.len()); // // to be removed
    println!("arr new len: {:?}", arr.len()); // // to be removed
    
    initial_len - arr.len()
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn test_llmresult_vector_duplicates() {
        let mut arr = vec![
            Song {
                title: "afro".to_string(),
                artist: "fela".to_string(),
                album: "1999 Hella".to_string(),
                uuid: "uuid".to_string(),
                cover_art: "art".to_string(),
                preview_url: "url".to_string(),
            },
            Song {
                title: "aa".to_string(),
                artist: "artist".to_string(),
                album: "1 Hella".to_string(),
                uuid: "uuid".to_string(),
                cover_art: "art".to_string(),
                preview_url: "url".to_string(),
            },
            Song {
                title: "afro".to_string(),
                artist: "fela".to_string(),
                album: "1999 Hella".to_string(),
                uuid: "uuid".to_string(),
                cover_art: "art".to_string(),
                preview_url: "url".to_string(),
            },
            
        ];
        
        remove_duplicates(&mut arr);
        
        assert_eq!(arr.len(), 2);
    }
}
