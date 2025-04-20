extern crate postgres;

use postgres::{Client, Error, GenericClient, NoTls};
use sha2::{Digest, Sha256, Sha512};
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::ptr::hash;

const CONN_TO: &'static str = "postgresql://postgres:postgres@192.168.88.242/postgres";

struct Person {
    id: i32,
    name: String,
    password: String,
}

fn get_user_line(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn check_for_empty_string(prompt: &str) -> bool {
    !prompt.is_empty()
}

fn str_to_sha254(prompt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(prompt);
    let result = format!("{:x}", hasher.finalize());
    result
}

fn add_person_to_pgsql() -> Result<(), Error> {
    let mut client = Client::connect(CONN_TO, NoTls)?;

    let username = get_user_line("Имя пользователя: ");
    if !check_for_empty_string(&username) { // доделать проверку
        println!("Строка пустая...")
    }

    let password = get_user_line("Пароль: ");
    if !check_for_empty_string(&password) { // доделать проверку
        println!("Строка пустая...")
    }

    let sha254_password = str_to_sha254(&password);

    println!("Username: {}", username.clone());
    println!("Password: {}", sha254_password.clone());

    let person = Person {
        id: 0,
        name: username.clone(),
        password: sha254_password.clone(),
    };

    client.execute(
        "INSERT INTO users (name, password) VALUES ($1, $2)",
        &[&person.name, &person.password],
    )?;

    Ok(())
}

fn get_all_persons() -> Result<(), Error> {
    let mut client = Client::connect(CONN_TO, NoTls)?;

    println!("Список всех пользователей");
    for row in client.query("SELECT * FROM users", &[])? {
        let user = Person {
            id: row.get(0),
            name: row.get(1),
            password: row.get(2),
        };
        println!(
            "id: {}, user: {}, hash_password: {}",
            user.id, user.name, user.password
        );
    }
    Ok(())
}

fn con_and_create_to_pgsql() -> Result<(), Error> {
    let mut client = Client::connect(CONN_TO, NoTls)?;

    client.batch_execute(
        "
         CREATE TABLE IF NOT EXISTS users  (
             id              SERIAL PRIMARY KEY,
             name           TEXT NOT NULL,
             password       TEXT NOT NULL
             )
     ",
    )?;
    Ok(())
}

fn hash_test(prompt: String) -> Result<(), Error> {
    let text = String::from("asdasd");

    let mut hasher1 = Sha256::new();
    let mut hasher2 = Sha256::new();

    hasher1.update(prompt);
    hasher2.update(text);

    let prompt_result = hasher1.finalize();
    let text_result = hasher2.finalize();

    if prompt_result == text_result {
        println!("Совпадает");
    } else {
        println!("Не совпадает");
    }
    Ok(())
}

fn main() {
    match con_and_create_to_pgsql() {
        Ok(()) => println!("БД: Ок"),
        Err(e) => println!("Connect to Database: {}", e),
    }
    match add_person_to_pgsql() {
        Ok(()) => println!("Add new entry: Ok"),
        Err(e) => println!("Error adding user: {}", e),
    }
    match get_all_persons() {
        Ok(()) => println!("Receiving records: Ok"),
        Err(e) => println!("Error retrieving records: {}", e),
    }
    // match hash_test("asdasd".to_string()) {
    //     Ok(()) => println!("Hash ok"),
    //     Err(e) => println!("Hash not ok: {}", e),
    // }
}
