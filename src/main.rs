mod args;
mod snp;
use crate::args::PersonalArgs;
use crate::snp::PersonalGenome;
use rocket::Request;
use rocket::get;
use rocket::routes;
use rocket::form;
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
*Author Gaurav Sablok
*Universitat Potsdam
*Date 2024-1-16

 A rocket framework API for the OpenSNP portal aimed for resequencing
 of the genome scans for the personalized genome science. The REST API
 reads the personal genome file and then scans the personal genome for
 REST API. I previously used Actix and Leptos for the web in RUST but
 today i build this using the rocket web and it is a RESTAPI for personal
 genomes.

*/

async fn main() {
    rocket::build()
        .mount("/", routes![personalgenome])
        .launch()
        .await;
}

/*
 * roter function for the personal genome snp profile.
 * */
#[get("/rsidpath")]
async fn personalgenome(rsidpath: &str) -> Result<String, Box<dyn Error>> {
    let personalgenomeopen = File::open("personalgenome.file").expect("file not found");
    let personalgenomeread = BufReader::new(personalgenomeopen);
    let mut personalgenome: Vec<PersonalGenome> = Vec::new();

    for i in personalgenomeread.lines() {
        let line = i.expect("line not found");
        let personalmutable: Vec<_> = line.split("\t").collect::<Vec<_>>();
        let genotypesplit: Vec<String, String> = personalmutable[3]
            .split(|c: char| !c.is_numeric())
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        personalgenome.push(PersonalGenome {
            rsid: personalmutable[0].to_string(),
            chromosome: personalmutable[1].parse::<usize>().unwrap(),
            position: personalmutable[2].parse::<usize>().unwrap(),
            genotype: personalmutable[3].to_string(),
            indivisualsnp: genotypesplit[0].to_string(),
            humansnp: genotypesplit[1].to_string(),
        });
    }

    for i in personalgenome.iter() {
        if rsidpath == i.rsid {
            println!("rsid queried:{}", i.rsid);
            println!("chromosome:{}", i.chromosome);
            println!("position:{}", i.position);
            println!("genotype:{}", i.genotype);
            println!("indiviualsnp:{}", i.indivisualsnp);
            println!("humansnp:{}", i.humansnp);
        }
    }

    Ok("personal genome string".to_string())
}
