# time_analyzer

## Описание
Данная утилита позволяет замерять время выполнения сторонней программы и выводить ее асимптотическую сложность.
Утилита может генерировать входные значения для сторонней программы, которые описываются в специальном конфигурационном файле.
Сгенерированные значения записываются в файл, из которого сторонней программе необходимо их считать. В файл вначале записывается количество значений, а затем сами значения.

## Установка
Ниже описан процесс сборки данной утилиты. Минимальная поддерживаемая версия rust 1.50. Установку можно произвести с помощью [Rustup](https://rustup.rs/).

````
$ git clone https://github.com/Naymoll/time_analyzer.git
$ cd time_analyzer
# Для Debug сборки
$ cargo build
# Для Release сборки
$ cargo build --release
````

## Описание конфигурационного файла

### Типы генерируемых значений
#### Целые числа 64-битные
````
{
   "type" : "Int",
   "min": 0,
   "max": 1000
}
````
* min - По умолчанию -9_223_372_036_854_775_808.
* max - По умолчанию 9_223_372_036_854_775_807.

#### Вещественные числа 64-битные
````
{
   "type" : "Float",
   "min": 10.0,
   "max": 1000.0,
}
````
* min - По умолчанию -1.7976931348623157E+308.
* max - По умолчанию 1.7976931348623157E+308.

#### Символы строк
Генерируемые значения: 0-9 | a-z | A -Z
````
{
   "type" : "Char",
}
````
#### Булевы значения
Генерируемые значения: 0 | 1
````
{
   "type" : "Bool",
}
````

### Генерируемые аргументы
#### Диапазон чисел
````
{
    "Range": { 
        "start": 1024,
        "end": 160124,
        "multiplier": 2
    }
}
````
* start - начальная длина.
* end - конечная длина.
* multiplier - множитель длины.

Беззнаковые целые, платформозависимые.
#### Массив
````
{
   "Array" : {
       "value": {
           "type": "Int",
           "min" : 10,
           "max": 20000
       },
       "start": 1024,
       "end": 160124,
       "multiplier": 2
   }
}
````
#### Матрица
````
{
   "Matrix" : {
       "value": {
           "type": "Int",
           "min" : 10,
           "max": 20000
       },
       "rows": {
           "start": 1024,
           "end": 160124,
           "multiplier": 2
       },
       "columns": {
           "start": 1024,
           "end": 160124,
           "multiplier": 2
       }
   }
}
````
#### Описание остальных параметров
````
{
   "path": "/path/to/bin/file.out",
   "path_to_temp": "/path/to/tmp/tmp",
   "args": [],
   "gens": 6,
   "iters": 1
}
````
* path - Путь до исполняемого файла.
* path_to_temp - Путь до папки, где будут генерироваться файлы со значениями.
* args - Входные аргументы. Array | Matrix | Range.
* gens - Количество генерация с новой длинной. Беззнаковое целое, платформозависимое.
* iters - Количество повторений генераций в поколении. Беззнаковое целое, платформозависимое.

## Пример описания конфигурационного файла
````
{
   "path": "/path/to/bin/file.out",
   "path_to_temp": "/path/to/tmp/tmp",
   "args": [
       {
           "Array" : {
               "value": {
                   "type": "Int"
               },
               "start": 1024,
               "multiplier": 2
           }
       }
   ],
   "gens": 6,
   "iters": 1
}
````

## Пример работы
![Пример](https://drive.google.com/uc?export=download&id=1waJnhjvjaTWkOnKxGFsKy5Fjt-QeCX4T)
