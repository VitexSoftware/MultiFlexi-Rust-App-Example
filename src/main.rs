use dotenv::dotenv;
use std::env;

fn main() {

    println!("Hello, world!");

    dotenv().ok();

    let abraflexi_url = env::var("ABRAFLEXI_URL").expect("ABRAFLEXI_URL must be set");
    let abraflexi_login = env::var("ABRAFLEXI_LOGIN").expect("ABRAFLEXI_LOGIN must be set");
    let abraflexi_password = env::var("ABRAFLEXI_PASSWORD").expect("ABRAFLEXI_PASSWORD must be set");
    let abraflexi_company = env::var("ABRAFLEXI_COMPANY").expect("ABRAFLEXI_COMPANY must be set");

    println!("ABRAFLEXI_URL: {}", abraflexi_url);
    println!("ABRAFLEXI_LOGIN: {}", abraflexi_login);
    println!("ABRAFLEXI_PASSWORD: {}", abraflexi_password);
    println!("ABRAFLEXI_COMPANY: {}", abraflexi_company);
}
