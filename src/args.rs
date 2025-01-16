use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]
pub struct PersonalArgs {
    /// please provide the path to the graph file
    pub personalgenomescan: String,
    /// please provide the number of the upperbase
    pub upper: Option<usize>,
    /// please provide the number of the lowebase
    pub lower: Option<usize>,
}
