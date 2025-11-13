use chrono::{DateTime, Utc};
use osapi::{Client, Week, Weekday};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("https://api.thisishyum.ru/schedule_api/tyumen/").with_college(1);

    let schedule = client
        .campus(1)
        .group(34)
        .schedules()
        .today()
        .send()
        .await?;

    for lesson in &schedule[0].lessons {
        println!(
            "Пара {}: {} - {}",
            lesson.order, lesson.start_time, lesson.end_time
        );
        println!("  Предмет: {}", lesson.title);
        println!("  Преподаватель: {}", lesson.teacher);
        println!("  Кабинет: {}", lesson.cabinet);
    }

    Ok(())
}
