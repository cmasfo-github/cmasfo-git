use rand::seq::SliceRandom;
use rand::prelude::IteratorRandom;
use std::thread;
use std::time::{Duration, Instant, SystemTime};

pub(in super::super) fn silly_bg() {
    let mut log: Vec<(String, SystemTime)> = Vec::new();

    let responses = vec![
        "Hello there!",
        "How are you today?",
        "What brings you here?",
        "Nice weather we're having, isn't it?",
        "Have you tried turning it off and on again?",
    ];

    let mut last_interaction = Instant::now();
    let mut user_input = String::new();
    let mut timer = Duration::from_secs(5);

    println!("Hello! How can I assist you today?");

    loop {
        if !user_input.is_empty() {
            println!("You: {}", user_input);
            log.push((user_input.trim_end().to_string(), SystemTime::now()));
            last_interaction = Instant::now();
            timer = Duration::from_secs(0);
        } else if last_interaction.elapsed() >= timer {
            let context = get_context(&log);
            let response = responses.iter()
            .filter(|r| r.contains(&context))
            .choose(&mut rand::thread_rng()).unwrap();
            println!("{}", response);
            log.push((response.to_string(), SystemTime::now()));
            last_interaction = Instant::now();
            timer = Duration::from_secs(5);
        }

        if let Some(input) = read_user_input(&mut user_input) {
            log.push((input.trim_end().to_string(), SystemTime::now()));
            last_interaction = Instant::now();
            timer = Duration::from_secs(0);
        }

        thread::sleep(Duration::from_secs(1));

        if last_interaction.elapsed() >= Duration::from_secs(5) {
            user_input.clear();
            timer = Duration::from_secs(5);
        }
    }
}

fn read_user_input(user_input: &mut String) -> Option<String> {
    let mut buffer = String::new();
    if std::io::stdin().read_line(&mut buffer).unwrap_or(0) > 0 {
        user_input.push_str(&buffer.trim_end());
        Some(buffer)
    } else {
        None
    }
}

fn get_context(log: &[(String, SystemTime)]) -> String {
    const CONTEXT_DURATION: Duration = Duration::from_secs(30);
    let now = SystemTime::now();
    let context_log = log.iter().rev().take_while(|(_, time)| now.duration_since(*time).unwrap_or_default() <= CONTEXT_DURATION);
    let context_strings = context_log.map(|(string, _)| string.as_str());
    context_strings.collect::<Vec<_>>().join(" ")
}
