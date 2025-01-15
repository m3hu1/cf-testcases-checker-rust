use clap::Parser;
use colored::Colorize;
use reqwest;
use scraper::{Html, Selector};
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tokio;

#[derive(Parser)]
#[command(name = "cfgo")]
struct Cli {
    problem_id: String,

    #[arg(short, long)]
    code_dir: Option<PathBuf>,
}

struct TestCase {
    input: String,
    output: String,
}

async fn fetch_test_cases(problem_id: &str) -> Result<Vec<TestCase>, Box<dyn std::error::Error>> {
    let (contest_id, problem_index) = if problem_id.chars().last().unwrap().is_alphabetic() {
        let index = problem_id.len() - 1;
        (&problem_id[..index], &problem_id[index..])
    } else {
        panic!("Invalid problem ID format");
    };

    // one of these URLs should work
    let urls = vec![
        format!(
            "https://codeforces.com/contest/{}/problem/{}",
            contest_id, problem_index
        ),
        format!(
            "https://codeforces.com/problemset/problem/{}/{}",
            contest_id, problem_index
        ),
    ];

    let client = reqwest::Client::new();
    let mut last_error = None;

    // dono try karo
    for url in urls {
        match client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    let response_text = response.text().await?;
                    let document = Html::parse_document(&response_text);

                    let input_selector = Selector::parse("div.input pre").unwrap();
                    let output_selector = Selector::parse("div.output pre").unwrap();

                    let inputs: Vec<_> = document
                        .select(&input_selector)
                        .map(|element| element.text().collect::<Vec<_>>().join("\n"))
                        .collect();

                    let outputs: Vec<_> = document
                        .select(&output_selector)
                        .map(|element| element.text().collect::<Vec<_>>().join("\n"))
                        .collect();

                    // mil gaye test cases
                    if !inputs.is_empty() && !outputs.is_empty() {
                        // i + o = t
                        let test_cases: Vec<TestCase> = inputs
                            .into_iter()
                            .zip(outputs.into_iter())
                            .map(|(input, output)| TestCase {
                                input: input.trim().to_string(),
                                output: output.trim().to_string(),
                            })
                            .collect();

                        return Ok(test_cases);
                    }
                }
            }
            Err(e) => {
                last_error = Some(e);
                continue;
            }
        }
    }

    // kuch nahi chala
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!(
            "Could not fetch test cases for problem {}{}. Last error: {:?}",
            contest_id, problem_index, last_error
        ),
    )))
}

async fn run_tests(code_path: &PathBuf, test_cases: &[TestCase]) {
    println!("\n{}", "Running Tests...".blue().bold());
    println!("{}", "=================".blue().bold());

    let mut all_passed = true;

    for (i, test) in test_cases.iter().enumerate() {
        let passed = run_test_case(code_path, &test.input, &test.output, i + 1).await;

        if !passed {
            all_passed = false;
        }
    }

    println!(
        "\n{}",
        if all_passed {
            "All test cases passed! Kardo submit bhai ✨".green().bold()
        } else {
            "Some test cases failed ✗".red().bold()
        }
    );
}

async fn run_test_case(
    code_path: &PathBuf,
    input: &str,
    expected_output: &str,
    test_num: usize,
) -> bool {
    println!("\n{}:", format!("Test Case {}", test_num).blue().bold());
    println!("{}", "Input:".yellow());
    println!("{}", input);

    let main_file = code_path.join("main.go");
    if !main_file.exists() {
        println!(
            "{}",
            format!("Error: main.go not found in {}", code_path.display())
                .red()
                .bold()
        );
        return false;
    }

    let mut child = Command::new("go")
        .arg("run")
        .arg(&main_file)
        .current_dir(code_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start Go program");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(input.trim().as_bytes())
        .expect("Failed to write to stdin");
    stdin.flush().expect("Failed to flush stdin");

    let output = child.wait_with_output().expect("Failed to read stdout");
    let actual_output = String::from_utf8_lossy(&output.stdout).trim().to_string();

    println!("{}", "Your Output:".yellow());
    println!("{}", actual_output);
    println!("{}", "Expected Output:".yellow());
    println!("{}", expected_output.trim());

    let passed = actual_output == expected_output.trim();
    if passed {
        println!("{}", "Status: PASSED ✓".green().bold());
    } else {
        println!("{}", "Status: FAILED ✗".red().bold());
    }

    passed
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let home_dir = env::var("HOME").expect("Could not find HOME directory");
    let default_code_dir = PathBuf::from(home_dir)
        .join("Documents")
        .join("code")
        .join("golang");
    let code_dir = cli.code_dir.unwrap_or(default_code_dir);
    let code_path = code_dir.join(&cli.problem_id);

    println!(
        "{}",
        "Codeforces Test Runner by Mehul Pathak".green().bold()
    );
    println!("Problem: {}", cli.problem_id.blue().bold());
    println!("Fetching test cases...");

    match fetch_test_cases(&cli.problem_id).await {
        Ok(test_cases) => {
            if test_cases.is_empty() {
                println!("{}", "No test cases found!".red().bold());
                return Ok(());
            }
            println!("Found {} test cases", test_cases.len().to_string().green());
            run_tests(&code_path, &test_cases).await;
        }
        Err(e) => {
            println!(
                "{}",
                format!("Error fetching test cases: {}", e).red().bold()
            );
        }
    }

    Ok(())
}
