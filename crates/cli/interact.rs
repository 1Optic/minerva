use dialoguer::Select;
use tokio_postgres::Client;

use minerva::change::Change;
use minerva::error::{Error, RuntimeError};

pub async fn interact(client: &mut Client, change: &(dyn Change + Send)) -> Result<bool, Error> {
    let mut items: Vec<String> = vec!["Yes".to_string(), "No".to_string()];

    let information_options: Vec<String> = change
        .information_options()
        .iter()
        .map(|i| i.name())
        .collect();

    items.extend_from_slice(&information_options);

    let mut continuation_answer: Option<bool> = None;

    while continuation_answer.is_none() {
        let selection = Select::new()
            .with_prompt("Apply change?")
            .default(0)
            .items(&items)
            .interact()
            .map_err(|e| {
                Error::Runtime(RuntimeError {
                    msg: format!("Could not process input: {e}"),
                })
            })?;

        if selection > 1 {
            let information_option = &change.information_options()[selection - 2];

            let information = information_option.retrieve(client).await;

            println!();
            for line in information {
                println!("{line}");
            }
            println!();
        } else {
            continuation_answer = Some(selection == 0);
        }
    }

    Ok(continuation_answer.unwrap())
}
