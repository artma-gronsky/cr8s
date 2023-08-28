use clap::{Arg, Command};
use dotenv::dotenv;

extern crate cr8s;

fn main() {
    dotenv().ok();
    
    let matches = Command::new("Cr8s")
        .about("Cr8s commands")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
                .about("User management ")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Create new user with multiple roles attached")
                        .arg_required_else_help(true)
                        .arg(Arg::new("username").required(true))
                        .arg(Arg::new("password").required(true))
                        .arg(
                            Arg::new("roles")
                                .required(true)
                                .num_args(1..)
                                .value_delimiter(','),
                        ),
                )
                .subcommand(Command::new("list").about("List of all abailable users"))
                .subcommand(
                    Command::new("delete")
                        .about("Delete user by id")
                        .arg_required_else_help(true)
                        .arg(Arg::new("id").required(true).value_parser(clap::value_parser!(i32))),
                ),
        )
        .subcommand(
            Command::new("digest-send")
            .about("Send an email about newest crates")
            .arg(Arg::new("to").required(true))
            .arg(Arg::new("hours_since") .required(true).value_parser(clap::value_parser!(i32)))
        )
        .get_matches();


        match matches.subcommand() {
            Some(("users", sub_matches)) => {
                match sub_matches.subcommand(){
                            Some(("create", arg)) =>{
                                cr8s::commands::crate_user(
                                    arg.get_one::<String>("username").unwrap().to_owned(), 
                                    arg.get_one::<String>("password").unwrap().to_owned(),
                                    arg.get_many::<String >("roles").unwrap().map(|v| v.to_string()).collect())  ;
                            }
                            Some(("list", _)) =>{
                                cr8s::commands::list_users()
                            }
                            Some(("delete", arg)) =>{
                               cr8s::commands::delete_user(*arg.get_one::<i32>("id").unwrap())
                            }
                            _ => {}
                        }
            },
            Some(("digest-send", sub_matches)) => 
            cr8s::commands::send_digest(
                sub_matches.get_one::<String>("to").unwrap().to_owned(), 
                sub_matches.get_one::<i32>("hours_since").unwrap().to_owned()
            ),
            _ => {},
        };
}
