use csv::{Reader, ReaderBuilder};
use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};
use plotpy::{generate3d, Plot, Surface};
use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::prelude::{BLACK, WHITE};
use plotters::series::LineSeries;
use root_io::RootFile;
use std::error::Error;
use std::fmt::Debug;
use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nom::number::complete::{be_f32, be_i32, be_u32};

#[tokio::main]
async fn main() -> Result<(), PlatformError> {
    // Sanity check that program ran
    println!("Hello, world!");

    // Call your function and match the result
    match read_csv_data() {
        Ok(()) => {
            // Handle the successful case
            println!("Read CSV Data function executed successfully!");
        }
        Err(err) => {
            // Handle the error case
            eprintln!("Error: {}", err);
        }
    }

    // Call your function and match the result
    match read_csv_file() {
        Ok(()) => {
            // Handle the successful case
            println!("Read CSV file function executed successfully!");
        }
        Err(err) => {
            // Handle the error case
            eprintln!("Error: {}", err);
        }
    }

    // Call your function and match the result
    match root_file_parser().await {
        Ok(()) => {
            // Handle the successful case
            println!("root file parser function executed successfully!");
        }
        Err(err) => {
            // Handle the error case
            eprintln!("Error: {}", err);
        }
    }

    // // Call your function and match the result
    match plotters() {
        Ok(()) => {
            // Handle the successful case
            println!("Plotters function executed successfully!");
        }
        Err(err) => {
            // Handle the error case
            eprintln!("Error: {}", err);
        }
    }

    // Call your function and match the result
    match plotpy() {
        Ok(()) => {
            // Handle the successful case
            println!("Plotpy function executed successfully!");
        }
        Err(err) => {
            // Handle the error case
            eprintln!("Error: {}", err);
        }
    }

    let main_window = WindowDesc::new(ui_builder());
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}

// Reads static csv data
fn read_csv_data() -> Result<(), Box<dyn Error>> {
    let data = "\
city;country;pop
Boston;United States;4628910
";

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(data.as_bytes());

    if let Some(result) = rdr.records().next() {
        let record = result?;
        assert_eq!(record, vec!["Boston", "United States", "4628910"]);
        Ok(())
    } else {
        Err(From::from("expected at least one record but got none"))
    }
}

// Reads CSV file from a given static path
fn read_csv_file() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(Path::new("./src/data.csv"))?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn plotters() -> Result<(), Box<dyn Error>> {
    let drawing_area = BitMapBackend::new("src/1.0.png", (1024, 768)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(0..100, 0..100)
        .unwrap();

    chart
        .draw_series(LineSeries::new((0..100).map(|x| (x, 100 - x)), &BLACK))
        .unwrap();

    Ok(())
}

fn plotpy() -> Result<(), Box<dyn Error>> {
    let mut surface = Surface::new();

    surface
        .set_with_wireframe(true)
        .set_colormap_name("Pastel1")
        .set_with_colorbar(true)
        .set_colorbar_label("temperature")
        .set_line_color("#1862ab")
        .set_line_style(":")
        .set_line_width(0.75);

    // draw surface
    let n = 9;
    let (x, y, z) = generate3d(-2.0, 2.0, -2.0, 2.0, n, n, |x, y| x * x + y * y);

    surface.draw(&x, &y, &z);

    let mut plot = Plot::new();
    plot.add(&surface);

    // save figure
    plot.save("./src/plotpy.svg")?;

    Ok(())
}

fn ui_builder() -> impl Widget<u32> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("increment")
        .on_click(|_ctx, data, _env| *data += 1)
        .padding(5.0);

    Flex::column().with_child(label).with_child(button)
}

async fn root_file_parser() -> Result<(), Box<dyn Error>> {
    // let working_file_items = RootFile::new(Path::new("./src/lem23_his_0001.root")).await.unwrap();
    // let working_file_items = RootFile::new(Path::new("./src/simple.root")).await.unwrap();
    let psi_file_items = RootFile::new(Path::new("./src/lem23_his_0001.root"))
        .await
        .unwrap();

    // let tree = working_file_items.items()[0].as_tree().await.unwrap();
    let tree_psi = psi_file_items.items()[0].as_tree().await.unwrap();

    //
    // //let tree_name = &tree.branch_by_name("one").unwrap().name;
    let tree_name_psi = &tree_psi.branch_by_name("one").unwrap().name;

    Ok(())
}
