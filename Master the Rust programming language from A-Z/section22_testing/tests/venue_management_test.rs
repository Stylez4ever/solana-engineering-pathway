use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};

use section22_testing::attractions::{MovieTheater, Museum};
use section22_testing::management::VenueManagement;

#[fixture]
fn museum_with_three_paintings() -> Museum {
    let mut museum = Museum::new();
    museum.buy_painting("Goku spirit boom");
    museum.buy_painting("Vegita turning super");
    museum.buy_painting("Gohun one hand kamekamea");
    museum
}

#[fixture]
fn museum_management(museum_with_three_paintings: Museum) -> VenueManagement<Museum> {
    VenueManagement::new(museum_with_three_paintings)
}

#[fixture]
fn movie_theater_with_one_movie() -> MovieTheater {
    let mut movie_theater = MovieTheater::new();
    movie_theater.add_movies("The last airbender");
    movie_theater
}

#[fixture]
fn movie_theater_management(movie_theater_with_one_movie: MovieTheater) -> VenueManagement<MovieTheater> {
    VenueManagement::new(movie_theater_with_one_movie)
}

#[rstest]
fn venue_managemnt_interacts_with_venue(museum_with_three_paintings: Museum) {
    let mut venue_mgmt = VenueManagement::new(museum_with_three_paintings);
    venue_mgmt.make_money();

    assert_eq!(venue_mgmt.venue.revenue, 25);
    assert_eq!(venue_mgmt.venue.painting.len(), 3);
}

#[rstest]
fn venue_management_interacts_with_movie_theater_venue(mut movie_theater_management: VenueManagement<MovieTheater>) {
    movie_theater_management.make_money();
    assert_eq!(movie_theater_management.venue.sales, 15)
}

