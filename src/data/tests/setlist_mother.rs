use crate::data::models::setlistfm_response_models::{
    Artist, City, Coords, Country, Set, Setlist, Sets, Song, Venue,
};

pub struct SetlistMotherObject;

impl SetlistMotherObject {
    pub fn show(event_date: &str, sets: Vec<Set>) -> Setlist {
        Setlist {
            artist: Artist {
                mbid: "artist-mbid".to_string(),
                name: "Test Artist".to_string(),
                sort_name: "Test Artist".to_string(),
                disambiguation: None,
                url: "https://example.com/artist".to_string(),
            },
            venue: Venue {
                id: "venue-id".to_string(),
                name: "Test Venue".to_string(),
                url: "https://example.com/venue".to_string(),
                city: Some(City {
                    id: Some("city-id".to_string()),
                    name: Some("Test City".to_string()),
                    state: None,
                    state_code: None,
                    country: Some(Country {
                        code: Some("US".to_string()),
                        name: Some("United States".to_string()),
                    }),
                    coords: Some(Coords {
                        lat: Some(1.0),
                        long: Some(1.0),
                    }),
                }),
            },
            tour: None,
            sets: Sets { set: sets },
            info: None,
            url: "https://example.com/setlist".to_string(),
            id: "setlist-id".to_string(),
            version_id: "version-id".to_string(),
            event_date: event_date.to_string(),
            last_updated: "01-01-2024".to_string(),
        }
    }

    pub fn set(encore: Option<u32>, songs: Vec<Song>) -> Set {
        Set {
            name: None,
            encore,
            song: songs,
        }
    }

    pub fn song(name: Option<&str>, info: Option<&str>, tape: Option<bool>) -> Song {
        Song {
            name: name.map(|value| value.to_string()),
            info: info.map(|value| value.to_string()),
            cover: None,
            tape,
        }
    }
}
