use clap::Parser;
use sunny_and_sometimes_cloudy::api;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, default_value = "東京")]
    city: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let city = api::CityBuilder::new().city(&args.city).build();
    let responses = api::give_me_tenki(city)?;

    println!("★{}★ の天気予報(檜山沙耶は至高)", args.city);
    for response in responses {
        println!("{}", response);
    }
    Ok(())
}
