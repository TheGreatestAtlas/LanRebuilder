# LanRebuilder

## 🇵🇱 Opis

Ten program służy do przebudowywania plików .lan z gier Reality Pump.<br>
Jeżeli chcesz dodawać tym programem nowe wpisy do plików .lan to raczej użyj innych programów, które
są łatwiejsze w obsłudze np. LanEditor od Alanova albo KS LangEdit od RP.<br>
Ten program tak jak nazwa wskazuje - jest zorientowany bezpośrednio na masowe przebudowywanie plików .lan i do sprzątania wpisów.<br>

### Pakiet zawiera programy:
----
LanExporter + LanImporter<br>

### LanExporter
-----------
Program, który służy do exportu plików .lan do .txt.

Jak używać?:
------------
1. Włączamy program.
2. Wprowadzamy nazwę pliku z formatem .lan.
3. Program exportuje informacje do folderu o podobnej nazwie.
4. Każdy plik translate to jeden wpis językowy.
5. Plik _order.txt decyduje o tym, które pliki mają zostać skompilowane i w jakiej kolejności.
6. Jeżeli chcemy dopisać wpis językowy, należy stworzyć nowy plik txt z translate oraz dopisać go do listy w _order.txt.

Program działa również w trybie ARGC&ARGV:
LanExporterByAtl.exe <nazwa pliku.lan>

-------------------------------------------------------------------------------------------------------------------------

#### LanImporter
-----------
Program, który służy do kompilacji folderu z plikami .txt do formatu .lan.

Jak używać?:
------------
1. Włączamy program.
2. Wprowadzamy nazwę folderu.
3. Program kompiluje folder do postaci pliku z formatem .lan.

Program działa również w trybie ARGC&ARGV:
LanImporterByAtl.exe <nazwa folderu>

-------------------------------------------------------------------------------------------------------------------------

### Dodatkowe informacje:
---------------------
Plik o nazwie engine_flag.txt zawiera format, w którym są zapisane wpisy językowe.
Przykłady formatów:
1 = ANSI (char8_t)
2 = UTF-16
(większy od 2) = UTF-16

-------------------------------------------------------------------------------------------------------------------------

Programy obsługują pliki .lan z następujących gier:<br>
-> Earth 2150: Escape from the Blue Planet,
-> Earth 2150: The Moon Project,
-> Earth 2150: Lost Souls,
-> Heli Heroes,
-> World War II: Panzer Claws/Frontline Attack: War Over Europe,
-> World War III: Black Gold,
-> KnightShift/Polanie 2/Once Upon a Knight,
-> KnightShift II Curse of Souls/Polanie 3,
-> Częściowo Earth 2160 (format o wartości 3),
-> Częściowo Two Worlds (format o wartości 3)

## 🇬🇧 Description

### LanExporter
-----------
A program that is used to export .lan files to .txt.

How to use:
-----------
1. Turn on the program.
2. We enter the name of the .lan format file.
3. The program exports the information to a directory with a similar name.
4. Each translate file is one language entry.
5. The _order.txt file decides which files are to be compiled and in what order.
6. If you want to add a language entry, create a new txt file with translate and add it to the list in _order.txt.

The program also works in ARGC&ARGV mode:
LanExporterByAtl.exe <filename.lan>

-------------------------------------------------------------------------------------------------------------------------

### LanImporter
-----------
A program that is used to compile a directory of .txt files into .lan format.

How to use:
------------
1. Turn on the program.
2. We enter the name of the directory.
3. The program compiles the directory into a file with .lan format.

The program also works in ARGC&ARGV mode:
LanImporterByAtl.exe <directory name>

-------------------------------------------------------------------------------------------------------------------------

Additional information:
---------------------
The file named engine_flag.txt contains the format in which language entries are stored.
Format examples:
1 = ANSI (char8_t)
2 = UTF-16
(greater than 2) = UTF-16

-------------------------------------------------------------------------------------------------------------------------

The programs supports .wpk files from the following games:<br>
-> Earth 2150: Escape from the Blue Planet,
-> Earth 2150: The Moon Project,
-> Earth 2150: Lost Souls,
-> Heli Heroes,
-> World War II: Panzer Claws/Frontline Attack: War Over Europe,
-> World War III: Black Gold,
-> KnightShift/Polanie 2/Once Upon a Knight,
-> KnightShift II Curse of Souls/Polanie 3,
-> Earth 2160,