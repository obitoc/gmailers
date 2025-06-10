/* this tool made for Sending Gmails
 * Usage
 * ```
 * gmailme <"msg"> <time-to-sleep> <"email-to-send"> <times>
 * ```
 * Hint: Normal Pass of The gmail not Works Use A password that gen by Apps Password ( enable 2fa )
 */

use clap::Parser;

use lettre::{SmtpTransport,
            message::Message,
            transport::smtp::authentication::Credentials,
            Transport};
use colored::Colorize;
#[derive(Debug,Parser)]
#[command(author = "obito", version = "1", about = "Cli Tool For Gmail Sending", long_about = None)]
struct Args {

    // Time To Sleep
    #[arg(short,long,default_value = "5",help = "the time to sleep")]
    time: u64,
    // Your Gmail
    #[arg(short,long,help = "The account gmail address of yours")]
    gmail: String,
    // Acc Pass
    #[arg(short,long,help = "The password Of your Gmail")]
    password: String,
    // How Many
    #[arg(short,long,default_value = "1",help = "How many Time to send")]
    many: i32,
    // The Msg
    #[arg(help = "Your Message")]
    msg: String,
    // Subject
    #[arg(help = "The Subject")]
    subject: String,
    // Email to Send For
    #[arg(help = "the Email You want to Send to")]
    emailto: String,
}
fn main() {
    let args = Args::parse();
    let gmail = args.gmail;
    let password = args.password;
    let msg = args.msg;
    let subject = args.subject;
    let time = args.time;
    let how_many = args.many;
    let to = args.emailto;
    let msgbuild = Message::builder()
        .from(gmail.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(msg)
        .expect("Error While Building The Message");
    let creds = Credentials::new(
        gmail,
        password,
    );
    let sender = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    println!("{}","--------------------------".green().bold());
    for i in 1..=how_many{
        match sender.send(&msgbuild) {
            Ok(_) => println!("{}",format!("🟢 Success Sending, at Number {i}").green().bold()),
            Err(_e) => {

                println!("{}",format!("🔴 Error Sending, at Number {i}").red().bold());
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(time))
    }

    println!("{}","-------------end-------------".green().bold());
}
