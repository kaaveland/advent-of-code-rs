use std::{fs, io};
use std::fs::File;
use std::io::Write;
use anyhow::{anyhow, Result};
use reqwest::blocking::{Client, ClientBuilder};

fn obtain_cookie() -> Result<String> {
    println!("Enter your advent of code session cookie");
    let mut buf = String::new();
    let read = io::stdin().read_line(&mut buf)?;
    if read < 100 {
        Err(anyhow!("Not a valid session cookie, it should be over 100 bytes"))
    } else {
        Ok(buf.trim().replace("session:", "").replace("\"", ""))
    }
}

fn obtain_user_agent() -> Result<String> {
    println!("Enter an email account that can be used to contact you");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    if !buf.contains("@") {
        Err(anyhow!("Not a valid email account: {buf}"))
    } else {
        Ok(buf.trim().to_string())
    }
}

fn obtain_client() -> Result<Client> {
    let user_agent = obtain_user_agent()?;
    let client = ClientBuilder::new()
        .user_agent(user_agent)
        .build()?;
    Ok(client)
}

fn download_day(client: &Client, cookie: &str, day: u8) -> Result<String> {
    if day < 1 || day > 25 {
        Err(anyhow!("Day should be between 1 and 25, got {day}"))
    } else {
        let url = format!("https://adventofcode.com/2021/day/{day}/input");
        let resp = client.get(url).header("Cookie", format!("session={cookie}")).send()?;
        let status = resp.status();
        if status.is_success() {
            let data = resp.text()?;
            Ok(data)
        } else {
            Err(anyhow!("Got response {status} from aoc"))
        }
    }
}

fn put_day(day: u8, day_content: String) -> Result<()> {
    if day < 1 || day > 25 {
        Err(anyhow!("Day should be between 1 and 25, got {day}"))
    } else {
        let folder = format!("./input/day_{day:0>2}");
        let dest = format!("{folder}/input");
        fs::create_dir_all(folder)?;
        let mut fp = File::create(dest.as_str())?;
        fp.write_all(day_content.as_bytes())?;
        Ok(())
    }
}

pub fn single_day(day: u8) -> Result<()> {
    let client = obtain_client()?;
    let cookie = obtain_cookie()?;
    let content = download_day(&client, cookie.as_str(), day)?;
    put_day(day, content)
}

pub fn all_days() -> Result<()> {
    let client = obtain_client()?;
    let cookie = obtain_cookie()?;

    for day in 1..=25 {
        let day = day as u8;
        let content = download_day(&client, cookie.as_str(), day)?;
        put_day(day, content)?;
    }

    Ok(())
}