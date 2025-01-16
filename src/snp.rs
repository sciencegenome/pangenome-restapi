/*
 * holding all the structs in the separate files so that they
 * can be easily called as a reference call in the result.
 *
 * */
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct PersonalGenome {
   pub rsid: String,
   pub chromosome: usize,
   pub position: usize,
   pub genotype: String,
   pub indivisualsnp: String,
   pub humansnp: String,
}
