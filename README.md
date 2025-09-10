# LanRebuilder

## 🇵🇱 Opis

Ten pakiet programów służy do przebudowywania plików `.lan` z gier Reality Pump.  
Jeżeli chcesz dodawać nowe wpisy do plików `.lan`, lepiej użyć innych narzędzi, które są łatwiejsze w obsłudze, np. **LanEditor** od Alanova albo **KS LangEdit** od RP.  
LanRebuilder – jak sama nazwa wskazuje – jest przeznaczony głównie do masowego przebudowywania plików `.lan` i porządkowania wpisów.  

### Pakiet zawiera:
- **LanExporter**
- **LanImporter**

---

### LanExporter
Program służący do eksportu plików `.lan` do formatu `.txt`.

#### Jak używać?
1. Uruchom program.
2. Podaj nazwę pliku `.lan`.
3. Program wyeksportuje dane do folderu o tej samej nazwie.
4. Każdy plik `.txt` odpowiada jednemu wpisowi językowemu.
5. Plik `_order.txt` określa, które pliki zostaną skompilowane i w jakiej kolejności.
6. Aby dodać nowy wpis językowy, utwórz plik `.txt` z tłumaczeniem i dopisz go do listy w `_order.txt`.

Program obsługuje także uruchamianie z parametrami w wierszu poleceń:<br>
LanExporterByAtl.exe nazwa_pliku.lan<br>

---

### LanImporter
Program służący do kompilacji folderu z plikami `.txt` do formatu `.lan`.

#### Jak używać?
1. Uruchom program.
2. Podaj nazwę folderu.
3. Program skompiluje folder do pliku `.lan`.

Program obsługuje także uruchamianie z parametrami w wierszu poleceń:<br>
LanImporterByAtl.exe nazwa_folderu<br>

---

### Dodatkowe informacje
Plik `engine_flag.txt` określa format, w którym zapisywane są wpisy językowe.  
Przykłady formatów:
- `1` = ANSI (char8_t)  
- `2` = UTF-16  
- `> 2` = UTF-16  

---

### Obsługiwane gry
Programy obsługują pliki `.lan` z następujących gier:<br>
-> Earth 2150: Escape from the Blue Planet,<br>
-> Earth 2150: The Moon Project,<br>
-> Earth 2150: Lost Souls,<br>
-> Heli Heroes,<br>
-> World War II: Panzer Claws/Frontline Attack: War Over Europe,<br>
-> World War III: Black Gold,<br>
-> KnightShift/Polanie 2/Once Upon a Knight,<br>
-> KnightShift II: Curse of Souls/Polanie 3,<br>
-> Częściowo Earth 2160 (format o wartości 3),<br>
-> Częściowo Two Worlds (format o wartości 3)<br>


### Programy były testowane na:
-> 1 plikach .lan (czyli wszystkich) z gry Earth 2150: Escape from the Blue Planet - wynik testu 100%,<br>
-> 2 plikach .lan (czyli wszystkich) z gry Earth 2150: The Moon Project - wynik testu 100%,<br>
-> 7 plikach .lan (czyli wszystkich) z gry Earth 2150: Lost Souls - wynik testu 100%,<br>
-> 1 plikach .lan (czyli wszystkich) z gry Heli Heroes - wynik testu 100%,<br>
-> 2 plikach .lan (czyli wszystkich) z gry World War II: Panzer Claws/Frontline Attack: War Over Europe - wynik testu 100%,<br>
-> 2 plikach .lan (czyli wszystkich) z gry World War III: Black Gold - wynik testu 100%,<br>
-> 99 plikach .lan (czyli wszystkich) z gry KnightShift/Polanie 2/Once Upon a Knight - wynik testu 100%,<br>
-> 106 plikach .lan (czyli wszystkich) z gry KnightShift II Curse of Souls/Polanie 3 - wynik testu 100%,<br>
-> 4 plikach .lan (czyli wszystkich) z gry Earth 2160 - wynik testu 20%,<br>
	- Niska wartość wyniku spowodowana trzema zaszyfrowanymi plikami .lan, w dodatku jedyny możliwy do otwarcia plik .lan zawiera dodatkowe wpisy, których mój program jeszcze nie obsługuje.<br>
-> 9 plikach .lan (czyli wszystkich) z gry Two Worlds - wynik testu 22%<br>
	- Niska wartość wyniku spowodowana przez dodatkowe wpisy, których mój program jeszcze nie obsługuje.<br>

---

## 🇬🇧 Description

This package of programs is used to rebuild `.lan` files from Reality Pump games.  
If you want to add new entries to `.lan` files, it is better to use other tools that are easier to use, such as **LanEditor** from Alanov or **KS LangEdit** from RP.  
LanRebuilder – as the name suggests – is mainly intended for mass rebuilding of `.lan` files and organizing entries.  

### The package includes:
- **LanExporter**
- **LanImporter**

---

### LanExporter
A program for exporting `.lan` files to `.txt` format.

#### How to use?
1. Run the program.
2. Enter the name of the `.lan` file.
3. The program will export the data to a folder with the same name.
4. Each `.txt` file corresponds to one language entry.
5. The `_order.txt` file specifies which files will be compiled and in what order.
6. To add a new language entry, create a `.txt` file with the translation and add it to the list in `_order.txt`.

The program also supports running with command line parameters:<br>
LanExporterByAtl.exe file_name.lan<br>

---

### LanImporter
A program used to compile a folder with `.txt` files into the `.lan` format.

#### How to use?
1. Run the program.
2. Enter the folder name.
3. The program will compile the folder into a `.lan` file.

The program also supports running with command line parameters:<br>
LanImporterByAtl.exe directory_name<br>

---

### Additional information
The `engine_flag.txt` file specifies the format in which language entries are saved.  
Format examples:
- `1` = ANSI (char8_t)  
- `2` = UTF-16  
- `> 2` = UTF-16  

---

### Supported games
The programs support `.lan` files from the following games:<br>
-> Earth 2150: Escape from the Blue Planet,<br>
-> Earth 2150: The Moon Project,<br>
-> Earth 2150: Lost Souls,<br>
-> Heli Heroes,<br>
-> World War II: Panzer Claws/Frontline Attack: War Over Europe,<br>
-> World War III: Black Gold,<br>
-> KnightShift/Polanie 2/Once Upon a Knight,<br>
-> KnightShift II: Curse of Souls/Polanie 3,<br>
-> Partially Earth 2160 (format with a value of 3),<br>
-> Partially Two Worlds (format with a value of 3)<br>


### The programs were tested on:
-> 1 .lan file (i.e., all) from the game Earth 2150: Escape from the Blue Planet - test result 100%,<br>
-> 2 .lan files (i.e., all) from the game Earth 2150: The Moon Project - test result 100%,<br>
-> 7 .lan files (i.e. all) from the game Earth 2150: Lost Souls - test result 100%,<br>
-> 1 .lan file (i.e. all) from the game Heli Heroes - test result 100%,<br>
-> 2 .lan files (i.e. all) from the game World War II: Panzer Claws/Frontline Attack: War Over Europe - test result 100%,<br>
-> 2 .lan files (i.e. all) from the game World War III: Black Gold - test result 100%,<br>
-> 99 .lan files (i.e. all) from the game KnightShift/Polanie 2/Once Upon a Knight - test result 100%,<br>
-> 106 .lan files (i.e. all) from the game KnightShift II Curse of Souls/Polanie 3 - test result 100%,<br>
-> 4 .lan files (i.e. all) from the game Earth 2160 - test result 20%,<br>
    - The low result is caused by three encrypted .lan files, and the only .lan file that can be opened contains additional entries that my program does not yet support.<br>
-> 9 .lan files (i.e. all) from the game Two Worlds - test result 22%<br>
    - The low result is caused by additional entries that my program does not yet support.<br>