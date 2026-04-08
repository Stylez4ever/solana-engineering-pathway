#[derive(Debug, PartialEq, Eq)]
enum ChannelType {
    Comedy,
    News,
    ProgrammingTutorials,
}


#[derive(Debug)]
struct TVChannel {
    name: String,
    channel_type: ChannelType,
}



fn pratice_16() {
    let channels = [
        TVChannel {
            name: String::from("ETV"),
            channel_type: ChannelType::Comedy,
        },

        TVChannel {
            name: String::from("RustLive"),
            channel_type: ChannelType::ProgrammingTutorials,
        },

        TVChannel {
            name: String::from("SABC NEWS"),
            channel_type: ChannelType::News,
        },

        TVChannel {
            name: String::from("RustTV"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
    ];


    // This will return true if all of the elements satisfy this criteria (channel.channel_type == ChannelType::ProgrammingTutorials)
    let all_are_rust = channels.iter().all(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
    println!("{:?}", all_are_rust);

    // This will return true if any of the elements satisfy this criteria
    let any_are_rust = channels.iter().any(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
    println!("{:?}", any_are_rust);
}
