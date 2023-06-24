mod house;

use house::{get_housing, Bathroom, Community, Hall, Laundry, Location, RoomStyle};

fn main() {
    let mut housing = get_housing();

    let weights = Weights {
        bathroom_shared_on_hall: 0.0,
        bathroom_shared_per_room: 3.0,
        west_campus: 1.0,
        east_campus: 0.0,
        doubles: 0.0,
        triples: 0.0,
        quad: 0.0,
        suite: 5.0,
        coed: 3.0,
        elevator: 1.0,
        laundry_other_building: 0.0,
        laundry_one_floor: 2.0,
        laundry_every_floor: 5.0,
        built: 7.0,
        last_renovated: 8.0,
        room_area: 1.0,
    };

    housing.sort_by(|a, b| {
        score_function(a, &weights)
            .partial_cmp(&score_function(b, &weights))
            .unwrap()
    });
    housing.reverse();

    for hall in housing {
        println!("{}: {}", hall.name, score_function(&hall, &weights));
    }
}

struct Weights {
    bathroom_shared_on_hall: f64,
    bathroom_shared_per_room: f64,
    west_campus: f64,
    east_campus: f64,
    doubles: f64,
    triples: f64,
    quad: f64,
    suite: f64,
    coed: f64,
    elevator: f64,
    laundry_other_building: f64,
    laundry_one_floor: f64,
    laundry_every_floor: f64,
    built: f64,
    last_renovated: f64,
    room_area: f64,
}

fn score_function(hall: &Hall, weights: &Weights) -> f64 {
    let mut score = 0.0;
    score += match hall.bathroom {
        Bathroom::SharedOnHall => weights.bathroom_shared_on_hall,
        Bathroom::PerRoom => weights.bathroom_shared_per_room,
    };
    score += match hall.location {
        Location::WestCampus => weights.west_campus,
        Location::EastCampus => weights.east_campus,
    };
    for style in hall.room_styles {
        score += match style {
            RoomStyle::Doubles => weights.doubles,
            RoomStyle::Triples => weights.triples,
            RoomStyle::Quad => weights.quad,
            RoomStyle::Suite => weights.suite,
        };
    }
    if hall.coed {
        score += weights.coed;
    }
    if hall.elevator {
        score += weights.elevator;
    }
    score += match hall.laundry {
        Laundry::OtherBuilding => weights.laundry_other_building,
        Laundry::OneFloor => weights.laundry_one_floor,
        Laundry::EveryFloor => weights.laundry_every_floor,
    };
    fn normalize(x: i32, min_x: i32, max_x: i32) -> f64 {
        (x - min_x) as f64 / (max_x - min_x) as f64
    }
    score += normalize(hall.built, 1925, 1984) * weights.built;
    score += normalize(hall.last_renovated, 1984, 2015) * weights.last_renovated;
    score += normalize(hall.room_area, 21600, 27360) * weights.room_area;
    score
}
