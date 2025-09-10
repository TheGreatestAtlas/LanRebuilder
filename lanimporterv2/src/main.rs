use std::env;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::{Path, PathBuf};
use encoding_rs::WINDOWS_1250;

fn show_start_screen()
{
    println!("***********************\n|LAN Importer by ATLAS|\n***2025*Rust*Edition***");
}

/// Odczytuje engine flag z pliku engine_flag.txt
/// w formacie: test: {wartość} = ANSI lub test: {wartość} = UTF-16
fn read_engine_flag(folder_path: &Path) -> io::Result<u32> 
{
    let flag_path = folder_path.join("engine_flag.txt");
    let file = File::open(flag_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() 
    {
        let line = line?;
        if let Some((value_part, _encoding)) = line.split_once('=') 
        {
            let value_str = value_part.trim().split_whitespace().last().unwrap_or("1");
            if let Ok(flag) = value_str.parse::<u32>() 
            {
                return Ok(flag);
            }
        }
    }

    Ok(2) // domyślnie UTF-16
}

fn main() -> io::Result<()> 
{
    let args: Vec<String> = env::args().collect();

    let folder_path: PathBuf = if args.len() == 2 
    {
        PathBuf::from(&args[1])
    } 
    else 
    {
        show_start_screen();
        println!("Specify the name of the folder to compile:");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        PathBuf::from(input.trim())
    };

    // odczyt engine flag
    let engine_flag = read_engine_flag(&folder_path)?;

    let folder_name = folder_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let order_file_path = folder_path.join(format!("{folder_name}_order.txt"));
    let order_file = File::open(&order_file_path)?;
    let order_file_reader = io::BufReader::new(order_file);

    let lines: Vec<String> = order_file_reader.lines().collect::<Result<_, _>>()?;

    let output_path = folder_path.with_extension("lan");
    let mut output_file = File::create(&output_path)?;

    let number_of_lans = lines.len() as u32;

    let format = b"LAN\0";

    output_file.write_all(format)?;
    output_file.write_all(&engine_flag.to_le_bytes())?;
    output_file.write_all(&number_of_lans.to_le_bytes())?;

    for line in lines 
    {
        let path = folder_path.join(&line);

        // Obsługa specjalnego przypadku ".txt"
        let (lan_code_name, buffer_bytes) = if line.trim() == ".txt" 
        {
            (String::new(), Vec::new())
        } 
        else 
        {
            // normalny przypadek
            let lan_code_name = Path::new(&line)
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            let mut input_file = File::open(path)?;
            let mut buffer = String::new();
            input_file.read_to_string(&mut buffer)?;

            let buffer_bytes = if engine_flag == 1 
            {
                // ANSI / Windows-1250
                let (encoded, _, _) = WINDOWS_1250.encode(&buffer);
                encoded.into_owned()
            } 
            else 
            {
                // UTF-16
                let utf16: Vec<u16> = buffer.encode_utf16().collect();
                let mut bytes = Vec::with_capacity(utf16.len() * 2);
                for val in utf16 
                {
                    bytes.extend_from_slice(&val.to_le_bytes());
                }
                bytes
            };

            (lan_code_name, buffer_bytes)
        };

		
        let lan_code_name_len = lan_code_name.len() as u32;
		
		
		let length = if engine_flag == 1 
        {
			// ANSI / Windows-1250: liczba bajtów
			buffer_bytes.len() as u32
		} 
        else 
        {
			// UTF-16: liczba 16-bitowych słów
			buffer_bytes.len() as u32 / 2 // bo każdy u16 to 2 bajty
		};

        output_file.write_all(&lan_code_name_len.to_le_bytes())?;
        output_file.write_all(lan_code_name.as_bytes())?;
        output_file.write_all(&length.to_le_bytes())?;
        output_file.write_all(&buffer_bytes)?;
    }

    Ok(())
}
