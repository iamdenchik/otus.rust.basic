use std::fmt;
use std::ops::{Add, AddAssign};

#[derive(Clone, Debug)]
struct Cat {
    name: String,
    age: u32,
}

impl Cat {
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone)]
enum Pet {
    Dog {
        #[allow(dead_code)]
        name: String,
        #[allow(dead_code)]
        age: u32,
    },
    Cat(Cat),
}

impl From<Cat> for Pet {
    fn from(cat: Cat) -> Self {
        Pet::Cat(cat)
    }
}

impl TryFrom<Pet> for Cat {
    type Error = &'static str;

    fn try_from(pet: Pet) -> Result<Self, Self::Error> {
        match pet {
            Pet::Cat(cat) => Ok(cat),
            _ => Err("Не кот"),
        }
    }
}

impl Add<u32> for Cat {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Self {
            name: self.name,
            age: self.age + rhs,
        }
    }
}

impl AddAssign<u32> for Cat {
    fn add_assign(&mut self, rhs: u32) {
        self.age += rhs;
    }
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Кот {{ кличка: {}, возраст: {} }}", self.name, self.age)
    }
}

fn main() {
    let mut cat = Cat::new("Кот", 6);

    println!("Дебаг для  кота: {:?}", cat);

    let pet: Pet = cat.clone().into();
    println!("Преобразование животного: {:?}", pet);

    match Cat::try_from(pet) {
        Ok(cat) => println!("Копирования в кота: {:?}", cat),
        Err(err) => println!("Ошибка копирования в кота: {}", err),
    }

    println!("Кличка кота: {}", cat.name());

    cat += 2;
    println!("После добавление двух лет: {}", cat);

    let older_cat = cat + 5;
    println!("Добавляем коту 5 лет: {}", older_cat);

    let dog = Pet::Dog {
        name: String::from("Собака"),
        age: 9,
    };

    println!("Дебаг для собаки: {:?}", dog);

    match Cat::try_from(dog.clone()) {
        Ok(cat) => println!("Копирование собаки в кота: {:?}", cat),
        Err(err) => println!("Ошибка копирования собаки в кота: {}", err),
    }

    let pet_dog: Pet = dog;
    println!("Собака: {:?}", pet_dog);
}
