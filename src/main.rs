use clap::Parser;

mod calc;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The number to calculate the number of lattice points with a radius lower than or equal to sqrt of this number
    #[arg(short, long)]
    num: u32
}

fn main() {
    let args = Args::parse();
    let num = args.num;
    let lattice_points = calc::num_latice_points_in_circle_sqrtn(num);
    let pi_approximation: f64 = calc::pi_approximation_sqrtn(num);
    println!("The number of lattice points with a radius lower than sqrt({}) is {}", num, lattice_points);
    println!("The approximation of pi is {}", pi_approximation);

}
