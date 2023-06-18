use async_openai::{types::CreateCompletionRequestArgs, Client};
use clap::Parser;
use std::error::Error;
use std::process::Command;

/// AI in your terminal
#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Args {
    /// Human language prompt (how many files in this folder?)
    prompt: String,
    /// Do you want to approve the command?
    #[arg(short, long)]
    approve: Option<String>,
    /// File to run commands on
    #[arg(short, long)]
    file: Option<String>,
}

async fn prompt_to_script(user_prompt: String) -> Result<String, Box<dyn Error>> {
    let final_prompt = format!("Act as a bash script writer from the human prompt. Return only the bash script I can add to the 'sh -c <bash script>' command without other explanations. Here is a prompt: {}", user_prompt);

    let client = Client::new();
    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt(final_prompt)
        .max_tokens(40_u16)
        .build()?;

    let response = client.completions().create(request).await?;
    let mut shell_script = String::new();

    for choice in response.choices {
        shell_script.push_str(&choice.text.trim());
    }

    Ok(shell_script)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // neca how many files in this folder?
    let args = Args::parse();
    let shell_script: String = prompt_to_script(args.prompt).await.unwrap();
    
    println!("Command: {}", shell_script);

    Command::new("sh")
        .arg("-c")
        .arg(shell_script)
        .spawn()?
        .wait_with_output()?
        .stdout;

    Ok(())
}
