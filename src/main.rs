fn main() {
    let img = image::open("src/wallpaper.png").unwrap();

    println!("{}", img.to_rgb8());
}