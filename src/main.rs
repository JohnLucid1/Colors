use qrcode::QrCode;
use std::env;


fn main() {
    let args: Vec<String> =  env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }

    let result = create_qr(&args[1]).unwrap();
    println!("{}", &result);

}

fn create_qr(url: &String) -> Result<String, &'static str>{

    if url.len() < 1{
        return Err("Not Enough Arguments");
    }

    let code = QrCode::new(url.as_bytes()).unwrap();
    let qrcode = code.render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();

    Ok(qrcode)
    
}