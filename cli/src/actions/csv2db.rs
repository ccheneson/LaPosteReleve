use std::path::Path;

use crate::db::DBActions;
use crate::csv::dir::list_files;
use crate::csv::parsing::*;
use crate::models::{AccountActivity, AccountBalance, BankingStatement};

use crate::db::ArcMutDB;

pub fn csv2db<T: DBActions, P: AsRef<Path>>(dir_path: P, arc_db : ArcMutDB<T>) -> anyhow::Result<()> {
    //-----  Get all csv statements ------------
    let statement_files = list_files(
        dir_path.as_ref(), 
        Some("csv")
    )?;

    //-----  Parse csvs ------------
    let mut banking_statements: Vec<BankingStatement> = vec!();

    for csv_path in statement_files {
        let banking_statement = parse_csv(csv_path)?;
        banking_statements.push(banking_statement);
    }
    
    //------ Insert to DB ------------    
    let mut db = arc_db.lock().unwrap();
    db.create_table()?;

    let mut latest_balance : Option<AccountBalance> = None;

    for statement in banking_statements {
        let activities: Vec<AccountActivity> = statement.activities.into_iter().collect::<Vec<AccountActivity>>();
        
        // Keep the latest balance
        latest_balance = match latest_balance {
            Some(b) if b.date.lt(&statement.balance.date) => Some(statement.balance),
            None => Some(statement.balance),
            _ => latest_balance
        };

        //Insert all account activities
        db.insert_activities(&activities)?;
    }

    //Insert the latest / newest account balance
    db.insert_balance(latest_balance.expect("We should get a latest balance"))?;
 

    Ok(())
}