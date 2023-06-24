#[derive(Debug)]
pub enum Bathroom {
    SharedOnHall,
    PerRoom,
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Community {
    FE,
    Explore,
    GrandChallenges,
    TYE,
    FYSA,
    GlobalLeadership,
}

#[derive(Debug)]
pub enum Location {
    WestCampus,
    EastCampus,
}

#[derive(Debug)]
pub enum RoomStyle {
    Doubles,
    Triples,
    Quad,
    Suite,
}

#[derive(Debug)]
pub enum Laundry {
    OtherBuilding,
    OneFloor,
    EveryFloor,
}

#[derive(Debug)]
pub struct Hall {
    pub name: &'static str,
    pub bathroom: Bathroom,
    pub communities: &'static [Community],
    pub location: Location,
    pub room_styles: &'static [RoomStyle],
    pub coed: bool,
    pub elevator: bool,
    pub laundry: Laundry,
    pub built: i32,
    pub last_renovated: i32,
    pub room_area: i32, // square inches
}

pub fn get_housing() -> Vec<Hall> {
    vec![
        Hall {
            name: "Armstrong",
            bathroom: Bathroom::SharedOnHall,
            communities: &[Community::FE],
            location: Location::WestCampus,
            room_styles: &[RoomStyle::Doubles],
            coed: true,
            elevator: true,
            laundry: Laundry::OtherBuilding,
            built: 1969,
            last_renovated: 2006,
            room_area: 24920,
        },
        Hall {
            name: "Brown",
            bathroom: Bathroom::SharedOnHall,
            communities: &[Community::FE],
            location: Location::EastCampus,
            room_styles: &[RoomStyle::Doubles],
            coed: true,
            elevator: false,
            laundry: Laundry::OtherBuilding,
            built: 1925,
            last_renovated: 1993,
            room_area: 21600,
        },
        Hall {
            name: "Caldwell",
            bathroom: Bathroom::SharedOnHall,
            communities: &[Community::FE, Community::Explore],
            location: Location::WestCampus,
            room_styles: &[RoomStyle::Doubles],
            coed: true,
            elevator: false,
            laundry: Laundry::OtherBuilding,
            built: 1969,
            last_renovated: 2002,
            room_area: 21600,
        },
        Hall {
            name: "Cloudman",
            bathroom: Bathroom::SharedOnHall,
            communities: &[Community::FE, Community::GrandChallenges],
            location: Location::EastCampus,
            room_styles: &[RoomStyle::Doubles, RoomStyle::Triples],
            coed: true,
            elevator: false,
            laundry: Laundry::OtherBuilding,
            built: 1931,
            last_renovated: 1995,
            room_area: 21600,
        },
        Hall {
            name: "Field",
            bathroom: Bathroom::SharedOnHall,
            communities: &[Community::FE],
            location: Location::EastCampus,
            room_styles: &[RoomStyle::Doubles, RoomStyle::Quad],
            coed: true,
            elevator: false,
            laundry: Laundry::OtherBuilding,
            built: 1961,
            last_renovated: 2005,
            room_area: 23760,
        },
        Hall {
            name: "Fitten",
            bathroom: Bathroom::SharedOnHall,
            communities: &[Community::FE],
            location: Location::WestCampus,
            room_styles: &[RoomStyle::Doubles],
            coed: true,
            elevator: true,
            laundry: Laundry::OneFloor,
            built: 1972,
            last_renovated: 2012,
            room_area: 26410,
        },
        Hall {
            name: "Folk",
            bathroom: Bathroom::SharedOnHall,
            communities: &[Community::FE, Community::Explore],
            location: Location::WestCampus,
            room_styles: &[RoomStyle::Doubles],
            coed: true,
            elevator: false,
            laundry: Laundry::OtherBuilding,
            built: 1969,
            last_renovated: 2005,
            room_area: 25576,
        },
        Hall {
            name: "Freeman",
            bathroom: Bathroom::SharedOnHall,
            communities: &[Community::FE],
            location: Location::WestCampus,
            room_styles: &[RoomStyle::Doubles],
            coed: true,
            elevator: true,
            laundry: Laundry::OtherBuilding,
            built: 1972,
            last_renovated: 2011,
            room_area: 27160,
        },
        Hall {
            name: "Glenn",
            bathroom: Bathroom::SharedOnHall,
            communities: &[Community::FE],
            location: Location::EastCampus,
            room_styles: &[RoomStyle::Doubles, RoomStyle::Triples, RoomStyle::Quad],
            coed: true,
            elevator: true,
            laundry: Laundry::EveryFloor,
            built: 1947,
            last_renovated: 2015,
            room_area: 23808,
        },
        Hall {
            name: "Hanson",
            bathroom: Bathroom::SharedOnHall,
            communities: &[Community::FE],
            location: Location::EastCampus,
            room_styles: &[RoomStyle::Doubles],
            coed: true,
            elevator: false,
            laundry: Laundry::OtherBuilding,
            built: 1961,
            last_renovated: 2002,
            room_area: 24156,
        },
        Hall {
            name: "Harris",
            bathroom: Bathroom::PerRoom,
            communities: &[Community::FE, Community::TYE],
            location: Location::EastCampus,
            room_styles: &[RoomStyle::Suite],
            coed: true,
            elevator: true,
            laundry: Laundry::OtherBuilding,
            built: 1926,
            last_renovated: 1992,
            room_area: 25920,
        },
        Hall {
            name: "Harrison",
            bathroom: Bathroom::SharedOnHall,
            communities: &[Community::FE],
            location: Location::EastCampus,
            room_styles: &[RoomStyle::Doubles, RoomStyle::Quad],
            coed: true,
            elevator: false,
            laundry: Laundry::OtherBuilding,
            built: 1939,
            last_renovated: 1998,
            room_area: 24156,
        },
        Hall {
            name: "Hefner",
            bathroom: Bathroom::SharedOnHall,
            communities: &[Community::FE],
            location: Location::WestCampus,
            room_styles: &[RoomStyle::Doubles],
            coed: true,
            elevator: true,
            laundry: Laundry::OtherBuilding,
            built: 1969,
            last_renovated: 2008,
            room_area: 24920,
        },
        Hall {
            name: "Hopkins",
            bathroom: Bathroom::SharedOnHall,
            communities: &[Community::FE],
            location: Location::EastCampus,
            room_styles: &[RoomStyle::Doubles, RoomStyle::Quad],
            coed: true,
            elevator: false,
            laundry: Laundry::OneFloor,
            built: 1961,
            last_renovated: 1995,
            room_area: 24120,
        },
        Hall {
            name: "Howell",
            bathroom: Bathroom::SharedOnHall,
            communities: &[Community::FE, Community::GrandChallenges],
            location: Location::EastCampus,
            room_styles: &[RoomStyle::Doubles, RoomStyle::Quad],
            coed: true,
            elevator: false,
            laundry: Laundry::OtherBuilding,
            built: 1939,
            last_renovated: 1999,
            room_area: 23808,
        },
        Hall {
            name: "Matheson",
            bathroom: Bathroom::SharedOnHall,
            communities: &[Community::FYSA, Community::FE, Community::GlobalLeadership],
            location: Location::EastCampus,
            room_styles: &[RoomStyle::Doubles],
            coed: true,
            elevator: false,
            laundry: Laundry::OtherBuilding,
            built: 1961,
            last_renovated: 2002,
            room_area: 24120,
        },
        Hall {
            name: "Montag",
            bathroom: Bathroom::SharedOnHall,
            communities: &[Community::FE],
            location: Location::WestCampus,
            room_styles: &[RoomStyle::Doubles],
            coed: true,
            elevator: true,
            laundry: Laundry::OtherBuilding,
            built: 1972,
            last_renovated: 2011,
            room_area: 27160,
        },
        Hall {
            name: "Perry",
            bathroom: Bathroom::SharedOnHall,
            communities: &[Community::FYSA, Community::FE, Community::GlobalLeadership],
            location: Location::EastCampus,
            room_styles: &[RoomStyle::Doubles],
            coed: true,
            elevator: false,
            laundry: Laundry::OtherBuilding,
            built: 1961,
            last_renovated: 2002,
            room_area: 23040,
        },
        Hall {
            name: "Smith",
            bathroom: Bathroom::SharedOnHall,
            communities: &[Community::FE],
            location: Location::EastCampus,
            room_styles: &[RoomStyle::Doubles, RoomStyle::Quad],
            coed: false,
            elevator: false,
            laundry: Laundry::OtherBuilding,
            built: 1947,
            last_renovated: 1993,
            room_area: 23736,
        },
        Hall {
            name: "Towers",
            bathroom: Bathroom::SharedOnHall,
            communities: &[Community::FE],
            location: Location::EastCampus,
            room_styles: &[RoomStyle::Doubles, RoomStyle::Triples, RoomStyle::Quad],
            coed: true,
            elevator: true,
            laundry: Laundry::EveryFloor,
            built: 1947,
            last_renovated: 2014,
            room_area: 23994,
        },
        Hall {
            name: "Woodruff North",
            bathroom: Bathroom::PerRoom,
            communities: &[Community::FE],
            location: Location::WestCampus,
            room_styles: &[RoomStyle::Suite],
            coed: true,
            elevator: true,
            laundry: Laundry::OtherBuilding,
            built: 1984,
            last_renovated: 2020,
            room_area: 27360,
        },
        Hall {
            name: "Woodruff South",
            bathroom: Bathroom::PerRoom,
            communities: &[Community::FE],
            location: Location::WestCampus,
            room_styles: &[RoomStyle::Suite],
            coed: true,
            elevator: true,
            laundry: Laundry::OneFloor,
            built: 1984,
            last_renovated: 2020,
            room_area: 27360,
        },
    ]
}