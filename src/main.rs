use std::vec;
use postgres::{Client, NoTls};
use std::env;
use dotenv::dotenv;
struct Users {
    user_id: i32,
    name: String,
    email: String
}

fn main() {
    // init dot env
    dotenv().ok();
    let DATABASE_URL = env::var("DATABASE_URL");
    let mut client = Client::connect(&DATABASE_URL.unwrap(), NoTls).unwrap();
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

    println!("{} {} {} ", user_detail[0].name , user_detail[0].email, user_detail[0].user_id)
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

