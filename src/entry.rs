pub struct Entry {
    pub name: String,

    pub label: Option<String>,
    pub tip: Option<String>,

    pub command: String,
    pub check: EntryCheck,
}

impl Entry {
    pub fn can_be_added(&self) -> bool {
        match self.check {
            EntryCheck::Yes => true,
            EntryCheck::No => false,
            EntryCheck::OnlyFolder => false,
            EntryCheck::CommandExist(_) => todo!(),
        }
    }
}

pub enum EntryCheck {
    Yes,
    No,
    OnlyFolder,
    CommandExist(String),
}
