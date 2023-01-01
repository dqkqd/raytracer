use clap::Parser;
use raytracer::render_image;

#[derive(Parser)]
struct Cli {
    yaml: std::path::PathBuf,
    image_output: std::path::PathBuf,
}

impl Cli {
    fn run(&self) {
        render_image(&self.yaml, &self.image_output);
    }
}

fn main() {
    let cli = Cli::parse();
    cli.run();
}
