#[path = "./structs/book.rs"]
mod book;
#[path = "./structs/details.rs"]
mod details;
#[path = "./structs/login.rs"]
mod login;
#[path = "./structs/times.rs"]
mod times;

use book::BookResponse;
use chrono::prelude::*;
use chrono::NaiveTime;
use details::DetailsResponse;
use login::LoginRoot;
use reqwest::header::{HeaderMap, AUTHORIZATION, CACHE_CONTROL, HOST, USER_AGENT};
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use std::process;
use times::VenueResponse;

#[derive(Clone)]
struct Config {
    day: String,
    reserve_time: Option<NaiveTime>,
    party_size: String,
    venue_id: String,
    area: Option<String>,
    new_reservations: NaiveTime,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct DetailsBody {
    commit: i8,
    config_id: String,
    day: String,
    party_size: i8,
}

#[derive(Serialize)]
struct ReservePayment {
    id: i32,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Fill out config here
    let config = Config {
        day: "2022-07-30".to_string(),
        reserve_time: Some(NaiveTime::parse_from_str("17:00", "%H:%M").unwrap()),
        party_size: "2".to_string(),
        // Find venue ID in network tab
        venue_id: "834".to_string(),
        // Leave this as None, choosing a specific area does not work.
        area: None,
        // When to start looking for reservations
        new_reservations: NaiveTime::parse_from_str("9:00", "%H:%M").unwrap(),
        email: "foo@bar.com".to_string(),
        password: "foobar123".to_string(),
    };

    let mut login_params: HashMap<String, String> = HashMap::new();
    login_params.insert("email".to_string(), config.email);
    login_params.insert("password".to_string(), config.password);

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        r#"ResyAPI api_key="VbWk7s3L4KiK5fzlO7JD3Q5EYolJI7n5""#
            .parse()
            .unwrap(),
    );
    headers.insert(CACHE_CONTROL, "no-cache".parse().unwrap());
    headers.insert(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert(HOST, "resy.com".parse().unwrap());

    let login: LoginRoot = reqwest::Client::new()
        .post("https://api.resy.com/3/auth/password")
        .headers(headers.clone())
        .form(&login_params)
        .send()
        .await?
        .json()
        .await?;

    loop {
        if config.new_reservations >= Local::now().time() {
            continue;
        }
        println!("Finding venue");

        let find_reservation: VenueResponse = reqwest::Client::new()
            .get(format!(
                "https://api.resy.com/4/find?lat=0&long=0&day={}&party_size={}&venue_id={}",
                &config.day, &config.party_size, &config.venue_id
            ))
            .headers(headers.clone())
            .send()
            .await?
            .json()
            .await?;

        let slots = &find_reservation.results.venues[0].slots;

        if slots.len() < 1 {
            println!("cannot find slot yet");
            continue;
        }

        for slot in slots {
            if config.reserve_time.is_some()
                && !(slot.date.start.time() == config.reserve_time.unwrap())
                || config.area.is_some() && &slot.config.location == config.area.as_ref().unwrap()
            {
                continue;
            }

            println!("Slot found");

            let details_body = DetailsBody {
                commit: 1,
                config_id: slot.config.token.clone(),
                day: config.day.clone(),
                party_size: config.party_size.parse::<i8>().unwrap(),
            };

            let booking_details: DetailsResponse = reqwest::Client::new()
                .post("https://api.resy.com/3/details")
                .json(&details_body)
                .headers(headers.clone())
                .header("x-resy-universal-auth", &login.token)
                .send()
                .await?
                .json()
                .await?;

            println!("{:#?}", booking_details);
            println!("Book token received");

            let mut reserve_params: HashMap<String, String> = HashMap::new();
            reserve_params.insert("book_token".to_string(), booking_details.book_token.value);
            reserve_params.insert(
                "source_id".to_string(),
                "resy.com-venue-details".to_string(),
            );

            if booking_details.cancellation.fee.is_some() {
                if booking_details.user.payment_methods.is_none() {
                    println!(
                    "Payment required but none found on Resy account. Please add one to continue"
                );
                    process::exit(1)
                }

                let method = &booking_details.user.payment_methods.unwrap()[0];

                let payment_info = ReservePayment { id: method.id };

                println!(
                    "Payment required, using {} card that ends in {}",
                    method.card_type, method.display
                );
                reserve_params.insert(
                    "struct_payment_method".to_string(),
                    serde_json::to_string(&payment_info).unwrap(),
                );
            }

            let reserve: BookResponse = reqwest::Client::new()
                .post("https://api.resy.com/3/book")
                .form(&reserve_params)
                .headers(headers)
                .header("x-resy-universal-auth", login.token)
                .send()
                .await?
                .json()
                .await?;

            println!("{:#?}", reserve);
            println!("Reservation booked!");
            break;
        }
        break;
    }

    Ok(())
}
