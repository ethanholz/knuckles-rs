use knuckles_parse::records;

#[cfg(not(feature = "parallel"))]
use knuckles_parse::pdbreader_single;

#[cfg(feature = "parallel")]
use knuckles_parse::pdbreader_parallel as pdbreader;

fn main() {
    let contents =
        std::fs::read_to_string("tests/4pth.pdb").expect("Something went wrong reading the file");
    let mut times = Vec::new();
    for i in 0..50 {
        let start = std::time::Instant::now();
        #[cfg(feature = "parallel")]
        let _ = pdbreader(&contents);
        #[cfg(not(feature = "parallel"))]
        let _ = pdbreader_single(&contents);

        // let _ = pdbreader_single(&contents);
        // let _ = pdbreader("tests/4pth.pdb");
        let end = std::time::Instant::now();
        times.push(end - start);
        println!("Run {}: Time: {:?}", i, end - start);
    }
    let sum: std::time::Duration = times.iter().sum();
    let avg = sum / times.len() as u32;
    println!("Total time: {:?}", sum);
    println!("Average time: {:?}", avg);
    #[cfg(feature = "parallel")]
    let out = pdbreader(&contents);

    #[cfg(not(feature = "parallel"))]
    let out = pdbreader_single(&contents);

    // println!("{:?}", out[0]);
    out.iter()
        // .filter(|&record| matches!(record, records::Record::DBRef(_)))
        .filter(|&record| matches!(record, records::Record::Atom(_)))
        .take(10)
        .for_each(|record| {
            println!("{}", record);
        });
}
