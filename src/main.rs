use mpris::PlayerFinder;
use urlencoding::decode;

fn main() {
    let finder = PlayerFinder::new().unwrap();
    let player = finder.find_active().unwrap();
    let metadata = player.get_metadata().unwrap();
    println!("Artist: {}, Track Name: {}, URL: {}", 
        metadata.artists().unwrap()[0],
        metadata.title().unwrap(),
        decode(metadata.url().unwrap()).expect("FromUTF8Error"))
}


