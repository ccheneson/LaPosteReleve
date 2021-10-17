use chrono::NaiveDate;
use ordered_float::OrderedFloat;
use std::collections::HashSet;
use std::collections::HashMap;
use std::path::Path;
use anyhow;

use crate::models::{AccountBalance, AccountActivity, BankingStatement};


fn get_balance(stats: HashMap<String, String>) -> anyhow::Result<AccountBalance> {
    let (date, balance) = (stats.get("Date"), stats.get("Solde (EUROS)"));
    let mut err : Vec<&str> = vec!();
    if date.is_none() {
        err.push("Missing statement date");
    };
    if balance.is_none() {
        err.push("Missing balance");
    };
    if ! err.is_empty() {
        return Err(anyhow::anyhow!(err.join(", ")));
    }
    
    Ok(AccountBalance{ 
        row_id: None,
        date : NaiveDate::parse_from_str(date.unwrap(), "%d/%m/%Y")? , 
        balance_euro : balance.unwrap().replace(",", ".").parse::<OrderedFloat<f64>>()?
    })
}


pub fn parse_csv<P: AsRef<Path>>(csv_path: P) -> anyhow::Result<BankingStatement> {
    let mut stats: HashMap<String,String> = HashMap::new();
    let mut activities: HashSet<AccountActivity> = HashSet::new();

    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .delimiter(b';')
        .from_path(csv_path)?;        

    let data = reader.records()
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap());

    for record in data {
        // Balance
        if  record.len() == 2 as usize {
            match (record.get(0), record.get(1)) {
                (Some(header), Some(value)) => {
                    let header = str::trim(header);
                    let value = str::trim(value);
                    if header == "Date" || header == "Solde (EUROS)" {
                        stats.insert(header.to_string(), value.to_string());
                    }
                },
                _ => continue
            }
        } 
        // Activity
        else if record.len() > 2 as usize {            
            match (record.get(0), record.get(1), record.get(2)) {
                (Some(date), Some(statement), Some(amount)) 
                        if date == "Date" || statement.contains("Montant") || amount.contains("Montant") => {
                    continue;
                },
                (Some(date), Some(statement), Some(amount)) => {                    
                    let date = NaiveDate::parse_from_str(date.trim(), "%d/%m/%Y")?;
                    let statement = statement.trim();
                    let amount = amount.replace(",", ".").parse::<f64>()?;

                    activities.insert(AccountActivity {
                        row_id: None,
                        date,
                        statement: statement.to_string(),
                        amount: OrderedFloat(amount),
                        tag_pattern_id: None
                    });
                },
                _ => continue
            }
        }

    }
   
    let balance = get_balance(stats)?;

    Ok(BankingStatement { row_id: None, balance, activities})
}




#[test]
fn test() -> anyhow::Result<()> {

    // Tests run at the project level
    let result = parse_csv("./data/input01.csv")?;
    let expected_activity = AccountActivity {
        row_id: None,
        date : NaiveDate::parse_from_str("12/03/2021", "%d/%m/%Y")?,
        statement : "BUY SOMETHING 03".to_string(),
        amount : OrderedFloat(-15.00),
        tag_pattern_id: None
    };

    assert_eq!(result.balance.balance_euro, 187.77, "Wrong balance found");
    assert_eq!(result.activities.len(), 9, "Wrong count of activities");
    assert!(result.activities.contains(&expected_activity),"Expected activity not found");

    Ok(())

}