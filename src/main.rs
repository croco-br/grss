use exitfailure::ExitFailure;
use failure::ResultExt;
use std::convert::TryInto;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file '{:?}'", &args.path))?;

    let pb = indicatif::ProgressBar::new(content.lines().count().try_into().unwrap());
    let mut found = Vec::new();
    for line in content.lines() {
        if line.contains(&args.pattern) {
            found.push(line);
        }
        pb.inc(1);
    }
    pb.finish_with_message("done");
    found.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    println!("{:?}", found);
    Ok(())
}
