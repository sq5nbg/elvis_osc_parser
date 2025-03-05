
use std::{
    //env,
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

use clipboard_win::{formats, set_clipboard};
use dirs::desktop_dir;
use rfd::{FileDialog, MessageDialog};

fn starts_with_number(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let first = s.chars().next().unwrap();
    first.is_numeric() || (first == '-' && s.len() > 1 && s.chars().nth(1).unwrap().is_numeric())
}

// fn problem_with_file() {
//     println!("Plik jest niewłaściwy!");
//     exit(1);
// }

fn get_value(s: &str) -> Option<f64> {
    if starts_with_number(s) {
        let res = s.trim().replace(',', ".").parse::<f64>();
        match res {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    } else {
        None
    }
}

fn main() {
    //let mut argv = env::args();
    println!("NI Elvis II+. Konwertuje dane z pliku .txt pochodzącego od przyrządu wirtualnego Oscilloscope. Dane zapisane powinny być albo rzeczywiste albo symulowane ale nie jednocześnie! Dane w postaci tablic numpy.array umieszczone są w schowku. Wczytaj do edytora przez: Ctrl + v. Wybierz plik .txt do odczytu w okienku dialogowym");

    let mut ch1_v: Vec<f64> = Vec::new();
    let mut ch2_v: Vec<f64> = Vec::new();
    let mut ch3_v: Vec<f64> = Vec::new();
    let mut ch4_v: Vec<f64> = Vec::new();

    //let file_name: String = argv.nth(1).expect("File not specified");

    let file_opt = FileDialog::new()
        .set_directory(desktop_dir().unwrap())
        .pick_file();

    let file_name = match file_opt {
        Some(filn) => filn,
        None => {
                MessageDialog::new()
                    .set_title("Nie wybrano pliku!")
                    .set_description("Nie wybrano pliku. Aby wczytać dane i umieścić je w postaci tablic numpy.array w schowku, musisz wybrać plik z danymi do odczytu.")
                    .set_level(rfd::MessageLevel::Warning)
                    .show();
                exit(-1);
        }
    };


    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut dt1: f64 = 0.0;
    let mut dt2: f64 = 0.0;
    let mut dt3: f64 = 0.0;
    let mut dt4: f64 = 0.0;

    let mut columns = 0;

    for (i, lin) in reader.lines().enumerate() {
        let lin = lin.unwrap();

        if i == 2 {
            let tokens: Vec<&str> = lin.split_whitespace().collect();
            match tokens.len() {
                3 => {
                    columns = 1;
                    dt1 = get_value(tokens[2]).unwrap();
                    println!("Jeden kanał danych, dt={dt1:e}");
                }
                4 => {
                    columns = 2;
                    dt1 = get_value(tokens[2]).unwrap();
                    dt2 = get_value(tokens[3]).unwrap();
                    println!("Dwa kanały danych, dt1={dt1:e}, dt2={dt2:e}");
                }
                6 => {
                    columns = 4;
                    dt1 = get_value(tokens[2]).unwrap();
                    dt2 = get_value(tokens[3]).unwrap();
                    dt3 = get_value(tokens[4]).unwrap();
                    dt4 = get_value(tokens[5]).unwrap();
                    println!(
                        "Cztery kanały danych, dt1={dt1:e}, dt2={dt2:e}, dt3={dt3:e}, dt4={dt4:e}"
                    );
                }
                _ => {
                    println!(
                        "Niespodziwany format pliku. Błąd.");
                    exit(1);
                }
            }
        }
        if i > 4 {
            //Normalne dane
            let tokens: Vec<&str> = lin.split_ascii_whitespace().collect();
            if let Some(s) = tokens.get(2) {
                if let Some(v) = get_value(s) {
                    ch1_v.push(v);
                }
            }
            if let Some(s) = tokens.get(5) {
                if let Some(v) = get_value(s) {
                    ch2_v.push(v);
                }
            }
            if let Some(s) = tokens.get(8) {
                if let Some(v) = get_value(s) {
                    ch3_v.push(v);
                }
            }
            if let Some(s) = tokens.get(11) {
                if let Some(v) = get_value(s) {
                    ch4_v.push(v);
                }
            }
        }
    }

    let time1_v: Vec<f64> = (0..ch1_v.len()).map(|i| (i as f64) * dt1).collect();
    let time2_v: Vec<f64> = (0..ch2_v.len()).map(|i| (i as f64) * dt2).collect();
    let time3_v: Vec<f64> = (0..ch3_v.len()).map(|i| (i as f64) * dt3).collect();
    let time4_v: Vec<f64> = (0..ch4_v.len()).map(|i| (i as f64) * dt4).collect();

    let mut text = String::new();
    if columns > 0 {
        text.push_str(&format!("time1_v = np.array({time1_v:?})\r\n"));
        text.push_str(&format!("ch1_v = np.array({ch1_v:?})\r\n"));
    };

    if columns > 1 {
        text.push_str(&format!("time2_v = np.array({time2_v:?})\r\n"));
        text.push_str(&format!("ch2_v = np.array({ch2_v:?})\r\n"));
    };

    if columns > 2 {
        text.push_str(&format!("time3_v = np.array({time3_v:?})\r\n"));
        text.push_str(&format!("ch3_v = np.array({ch3_v:?})\r\n"));
    };

    if columns > 3 {
        text.push_str(&format!("time4_v = np.array({time4_v:?})\r\n"));
        text.push_str(&format!("ch4_v = np.array({ch4_v:?})\r\n"));
    };

    set_clipboard(formats::Unicode, text).expect("Cannot set clipboard");
    MessageDialog::new()
        .set_title("Dane umieszcono w schowku")
        .set_description("Wczytane dane zostały umieszczone w postaci tablic np.array w schowku. W edytorze, wklej ze schowka (Ctrl+v) w dogodnym miejscu.")
        .set_level(rfd::MessageLevel::Info)
        .show();
}
