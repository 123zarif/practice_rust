use crate::models::events::Event;
use mongodb::Collection;

pub async fn add_event(collection: &Collection<Event>) -> mongodb::error::Result<()> {
    let mut name = String::new();
    let mut event_type = String::new();
    let mut start_time = String::new();
    let mut end_time = String::new();
    let mut days: Vec<u8> = vec![];

    println!("Enter event name:");
    std::io::stdin()
        .read_line(&mut name)
        .expect("Why can't you write name man... Its just a string- Thats embracing!");

    println!("Enter event type:");
    std::io::stdin()
        .read_line(&mut event_type)
        .expect("Why can't you write name man... Its just a string- Thats embracing!");

    println!("Enter start time (HH:MM):");
    std::io::stdin()
        .read_line(&mut start_time)
        .expect("Invalid time");

    println!("Enter end time (HH:MM):");
    std::io::stdin()
        .read_line(&mut end_time)
        .expect("Invalid time");

    println!("1 - Sunday");
    println!("2 - Monday");
    println!("3 - Tuesday");
    println!("4 - Wednesday");
    println!("5 - Thursday");
    println!("6 - Friday");
    println!("7 - Saturday");

    println!("Press (e) to finish");

    loop {
        println!("Enter Days:");

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Invalid Day!");

        if input.trim() == "e" {
            break;
        }

        let number: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong Input!");
                break;
            }
        };

        if number > 0 && number < 8 {
            days.push(number);
        } else {
            println!("Wrong Number Entered enter (1-7)!")
        }
    }

    Event::add_event(
        &collection,
        name.trim().to_string(),
        start_time.trim().to_string(),
        end_time.trim().to_string(),
        days,
        event_type.trim().to_string(),
    )
    .await?;

    Ok(())
}
