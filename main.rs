use std::env;
use std::fs;
use std::path::Path;
use std::io;

fn main() -> io::Result<()> {
    // 1. Получаем аргументы, переданные при запуске программы
    let args: Vec<String> = env::args().collect();

    // Проверяем, что передано достаточно аргументов
    // args[0] - это всегда имя самой программы
    // args[1] - папка, args[2..] - расширения
    if args.len() < 3 {
        println!("Недостаточно аргументов!");
        println!("Использование: {} <путь_к_папке> <расширение1> [расширение2] ...", args[0]);
        println!("Пример: {} ./my_folder tmp bak log", args[0]);
        return Ok(());
    }

    let target_dir = &args[1];
    
    // Берем все аргументы начиная с 3-го (индекс 2) и превращаем их в вектор строк в нижнем регистре
    let target_extensions: Vec<String> = args[2..]
        .iter()
        .map(|ext| ext.to_lowercase())
        .collect();

    println!("Создаю папку Trash в '{}'...", target_dir);
    
    // Формируем путь к папке Trash и создаем её
    let trash_path = Path::new(target_dir).join("Trash");
    fs::create_dir_all(&trash_path)?;

    println!("Ищу файлы с расширениями: {:?}...", target_extensions);

    // 2. Читаем содержимое папки
    for entry in fs::read_dir(target_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Пропускаем папки, работаем только с файлами
        if path.is_file() {
            // Получаем расширение файла
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();

                // 3. Главная проверка: есть ли это расширение в нашем списке?
                if target_extensions.contains(&ext_str) {
                    let file_name = entry.file_name();
                    let dest_path = trash_path.join(&file_name);

                    // Перемещаем файл (rename в Rust работает и для перемещения между папками)
                    fs::rename(&path, &dest_path)?;
                    println!("Перемещен: {:?}", file_name);
                }
            }
        }
    }

    println!("Готово! Все ненужные файлы в папке Trash.");
    Ok(())
}