use exitfailure::ExitFailure;
use std::env;
use colored::*;
use requestty::{Question, Answer};
//use spinners::{Spinner, Spinners};

mod coin;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
  println!("\n{}\n", "Coind".blue().bold());

  let mut args = env::args();
  let coin_id = args.nth(1);
  let currency = args.nth(0);

  if coin_id.is_none() {
    let question = Question::select("home")
      .message("What would you want to do?")
      .choices(vec![
        "Check what's trending",
        "Check a coin's current data"
      ]).build();
  

    let result: Answer = requestty::prompt_one(question).unwrap();
    match result.as_list_item().unwrap().index {
      0 => {
        let items = coin::Trending::fetch().await.unwrap();
        println!("{}\n", "Trending Coins".yellow().bold());
        
        for trending in items.coins {
          println!("Name: {}", trending.item.name.cyan().bold());
          println!("Symbol: {}", trending.item.symbol.cyan().bold());
          println!("Bitcoin Price: {}", trending.item.price_btc.to_string().cyan().bold());
          println!("Market Cap Rank: {}\n", trending.item.market_cap_rank.to_string().cyan().bold());
        }
      },
      1 => {
        let mut id = String::new();
        let mut money = String::new();
    
        println!("Enter coin id: ");
        std::io::stdin().read_line(&mut id).unwrap();
        println!("Enter currency: ");
        std::io::stdin().read_line(&mut money).unwrap();
    
        fetch_money(&id, &money).await;
      },
      _ => println!("Invalid")
    }
  } else {
    let mut money: String = String::from("php");
    if currency.is_some() {
      money = currency.unwrap();
    } else {
      println!("There are no specified currency; default one \"PHP\" will be used instead.");
    }

    let id = coin_id.unwrap();
    fetch_money(&id, &money).await;
  }

  println!("\n");
  return Ok(());
}

async fn fetch_money(id: &String, currency: &String) {
  //let spinner = Spinner::new(&Spinners::Dots9, "Fetching".into());
  let coin = coin::Coin::fetch(&id).await.unwrap();
  //spinner.stop();

  println!("Name: {}", coin.name.cyan().bold());
  println!("Symbol: {}", coin.symbol.cyan().bold());
  println!("Current Price: {} {}", currency.trim().to_uppercase().cyan(), coin.market_data.current_price[currency.trim()].to_string().cyan().bold());
}
