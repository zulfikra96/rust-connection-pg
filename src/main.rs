use std::vec;

use postgres::{Client, NoTls, Error};


struct Users {
    user_id: i32,
    name: String,
    email: String
}

fn main() {
    let mut client = Client::connect("postgres://qwork:qwork_production@localhost:5678/qwork", NoTls).unwrap();

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
        println!("{}", row.name);
    }
 }

fn get_user(client: &mut Client) -> Result<Vec<Users>,()> {
    let res = match client.query("SELECT user_id, name, email FROM users LIMIT 10", &[]) {
        Ok(res) => res,
        Err(err) =>  panic!("Something is not right",)
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

