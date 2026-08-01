// pub struct Sheet {
//     name: String,
//     ancestry: CanonicalPath,
//     class: CanonicalPath,
//     level: usize,
//     xp: usize,
//     alignment: CanonicalPath,
//     deity: CanonicalPath,
//     title: CanonicalPath,
//     background: CanonicalPath,
//     hp: isize,
//     stats: SheetPath,
//     languages: Vec<Language>,
//     gear: Vec<Gear>,
//     talents: Vec<Talent>,
// }

// pub struct SheetPath(String);
// pub struct CanonicalPath(String);

// pub struct Language {
//     identity: Identity,
//     source: CanonicalPath,
//     from: Option<SheetPath>,
// }
// pub struct Talent {
//     identity: Identity,
//     bonus: Bonus,
// }
// pub struct Gear {
//     identity:
// }

// pub struct Identity {}
// pub enum Bonus {}

// pub struct SheetItem {
//     identity: Identity,
//     path: Path,
//     components: HashMap<String, Expression>,
// }
// pub trait Path {}

// pub trait Expression {}
// impl Expression for String {}
// pub trait Number {}
// impl Number for i32 {}
// impl Expression for Number {}
// impl Expression for Vec {}
// pub struct SheetPath {}
// impl Path for SheetPath {}
// pub struct CanonicalPath {}
// impl Path for CanonicalPath {}
// pub struct Add {}
// impl Expression for Add {}
// pub struct Identity {}
