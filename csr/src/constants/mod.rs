#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Project {
    pub title: &'static str,
    pub description: &'static str,
}

pub const PROJECTS: &'static [Project] = &[
    Project {
        title: "Gotextbooks Wholesale",
        description: "sample",
    },
    Project {
        title: "BXI Inventory System",
        description: "sample",
    },
    Project {
        title: "Kandori",
        description: "sample",
    },
    Project {
        title: "Lixtagram Social",
        description: "sample",
    },
];
