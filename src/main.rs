extern crate rocket;
use rocket::catch;
use rocket::get;
use rocket::launch;
use rocket::routes;
use rocket::{Build, Rocket};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
*Author Gaurav Sablok
*Universitat Potsdam
*Date 2024-1-17

 A rocket framework API for the OpenSNP portal aimed for resequencing
 of the genome scans for the personalized genome science. The REST API
 reads the personal genome file and then scans the personal genome for
 REST API. I previously used Actix and Leptos for the web in RUST but
 today i build this using the rocket web and it is a RESTAPI for personal
 genomes.

 A complete deployable RUST API for any pangenome or personal genome snp.

*/

// structs for holding th personal genome and the pangenome snps for the rest api

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct PersonalGenome {
    pub rsid: String,
    pub chromosome: String,
    pub position: usize,
    pub genotype: String,
    pub indivisualsnp: String,
    pub humansnp: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct PersonalFasta {
    pub header: String,
    pub sequence: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ParseStruct {
    pub rsid: String,
    pub chromosome: String,
    pub position: usize,
    pub genotype: String,
    pub indiviualsnp: String,
    pub humansnp: String,
    pub position10upstream: String,
    pub position50upstream: String,
    pub position100upstream: String,
    pub position150upstream: String,
    pub position200upstream: String,
    pub position10downstream: String,
    pub position50downstream: String,
    pub position100downstream: String,
    pub position150downstream: String,
    pub position200downstream: String,
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![personalgenome])
}
/*
 * roter function for the personal genome snp profile.
 * */
#[get("/<rsidpath>")]
fn personalgenome(rsidpath: &str) -> String {
    let personalgenomeopen = File::open("personalgenome.file").expect("file not found");
    let personalgenomeread = BufReader::new(personalgenomeopen);
    let mut personalgenome: Vec<PersonalGenome> = Vec::new();

    for i in personalgenomeread.lines() {
        let line = i.expect("line not found");
        if line.starts_with("#") {
          continue
        } else if !line.starts_with("#"){
        let personalmutable: Vec<_> = line.split("\t").collect::<Vec<_>>();
        let genotypesplit: Vec<_> = personalmutable[3].chars().collect::<Vec<_>>();
        personalgenome.push(PersonalGenome {
            rsid: personalmutable[0].to_string(),
            chromosome: personalmutable[1].to_string(),
            position: personalmutable[2].to_string().parse::<usize>().unwrap(),
            genotype: personalmutable[3].to_string(),
            indivisualsnp: genotypesplit[0].to_string(),
            humansnp: genotypesplit[1].to_string(),
        });
    }
       }

    let mut parse_personal_genome: Vec<ParseStruct> = Vec::new();
    let parse_personal_fasta: Vec<PersonalFasta> = personal_genome().unwrap();

    for i in personalgenome.iter() {
        for j in parse_personal_fasta.iter() {
            if i.chromosome == j.header {
                parse_personal_genome.push(ParseStruct {
                    rsid: i.rsid.clone(),
                    chromosome: i.chromosome.clone(),
                    position: i.position,
                    genotype: i.genotype.clone(),
                    indiviualsnp: i.indivisualsnp.clone(),
                    humansnp: i.humansnp.clone(),
                    position10upstream: j.sequence[i.position - 10..i.position].to_string(),
                    position50upstream: j.sequence[i.position - 50..i.position].to_string(),
                    position100upstream: j.sequence[i.position - 100..i.position].to_string(),
                    position150upstream: j.sequence[i.position - 150..i.position].to_string(),
                    position200upstream: j.sequence[i.position - 200..i.position].to_string(),
                    position10downstream: j.sequence[i.position..i.position + 10].to_string(),
                    position50downstream: j.sequence[i.position..i.position + 50].to_string(),
                    position100downstream: j.sequence[i.position..i.position + 100].to_string(),
                    position150downstream: j.sequence[i.position..i.position + 150].to_string(),
                    position200downstream: j.sequence[i.position..i.position + 200].to_string(),
                });
            }
        }
    }

    let mut finalparsestruct: Vec<ParseStruct> = Vec::new();

    for i in parse_personal_genome.iter() {
        if rsidpath == i.rsid {
            finalparsestruct.push(ParseStruct {
                rsid: i.rsid.clone(),
                chromosome: i.chromosome.clone(),
                position: i.position,
                genotype: i.genotype.clone(),
                indiviualsnp: i.indiviualsnp.clone(),
                humansnp: i.humansnp.clone(),
                position10upstream: i.position10upstream.clone(),
                position50upstream: i.position50upstream.clone(),
                position100upstream: i.position100upstream.clone(),
                position150upstream: i.position150upstream.clone(),
                position200upstream: i.position200upstream.clone(),
                position10downstream: i.position10downstream.clone(),
                position50downstream: i.position50downstream.clone(),
                position100downstream: i.position100downstream.clone(),
                position150downstream: i.position150downstream.clone(),
                position200downstream: i.position200downstream.clone(),
            });
        }
    }

    for i in finalparsestruct.iter() {
        println!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            i.rsid,
            i.chromosome,
            i.position,
            i.genotype,
            i.indiviualsnp,
            i.humansnp,
            i.position10upstream,
            i.position50upstream,
            i.position100upstream,
            i.position150upstream,
            i.position200upstream,
            i.position10downstream,
            i.position50downstream,
            i.position100downstream,
            i.position150downstream,
            i.position200downstream
        );
    }
    format!("The resturn values for the this pangenome snp or the personal genome snp {} is as follows:", rsidpath)
}

fn personal_genome() -> Result<Vec<PersonalFasta>, Box<dyn Error>> {
    let personalfasta = File::open("genome.fasta").expect("file not found");
    let personalread = BufReader::new(personalfasta);
    let mut header: Vec<String> = Vec::new();
    let mut sequence: Vec<String> = Vec::new();
    for i in personalread.lines() {
        let line = i.expect("line not found");
        if line.starts_with(">") {
            header.push(line.replace(">", "").replace("chr", ""));
        } else if !line.starts_with(">") {
            sequence.push(line);
        }
    }
    let mut personalstruct: Vec<PersonalFasta> = Vec::new();
    for i in 0..header.len() {
        personalstruct.push(PersonalFasta {
            header: header[i].clone(),
            sequence: sequence[i].clone(),
        });
    }

    Ok(personalstruct)
}

#[catch(404)]
fn snp_absence() -> String {
    String::from("SNP in the observed pangenome or the personal genome is not present")
}
