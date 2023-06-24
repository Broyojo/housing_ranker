use crate::hall::Hall;
use serde::{Deserialize, Serialize};

const ACCESS_TOKEN: &str = "";

// async fn get_coordintes(housing: Vec<Hall>) -> Vec<(f64, f64)> {
//     for hall in housing {
//         let url = format!(
//             "https://api.mapbox.com/geocoding/v5/address/{}.json",
//             hall.address
//         );
//         let response = reqwest::get(&url).await.unwrap();
//         let result = response.json().await?;
//     }

//     todo!();
// }

// struct ResponseBody {
//     code: String,
// }

async fn get_duration_matrix(coords1: Vec<(f64, f64)>, coord2: Vec<(f64, f64)>) -> Vec<Vec<f64>> {
    let url = "https://api.mapbox.com/directions-matrix/v1/mapbox/driving/A;B;C;D;E?sources=0&destinations=1;2;3;4&access_token=...";
    todo!();
}

#[derive(Deserialize, Debug)]
struct ResponseBody {
    code: String,
    durations: Vec<Vec<f64>>,
    destinations: Vec<Destination>,
    sources: Vec<Source>,
}

#[derive(Deserialize, Debug)]
struct Destination {
    name: String,
    location: (f64, f64),
    distance: f64,
}

#[derive(Deserialize, Debug)]
struct Source {
    name: String,
    location: (f64, f64),
    distance: f64,
}
