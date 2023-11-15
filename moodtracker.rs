use chrono::{Local};
use std::fs::{OpenOptions};
use std::io::{Write};

trait InputFeedback {
    fn prompt_user(&mut self);
    fn display_feedback(&self) -> String;
}

struct Exercise {
    did_exercise: bool,
}

impl Exercise {
    fn new() -> Self {
        Exercise { did_exercise: false }
    }
}

enum Feelings {
    Happy,
    Sad,
    Anxious,
    Neutral,
}

struct Mood {
    feeling: Feelings,
}

impl Mood {
    fn new() -> Self {
        Mood { feeling: Feelings::Neutral }
    }
}

struct WorkCompleted {
    count: i32,
}

impl WorkCompleted {
    fn new() -> Self {
        WorkCompleted { count: 0 }
    }
}

impl InputFeedback for Exercise {
    fn prompt_user(&mut self) {
        println!("Did you exercise? (y/n)");
        let mut user_input = String::new();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        self.did_exercise = match user_input.trim().to_lowercase().as_str() {
            "y" | "yes" => true,
            "n" | "no" => false,
            _ => {
                println!("I don't know what that means so I'm assuming you didn't exercise.");
                false
            }
        };
    }
    fn display_feedback(&self) -> String {
        if self.did_exercise {
            "You did exercise.".to_string()
        } else {
            "You did not exercise.".to_string()
        }
    }
}

impl InputFeedback for Mood {
    fn prompt_user(&mut self) {
        println!("How are you feeling today?");
        println!("Options: Happy, Sad, Anxious, Neutral");

        let mut user_input = String::new();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        self.feeling = match user_input.trim().to_lowercase().as_str() {
            "happy" => Feelings::Happy,
            "sad" => Feelings::Sad,
            "anxious" => Feelings::Anxious,
            "neutral" => Feelings::Neutral,
            _ => {
                println!("I don't know what that means so I'm choosing Neutral.");
                Feelings::Neutral
            }
        };
    }
    fn display_feedback(&self) -> String {
        match self.feeling {
            Feelings::Happy => "You are feeling happy!".to_string(),
            Feelings::Sad => "You are feeling sad.".to_string(),
            Feelings::Anxious => "You are feeling anxious.".to_string(),
            Feelings::Neutral => "You are feeling neutral.".to_string(),
        }
    }
}

impl InputFeedback for WorkCompleted {
    fn prompt_user(&mut self) {
        println!("How many work tasks did you complete today?");
        let mut work_input = String::new();
        std::io::stdin()
            .read_line(&mut work_input)
            .expect("Failed to read line");

        match work_input.trim().parse::<i32>() {
            Ok(count) => {
                self.count = count;
            }
            Err(_) => {
                println!("I don't know what that means so I'm setting work tasks to 0.");
                self.count = 0;
            }
        }
    }
    fn display_feedback(&self) -> String {
        format!("You completed {} work tasks so far today.", self.count)
    }
}

fn append_string_to_file(feedback: &str) -> std::io::Result<()> {
    let today = Local::now();
    let date_str = today.format("%m_%d_%y").to_string();
    let filename = format!("file_{}.txt", date_str);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&filename)?;

    writeln!(file, "{}", feedback)?;
    Ok(())
}

fn main() {
    let mut mood = Mood::new();
    let mut work_completed = WorkCompleted::new();
    let mut exercise = Exercise::new();

    mood.prompt_user();
    work_completed.prompt_user();
    exercise.prompt_user();

    let feedback_mood=mood.display_feedback();
    let feedback_tasks=work_completed.display_feedback();
    let feedback_exercise=exercise.display_feedback();

    println!("\nUser Responses:");
    println!("{}\n{}\n{}", feedback_mood, feedback_tasks, feedback_exercise);

    let current_datetime = Local::now();
    let formatted_datetime = current_datetime.format("%H:%M:%S | %y-%m-%d").to_string();
    append_string_to_file(&formatted_datetime).expect("Failed to append exercise feedback to file");
    append_string_to_file(&feedback_mood).expect("Failed to append mood feedback to file");
    append_string_to_file(&feedback_tasks).expect("Failed to append tasks feedback to file");
    append_string_to_file(&feedback_exercise).expect("Failed to append exercise feedback to file");

}
