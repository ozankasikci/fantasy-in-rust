use clap::{App, Arg, ArgMatches};
use rust_music_theory::chord::Chord;
use rust_music_theory::note::Notes;
use rust_music_theory::scale::Scale;

const AVAILABLE_SCALES: [&str; 9] = [
    "Major|Ionian",
    "Minor|Aeolian",
    "Dorian",
    "Phrygian",
    "Lydian",
    "Mixolydian",
    "Locrian",
    "Harmonic Minor",
    "Melodic Minor",
];

const AVAILABLE_CHORDS: [&str; 22] = [
    "Major Triad",
    "Minor Triad",
    "Suspended2 Triad",
    "Suspended4 Triad",
    "Augmented Triad",
    "Diminished Triad",
    "Major Seventh",
    "Minor Seventh",
    "Augmented Seventh",
    "Augmented Major Seventh",
    "Diminished Seventh",
    "Half Diminished Seventh",
    "Minor Major Seventh",
    "Dominant Seventh",
    "Dominant Ninth",
    "Major Ninth",
    "Dominant Eleventh",
    "Major Eleventh",
    "Minor Eleventh",
    "Dominant Thirteenth",
    "Major Thirteenth",
    "Minor Thirteenth",
];

fn scale_command(scale_matches: &ArgMatches) {
    match scale_matches.subcommand() {
        ("list", _) => {
            println!("Available Scales:");
            for scale in &AVAILABLE_SCALES {
                println!(" - {}", scale);
            }
        }
        _ => {
            let scale_args = scale_matches
                .values_of("args")
                .unwrap()
                .collect::<Vec<_>>()
                .join(" ");

            let scale = Scale::from_regex(&scale_args).unwrap();
            scale.print_notes();
        }
    }
}

fn chord_command(chord_matches: &ArgMatches) {
    match chord_matches.subcommand() {
        ("list", _) => {
            println!("Available chords:");
            for chord in &AVAILABLE_CHORDS {
                println!(" - {}", chord);
            }
        }
        _ => {
            let chord_args = chord_matches
                .values_of("args")
                .unwrap()
                .collect::<Vec<_>>()
                .join(" ");

            let chord = Chord::from_regex(&chord_args).unwrap();
            chord.print_notes();
        }
    }
}

fn main() {
    {
        use rust_music_theory::{
            chord::{Number::*, Quality::*, *},
            note::PitchClass::*,
        };
        println!("{:?}", Chord::with_inversion(C, Major, Triad, 0).notes());
        println!("{:?}", Chord::with_inversion(C, Major, Triad, 1).notes());
        println!("{:?}", Chord::with_inversion(C, Major, Triad, 2).notes());
        println!("{:?}", Chord::with_inversion(C, Major, Triad, 3).notes());
        println!("{:?}", Chord::with_inversion(C, Major, Triad, 4).notes());
        println!();
        println!("{:?}", Chord::with_inversion(C, Major, Seventh, 0).notes());
        println!("{:?}", Chord::with_inversion(C, Major, Seventh, 1).notes());
        println!("{:?}", Chord::with_inversion(C, Major, Seventh, 2).notes());
        println!("{:?}", Chord::with_inversion(C, Major, Seventh, 3).notes());
        println!();
        println!("{:?}", Chord::with_inversion(G, Major, Seventh, 0).notes());
        println!("{:?}", Chord::with_inversion(G, Major, Seventh, 1).notes());
        println!("{:?}", Chord::with_inversion(G, Major, Seventh, 2).notes());
        println!("{:?}", Chord::with_inversion(G, Major, Seventh, 3).notes());
    }

    let matches = App::new("RustMusicTheory")
        .version("0.1")
        .author("Ozan Kaşıkçı")
        .about("A music theory guide")
        .subcommand(
            App::new("scale")
                .about("Provides information for the specified scale")
                .subcommand(App::new("list").about("Prints out the available scales"))
                .arg(
                    Arg::with_name("args")
                        .help("scale args, examples:\nC melodic minor\nD# dorian")
                        .multiple(true),
                ),
        )
        .subcommand(
            App::new("chord")
                .about("Provides information for the specified chord")
                .subcommand(App::new("list").about("Prints out the available chords"))
                .arg(
                    Arg::with_name("args")
                        .help("chord args, examples:\nC minor\nAb augmented major seventh")
                        .multiple(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("scale", Some(scale_matches)) => {
            scale_command(scale_matches);
        }

        ("chord", Some(chord_matches)) => {
            chord_command(chord_matches);
        }

        _ => println!("Please use the help command to see the available commands"),
    }
}
