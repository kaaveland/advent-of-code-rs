use crate::YEARS;
use anyhow::{anyhow, Context, Result};
use reqwest::blocking::{Client, ClientBuilder};
use std::fs::File;
use std::io::Write;
use std::{fs, io};

fn obtain_cookie_from_disk() -> Result<Option<String>> {
    let home = dirs::home_dir().context("Unable to resolve $HOME")?;
    let cookie_path = home.join(".aoc_cookie");
    Ok(fs::read_to_string(cookie_path).ok())
}
fn write_cookie_to_disk(cookie: &str) -> Result<()> {
    let home = dirs::home_dir().context("Unable to resolve $HOME")?;
    let cookie_path = home.join(".aoc_cookie");
    Ok(fs::write(cookie_path, cookie)?)
}

fn cookie_fallback() -> Result<String> {
    println!("Enter your advent of code session cookie");
    let mut buf = String::new();
    let read = io::stdin().read_line(&mut buf)?;
    if read < 100 {
        Err(anyhow!(
            "Not a valid session cookie, it should be over 100 bytes"
        ))
    } else {
        Ok(buf.trim().replace("session:", "").replace('\"', ""))
    }
}

fn obtain_cookie() -> Result<String> {
    if let Some(cookie) = obtain_cookie_from_disk()? {
        Ok(cookie)
    } else {
        let cookie = cookie_fallback()?;
        write_cookie_to_disk(cookie.as_str())?;
        Ok(cookie)
    }
}

fn obtain_user_agent_from_disk() -> Result<Option<String>> {
    let home = dirs::home_dir().context("Unable to resolve $HOME")?;
    let uagent_path = home.join(".aoc_uagent");
    Ok(fs::read_to_string(uagent_path).ok())
}

fn write_user_agent_to_disk(uagent: &str) -> Result<()> {
    let home = dirs::home_dir().context("Unable to resolve $HOME")?;
    let uagent_path = home.join(".aoc_uagent");
    Ok(fs::write(uagent_path, uagent)?)
}

fn obtain_user_agent() -> Result<String> {
    if let Some(agent) = obtain_user_agent_from_disk()? {
        Ok(agent)
    } else {
        println!("Enter an email account that can be used to contact you");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        if !buf.contains('@') {
            Err(anyhow!("Not a valid email account: {buf}"))
        } else {
            write_user_agent_to_disk(buf.trim())?;
            Ok(buf.trim().to_string())
        }
    }
}

fn obtain_client() -> Result<Client> {
    let user_agent = obtain_user_agent()?;
    let client = ClientBuilder::new().user_agent(user_agent).build()?;
    Ok(client)
}

fn download_day(client: &Client, cookie: &str, year: u16, day: u8) -> Result<String> {
    valid_data(year, day)?;

    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let resp = client
        .get(url)
        .header("Cookie", format!("session={cookie}"))
        .send()?;
    let status = resp.status();
    if status.is_success() {
        let data = resp.text()?;
        Ok(data)
    } else {
        Err(anyhow!("Got response {status} from aoc"))
    }
}

fn valid_data(year: u16, day: u8) -> Result<()> {
    let years: Vec<_> = YEARS.iter().map(|(y, _)| *y).collect();
    if !((1..=25).contains(&day) && years.contains(&year)) {
        Err(anyhow!(
            "Day should be between 1 and 25 and year from {years:?}, got {day} and {year}"
        ))
    } else {
        Ok(())
    }
}

fn put_day(year: u16, day: u8, day_content: String) -> Result<()> {
    valid_data(year, day)?;

    let folder = format!("./input/{year}/day_{day:0>2}");
    let dest = format!("{folder}/input");
    fs::create_dir_all(folder)?;
    let mut fp = File::create(dest.as_str())?;
    fp.write_all(day_content.as_bytes())?;
    Ok(())
}

pub fn single_day(year: u16, day: u8) -> Result<()> {
    let client = obtain_client()?;
    let cookie = obtain_cookie()?;
    let content = download_day(&client, cookie.as_str(), year, day)?;
    put_day(year, day, content)
}

pub fn all_days(year: u16) -> Result<()> {
    let client = obtain_client()?;
    let cookie = obtain_cookie()?;

    for day in 1..=25 {
        let day = day as u8;
        let content = download_day(&client, cookie.as_str(), year, day)?;
        put_day(year, day, content)?;
    }

    Ok(())
}
