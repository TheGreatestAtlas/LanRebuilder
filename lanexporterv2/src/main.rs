
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

fn show_start_screen()
{
    println!("***********************\n|LAN Exporter by ATLAS|\n***2025*Rust*Edition***");
}

fn main() -> io::Result<()>
{
    let args: Vec<String> = env::args().collect();
    
    let filename: String = if args.len() == 2 
    {
        args[1].clone()
    } 
    else 
    {
        show_start_screen();
        println!("Enter the name of the .lan file:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input.trim().to_string()
    };

    let path = Path::new(&filename);

    let stem = path.file_stem()
        .unwrap_or_else(|| Path::new("output").as_os_str())
        .to_string_lossy()
        .to_string();

    // katalog docelowy = katalog z plikiem LAN + nazwa bez rozszerzenia
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let directory_path = parent.join(&stem);
    fs::create_dir_all(&directory_path)?;

    let mut file = File::open(&path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let mut cursor = Cursor::new(buffer);

    // 1. Format
    let mut format_bytes = [0u8; 4];
    cursor.read_exact(&mut format_bytes)?;
    let format = String::from_utf8_lossy(&format_bytes);
    println!("Format: {}", format);

    // 2. Engine flag
    let engine_flag = cursor.read_u32::<LittleEndian>()?;
    println!("Engine flag: {}", engine_flag);

    // Przygotowanie opisu flagi
    let flag_description = if engine_flag == 1
    {
        "ANSI (char8_t)"
    } 
    else if engine_flag >= 2 
    {
        "UTF-16"
    } 
    else 
    {
        "Nieznane kodowanie"
    };

    // Zapis flagi do pliku w katalogu wyjściowym
    let flag_file_path = directory_path.join("engine_flag.txt");
    let mut flag_file = File::create(&flag_file_path)?;
    writeln!(flag_file, "{} = {}", engine_flag, flag_description)?;	

    // 3. Number of entries
    let total_entries = cursor.read_u32::<LittleEndian>()?;
    println!("Number of language entries: {}", total_entries);

    let order_file_path = directory_path.join(format!("{}_order.txt", stem));
    let mut order_file = File::create(&order_file_path)?;

    for i in 0..total_entries 
    {
        // 4. ASCII field length
        let ascii_length = cursor.read_u32::<LittleEndian>()?;

        // 5. ASCII field
        let mut ascii_bytes = vec![0u8; ascii_length as usize];
        cursor.read_exact(&mut ascii_bytes)?;
        let ascii_string = String::from_utf8_lossy(&ascii_bytes);

        println!("ASCII field no. {}: {}", i, ascii_string);

		use encoding_rs::WINDOWS_1250; // przykładowo, można zmienić na inne

		// 6. String length
		let string_length = cursor.read_u32::<LittleEndian>()?;

		// 7. Dane tekstowe w zależności od engine_flag
		let decoded_string = if engine_flag == 1 
        {
		    // Dla engine_flag == 1: ANSI (np. Windows-1250)
		    let mut raw_bytes = vec![0u8; string_length as usize];
		    cursor.read_exact(&mut raw_bytes)?;

		    let (decoded, _, _) = WINDOWS_1250.decode(&raw_bytes);
		    
            decoded.into_owned()
		} 
        else if engine_flag >= 2 
        {
		    // Dla engine_flag == 2: UTF-16 (tak jak było)
		    let mut utf16_bytes = vec![0u8; (string_length * 2) as usize];
		    cursor.read_exact(&mut utf16_bytes)?;

		    let utf16_words: Vec<u16> = utf16_bytes
			    .chunks_exact(2)
			    .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
			    .collect();

		    String::from_utf16(&utf16_words).unwrap_or_else(|_| "UTF16 decoding error".to_string())
		} 
        else 
        {
			return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Nieznany engine_flag: {}", engine_flag)));
		};

		// Zapis pojedynczego pliku
		let output_file_path = directory_path.join(format!("{}.txt", ascii_string));
		let mut output_file = File::create(&output_file_path)?;
		output_file.write_all(decoded_string.as_bytes())?;

		writeln!(order_file, "{}.txt", ascii_string)?;

    }

    Ok(())
}

