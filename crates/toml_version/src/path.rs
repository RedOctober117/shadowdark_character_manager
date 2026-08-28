pub enum Path {
    Core { branch: String },
    Sheet { branch: String },
}

impl Path {
    pub fn parse(input: String) -> std::result::Result<Self, ()> {
        if let Some(splits) = input.split_once("::") {
            let branch = splits.1.to_string();
            match splits.0 {
                "core" => Ok(Path::Core { branch: branch }),
                "sheet" => Ok(Path::Sheet { branch: branch }),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}
