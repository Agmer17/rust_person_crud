use std::io::Write;
use std::process::Command;

struct Person {
    name: String,
    age: u8,
    gender: String,
}

impl Person {
    fn print_data(&self) {
        println!(
            "name: {}, age: {}, gender: {}",
            self.name, self.age, self.gender
        );
    }
}

fn main() {
    let mut person_list: Vec<Person> = Vec::new();
    loop {
        clear_terminal();
        let mut user_choice = String::new();
        println!("{} crud person {}", "=".repeat(10), "=".repeat(10));
        println!(
            "\
        Pilihan :
        1. Tambah orang
        2. Hapus orang
        3. Edit orang
        4. Tampilkan orang
        5. keluar
        Harap masukan angka sesuai pilihan yang ada!
        "
        );
        println!("\nmasukan pilihan : ");
        std::io::stdin().read_line(&mut user_choice).unwrap();

        let user_choice = user_choice.trim_end();
        let user_choice = match parse_input_to_num(&user_choice) {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        match user_choice {
            1 => {
                add_person(&mut person_list);
                wait_for_enter();
            }

            2 => {
                println!("kamu memilih nomer 2");
                break;
            }

            3 => {
                println!("kamu memilih nomer 3");
                break;
            }

            4 => {
                print_all_persons(&person_list);
                wait_for_enter();
            }

            5 => {
                println!("kamu memilih nomer 5");
                break;
            }

            _ => {
                println!("Harap pilih satu sampai 5!");
                continue;
            }
        }
    }
}

fn parse_input_to_num(d: &str) -> Result<u8, &'static str> {
    d.parse::<u8>().map_err(|_| "input tidak valid")
}

fn ask_input(msg: &str) -> String {
    let mut s = String::new();
    print!("{}", msg);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut s)
        .expect("Gagal membaca input");

    s.trim().to_string()
}

fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        // Untuk Linux / Mac
        Command::new("clear").status().unwrap();
    }
}

fn wait_for_enter() {
    print!("Tekan [enter] untuk kembali");
    std::io::stdout().flush().unwrap();
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
}

fn save_person(p: Person, l: &mut Vec<Person>) {
    println!("berhasil menyimpan data! data yang disimpan : ");
    p.print_data();
    l.push(p);
}

fn add_person(list: &mut Vec<Person>) {
    clear_terminal();
    print!(
        "\n{} form penambahan orang baru {}\n",
        "-".repeat(10),
        "-".repeat(10)
    );

    let new_name = ask_input("Masukin nama baru :");
    let new_age = ask_input("Masukin umur baru : ");

    let new_age = new_age.trim_end();

    let new_age = match parse_input_to_num(&new_age) {
        Ok(num) => num,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let new_gender = ask_input("Masukan gender : ");

    let new_person = Person {
        name: new_name,
        age: new_age,
        gender: new_gender,
    };

    save_person(new_person, list);
}

fn print_all_persons(list: &Vec<Person>) {
    for p in list {
        p.print_data();
    }
}
