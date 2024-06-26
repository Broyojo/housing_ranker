mod hall;

use hall::{get_housing, Bathroom, Hall, Laundry, Location, RoomStyle};

#[tokio::main]
async fn main() {
    let mut housing = get_housing();

    let weights = Weights {
        bathroom_shared_on_hall: 1.0,      // Less desirable
        bathroom_shared_per_room: 3.0,     // More desirable
        west_campus: 3.0,                  // Quiet, larger rooms, near some amenities
        east_campus: 3.0,                  // Vibrant, near dining options, Greek life
        doubles: 3.0,                      // A balance of privacy and social interaction
        triples: 1.0,                      // Less privacy
        quad: 2.0,                         // More social interaction, but potentially less privacy
        suite: 4.0,                        // More privacy and typically newer facilities
        coed: 3.0,                         // Maintaining the same weight
        elevator: 2.0,                     // Higher accessibility can be a nice convenience
        laundry_other_building: 1.0,       // Less convenient
        laundry_one_floor: 2.0,            // More convenient than another building
        laundry_every_floor: 4.0,          // Most convenient
        built: 2.0,                        // Newer buildings can have more modern amenities
        last_renovated: 4.0,               // Recent renovations may indicate updated facilities
        room_area: 5.0,                    // Space for studying and relaxation is important
        time_to_brittain: 3.0,             // Dining option
        time_to_student_center: 4.0,       // A hub for resources and activities
        time_to_college_of_computing: 6.0, // High priority for a CS major
        time_to_crc: 3.0,                  // Access to recreation center can be important
        time_to_library: 5.0,              // Access to resources for studying
    };

    let value_ranges = ValueRanges::from(housing.clone());
    println!("{value_ranges:?}");

    housing.sort_by(|a, b| {
        score_function(a, &weights, &value_ranges)
            .partial_cmp(&score_function(b, &weights, &value_ranges))
            .unwrap()
    });
    housing.reverse();

    for hall in housing {
        println!(
            "{}: {}",
            hall.name,
            score_function(&hall, &weights, &value_ranges)
        );
    }
}

#[derive(Debug)]
struct ValueRanges {
    built: (i32, i32),
    last_renovated: (i32, i32),
    room_area: (i32, i32),
    time_to_brittain: (i32, i32),
    time_to_student_center: (i32, i32),
    time_to_college_of_computing: (i32, i32),
    time_to_crc: (i32, i32),
    time_to_library: (i32, i32),
}

impl ValueRanges {
    fn new() -> Self {
        Self {
            built: (i32::MAX, i32::MIN),
            last_renovated: (i32::MAX, i32::MIN),
            room_area: (i32::MAX, i32::MIN),
            time_to_brittain: (i32::MAX, i32::MIN),
            time_to_student_center: (i32::MAX, i32::MIN),
            time_to_college_of_computing: (i32::MAX, i32::MIN),
            time_to_crc: (i32::MAX, i32::MIN),
            time_to_library: (i32::MAX, i32::MIN),
        }
    }
}

impl From<Vec<Hall>> for ValueRanges {
    fn from(halls: Vec<Hall>) -> Self {
        let mut ranges = ValueRanges::new();

        for hall in halls {
            ranges.built = (
                ranges.built.0.min(hall.built),
                ranges.built.1.max(hall.built),
            );
            ranges.last_renovated = (
                ranges.last_renovated.0.min(hall.last_renovated),
                ranges.last_renovated.1.max(hall.last_renovated),
            );
            ranges.room_area = (
                ranges.room_area.0.min(hall.room_area),
                ranges.room_area.1.max(hall.room_area),
            );
            ranges.time_to_brittain = (
                ranges.time_to_brittain.0.min(hall.time_to_brittain),
                ranges.time_to_brittain.1.max(hall.time_to_brittain),
            );
            ranges.time_to_student_center = (
                ranges
                    .time_to_student_center
                    .0
                    .min(hall.time_to_student_center),
                ranges
                    .time_to_student_center
                    .1
                    .max(hall.time_to_student_center),
            );
            ranges.time_to_college_of_computing = (
                ranges
                    .time_to_college_of_computing
                    .0
                    .min(hall.time_to_college_of_computing),
                ranges
                    .time_to_college_of_computing
                    .1
                    .max(hall.time_to_college_of_computing),
            );
            ranges.time_to_crc = (
                ranges.time_to_crc.0.min(hall.time_to_crc),
                ranges.time_to_crc.1.max(hall.time_to_crc),
            );
            ranges.time_to_library = (
                ranges.time_to_library.0.min(hall.time_to_library),
                ranges.time_to_library.1.max(hall.time_to_library),
            );
        }

        ranges
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
    time_to_brittain: f64,
    time_to_student_center: f64,
    time_to_college_of_computing: f64,
    time_to_crc: f64,
    time_to_library: f64,
}

fn score_function(hall: &Hall, weights: &Weights, value_ranges: &ValueRanges) -> f64 {
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
    score += normalize(hall.built, value_ranges.built.0, value_ranges.built.1) * weights.built;
    score += normalize(
        hall.last_renovated,
        value_ranges.last_renovated.0,
        value_ranges.last_renovated.1,
    ) * weights.last_renovated;
    score += normalize(
        hall.room_area,
        value_ranges.room_area.0,
        value_ranges.room_area.1,
    ) * weights.room_area;
    score +=
        (1.0 - normalize(
            hall.time_to_brittain,
            value_ranges.time_to_brittain.0,
            value_ranges.time_to_brittain.1,
        )) * weights.time_to_brittain;
    score +=
        (1.0 - normalize(
            hall.time_to_student_center,
            value_ranges.time_to_student_center.0,
            value_ranges.time_to_student_center.1,
        )) * weights.time_to_student_center;
    score +=
        (1.0 - normalize(
            hall.time_to_college_of_computing,
            value_ranges.time_to_college_of_computing.0,
            value_ranges.time_to_college_of_computing.1,
        )) * weights.time_to_college_of_computing;
    score +=
        (1.0 - normalize(
            hall.time_to_crc,
            value_ranges.time_to_crc.0,
            value_ranges.time_to_crc.1,
        )) * weights.time_to_crc;
    score +=
        (1.0 - normalize(
            hall.time_to_library,
            value_ranges.time_to_library.0,
            value_ranges.time_to_library.1,
        )) * weights.time_to_library;
    score
}
