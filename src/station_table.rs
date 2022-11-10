use crate::station::Station;
use std::io::{self, Write};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

enum BikeStatus {
    AVAILABLE,
    WARNING,
    EMPTY,
}

/// Resolve station's bike status.
///
/// If there's one or zero bikes status is EMPTY,
/// if there's five or less bikes status is WARNING
/// otherwise status is AVAILABLE
///  
/// examples:
///
/// resolve_bike_status(1) -> BikeStatus::EMPTY
/// resolve_bike_status(4) -> BikeStatus::WARNING
/// resolve_bike_status(5) -> BikeStatus::AVAILABLE
///
fn resolve_bike_status(bike_count: u32) -> BikeStatus {
    match bike_count {
        i if i < 2 => BikeStatus::EMPTY,
        i if i < 5 => BikeStatus::WARNING,
        _ => BikeStatus::AVAILABLE,
    }
}

/// Resolve color by BIKESTATUS.
///
/// examples:
///
/// resolve_color(BikeStatus::EMPTY) -> Color::Red;
/// resolve_color(BikeStatus::WARNING) -> Color::Yellow;
/// resolve_color(BikeStatus::AVAILABLE) -> Color::White;
///
fn resolve_color(status: BikeStatus) -> Color {
    match status {
        BikeStatus::EMPTY => Color::Red,
        BikeStatus::WARNING => Color::Yellow,
        BikeStatus::AVAILABLE => Color::White,
    }
}

fn print_headers() {
    let headers = ["Etäisyys", "Nimi", "Pyöriä/paikkoja"];
    let [distance, name, bikes] = headers;

    println!("\n{distance}\t{name}\t\t\t\t{bikes}");

    let char_count = headers.join("").len() + (8 * 5);
    for _n in 0..char_count {
        print!("-");
    }

    println!("");
}

fn print_rows(stations: Vec<Station>) -> io::Result<()> {
    let writer = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = writer.buffer();

    for station in stations {
        let Station {
            distance,
            name,
            bikes_available,
            spaces_available,
            ..
        } = station;

        let bike_status = resolve_bike_status(bikes_available);
        let bikes_color = resolve_color(bike_status);

        write!(&mut buffer, "{distance}m\t\t{name}\t\t")?;

        if name.len() < 16 {
            write!(&mut buffer, "\t")?;
        }

        buffer.set_color(ColorSpec::new().set_fg(Some(bikes_color)))?;
        write!(&mut buffer, "{bikes_available}")?;

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        write!(&mut buffer, "/{}", bikes_available + spaces_available)?;
        writeln!(&mut buffer, "")?;
    }

    writer.print(&buffer)
}

pub fn print_table(stations: Vec<Station>) {
    print_headers();
    let result = print_rows(stations);

    match result {
        Ok(_result) => (),
        Err(err) => println!("Whoopsie! {}", err),
    }
}
