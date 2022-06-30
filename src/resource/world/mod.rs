mod continent;

use lazy_static::lazy_static;
use rand::Rng;

lazy_static! {
    /// Сид ключ для уникальной генерации планетарного ландшафта
    pub static ref CURRENT_SEED: u32 = 0;

    /// Частота континентов планеты. Более высокая частота производит
    /// более мелкие и многочисленные континенты.
    /// Значение измеряется в радианах.
    pub(super) static ref CONTINENT_FREQUENCY: f64 = 1.0;

    /// Лакунарность континентов планеты. Изменение этого значения приводит к
    /// немного разные континенты. Для достижения наилучших результатов это значение должно
    /// быть случайным, но близким к 2.0.
    pub(super) static ref CONTINENT_LACUNARITY: f64 = 2.208984375;

    /// Лакунарность гор планеты. Изменение значения производит
    /// немного другие горы. Для достижения наилучших результатов это значение должно
    /// быть случайным, но близким к 2.0.
    pub(super) static ref MOUNTAIN_LACUNARITY: f64 = 2.142578125;

    /// Лакунарность холмов планеты. Изменение этого значения приводит к
    /// генерации других холмов. Для достижения наилучших результатов это значение должно быть
    /// случайно, но близко к 2.0.
    pub(super) static ref HILLS_LACUNARITY: f64 = 2.162109375;

    /// Лакунарность равнин планеты. Изменение этого значения приводит к
    /// генерации других равнин. Для достижения наилучших результатов это значение должно быть
    /// случайно, но близко к 2.0.
    pub(super) static ref PLAINS_LACUNARITY: f64 = 2.314453125;

    /// Лакунарность бесплодных земель планеты. Изменение этого значения приводит к
    /// генерации других бесплодных земль. Для достижения наилучших результатов это значение должно
    /// быть случайным, но близким к 2.0.
    pub(super) static ref BADLANDS_LACUNARITY: f64 = 2.212890625;

    /// Определяет "извилистость" гор.
    pub(super) static ref MOUNTAINS_TWIST: f64 = 1.0;

    /// Определяет «извилистость» холмов.
    pub(super) static ref HILLS_TWIST: f64 = 1.0;

    /// Определяет «извилистость» бесплодных земель.
    pub(super) static ref BADLANDS_TWIST: f64 = 1.0;

    /// Определяет уровень моря на планете. Это значение должно быть между -1,0
    /// (минимальная высота планеты) и +1.0 (максимальная высота планеты).
    pub(super) static ref SEA_LEVEL: f64 = 0.0;

    /// Указывает уровень на планете, на котором появляются континентальные шельфы.
    /// Это значение должно быть между -1,0 (минимальная высота планеты) и +1,0
    /// (максимальная высота планеты) и должно быть меньше `SEA_LEVEL`.
    pub(super) static ref SHELF_LEVEL: f64 = -0.375;

    /// Определяет количество гористой местности, которая появляется на
    /// планета. Значения варьируются от 0,0 (горы отсутствуют) до 1,0 (вся местность
    /// покрыто горами). Горный рельеф будет перекрывать холмистую местность.
    /// Поскольку местность бесплодных земель может перекрывать части горной местности
    /// местность, установка `MOUNTAINS_AMOUNT` на 1.0 может не полностью покрывать
    /// местность в горах.
    pub(super) static ref MOUNTAINS_AMOUNT: f64 = 0.48;

    /// Определяет количество холмистой местности, которая появляется на планете.
    /// Значения варьируются от 0,0 (холмы отсутствуют) до 1,0 (вся местность покрыта
    /// холмы). Это значение должно быть меньше `MOUNTAINS_AMOUNT`. Поскольку
    /// горный рельеф будет перекрывать части холмистой местности, а
    /// ландшафт бесплодных земель может перекрывать части холмистой местности, устанавливая
    /// `HILLS_AMOUNT` на 1.0 может не полностью покрывать холмистую местность.
    pub(super) static ref HILLS_AMOUNT: f64 = (1.0 + *MOUNTAINS_AMOUNT) / 2.0;

    /// Определяет количество бесплодных земель, покрывающих планету.
    /// Значения варьируются от 0,0 (без бесплодных земель) до 1,0 (вся местность покрыта
    /// бесплодные земли). Ландшафт бесплодных земель будет накладываться на любой другой тип ландшафта.
    pub(super) static ref BADLANDS_AMOUNT: f64 = 0.3125;

    /// Смещение для применения к определению типа ландшафта. Низкие значения (< 1,0)
    /// заставляют шероховатые области появляться только на больших высотах. Высокие значения
    /// (> 2.0) заставляют шероховатые области появляться на любой высоте.
    /// процент грубых участков на планете не зависит от этого значения.
    pub(super) static ref TERRAIN_OFFSET: f64 = 1.0;

    /// Определяет количество "оледенения" в горах. Это значение
    /// должен быть близок к 1,0 и больше 1,0.
    pub(super) static ref MOUNTAIN_GLACIATION: f64 = 0.375;

    /// Масштабирование для применения к высотам базового континента в планетарных
    /// единицы высоты.
    pub(super) static ref CONTINENT_HEIGHT_SCALE: f64 = (1.0 - *SEA_LEVEL) / 4.0;

    /// Максимальная глубина рек в планетарных единицах высоты.
    pub(super) static ref RIVER_DEPTH: f64 = 0.0234375;
}

pub struct WorldBuilder {
    current_seed: u32,
}

impl WorldBuilder {
    /// Создание конструктора для генерации мира.
    /// По умолчанию используется генерация с случайно сгенерированным seed ключом.
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            current_seed: rng.gen::<u32>(),
        }
    }

    /// Функция позволяющая указать свой seed ключ для генерации мира.
    pub fn set_seed(mut self, seed: u32) -> Self {
        self.current_seed = seed;
        self
    }
}