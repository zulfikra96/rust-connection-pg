use std::vec;
use postgres::{Client, NoTls};
use std::env;
use dotenv::dotenv;
struct Users {
    user_id: i32,
    name: String,
    email: String,
}


struct UsersProfile  {
    mobile: String,
    userable_id: i32,
    userable_type: String,
    password: String,
    country: String,
    country_code: String,
    region: String,
}

fn main() {
    // init dot env
    dotenv().ok();
    let database_url = env::var("DATABASE_URL");
    let mut client = Client::connect(&database_url.unwrap(), NoTls).unwrap();
    let query = client.query("SELECT user_id, name, email FROM users LIMIT 10", &[]);

    if query.is_ok() == true{
        for row in query.unwrap() {
            let result = Users {
                user_id: row.get(0),
                name: row.get(1),
                email: row.get(2)
            };
            println!("{}", result.name);
        }
    }

    let users = get_user(&mut client);

    for row in users.unwrap() {
        println!("Name {}, Email {}, user_id {}", row.name, row.email, row.user_id);
    }

    // Get user detail
    println!("Get user detail >>>>>>>>>>>>>>>>>>>>");

    let user_detail = get_user_by_email(&mut client,String::from("zulfikralahmudin@gmail.com")).unwrap();

    println!("{} {} {} ", user_detail[0].name , user_detail[0].email, user_detail[0].user_id);
    println!("Add user >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    let add_user = Users {
        name: String::from("Zulfikra L Abdjul"),
        email: String::from("lahmudin@gmail.com"),
        user_id:0
    };
    let add_user_profile = UsersProfile {
        country: String::from("indonesia"),
        country_code: String::from("id"),
        mobile: String::from("081231241"),
        password: String::from("1"),
        region: String::from("idn"),
        userable_id:999,
        userable_type:String::from("worker"),
    };
    add_new_user(&mut client, add_user, add_user_profile);
 }

fn get_user(client: &mut Client) -> Result<Vec<Users>,()> {
    
    let res = match client.query("SELECT user_id, name, email FROM users LIMIT 10", &[]) {
        Ok(res) => res,
        Err(_) =>  panic!("Something is not right",)
    };

    let mut response = vec![];
    if res.is_empty() {
        Err(())
    }else {
        for _res in res {
            let r = Users {
                user_id: _res.get(0),
                name: _res.get(1),
                email: _res.get(2)
            };
            response.push(r);
        }
        Ok(response)
    }
}

fn get_user_by_email(client: &mut Client, name: String) -> Result<Vec<Users>, ()> {
    let res = match client.query("SELECT name, user_id, email FROM users WHERE email = $1", &[&name.to_owned()]) {
        Ok(_res) => _res,
        Err(err) => panic!("{}", err)
    };
    let mut response = vec![];
    let user = Users {
        name: res[0].get(0),
        user_id: res[0].get(1),
        email: res[0].get(2)
    };

    response.push(user);
    Ok(response)
}

fn add_new_user(client: &mut Client, user: Users, user_profile: UsersProfile) -> Result<(), ()> {
    match client.execute(
        "INSERT INTO users (name, email, mobile, userable_id, userable_type, password, country, country_code) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", 
        &[&user.name, &user.email, &user_profile.mobile, &user_profile.userable_id, &"worker", &user_profile.password, &user_profile.country, &user_profile.country_code]) {
            Ok(_) => (),
            Err(err) => panic!("{}", err)
        }
    Ok(())
}