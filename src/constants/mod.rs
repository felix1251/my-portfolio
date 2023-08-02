#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Project {
    pub title: &'static str,
    pub context: &'static str,
}

pub const PROJECTS: &'static [Project] = &[
    Project {
        title: "Gotextbooks Wholesale",
        context: "sample",
    },
    Project {
        title: "BXI Inventory System",
        context: "sample",
    },
    Project {
        title: "Kandori",
        context: "sample",
    },
    Project {
        title: "Lixtagram Social",
        context: "sample",
    },
];
