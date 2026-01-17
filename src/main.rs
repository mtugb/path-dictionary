#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    name: String,
}
fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println("Hello {}!", args.name);
    }
}
