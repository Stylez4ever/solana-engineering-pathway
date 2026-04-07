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



fn practice_15() {
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

    let good_channels: Vec<&TVChannel> = channels.iter()
    .filter(|channel| channel.channel_type == ChannelType::ProgrammingTutorials)
    .collect();

    let good_channels_string: Vec<String> = channels.iter()
    .filter(|channel| channel.channel_type == ChannelType::ProgrammingTutorials)
    .map(|channel| channel.name.clone())
    .collect();

    println!("{:?}", good_channels);

    println!("{:?}", good_channels_string);

    let good_channel = channels.iter()
    .find(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
    

    match good_channel {
        Some(channel) => println!("Great choice to watch {channel:?}"),
        None => println!("There was no Rust programming on the TV (literally and metaphorically)"),
    }




}
