#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(self: &ChatMessage<DigitalContent>) {
        println!("Watching the {:?}", self.content)
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time (self: &ChatMessage<T>) -> String {
        self.time.clone()
        
    }
}

fn main() {
    let mut content_str = ChatMessage {
        content: "type shit",
        time: String::from("15:00"),
    };

    println!("{}", content_str.retrieve_time());


    let content_string = ChatMessage {
        content: String::from("fine shit"),
        time: String::from("15:02"),
    };
    println!("{}", content_string.retrieve_time());


    let content_dc = ChatMessage {
        content: DigitalContent::VideoFile,
        time: String::from("15:05"),
    };
    println!("{}", content_dc.retrieve_time());


    content_dc.consume_entertainment();
    

}




