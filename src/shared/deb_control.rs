use debcontrol::parse_str;

pub struct Field {
    pub name: String,
    pub value: String,
}

impl Field {
    pub fn from_old(o: &debcontrol::Field) -> Self {
        Field {
            name: o.name.to_string(),
            value: o.value.clone(),
        }
    }
}

pub struct Paragraph {
    fields: Vec<Field>,
}

impl Paragraph {
    pub fn from_old(o: &debcontrol::Paragraph) -> Self {
        let mut fields = Vec::new();

        for old in o.fields.iter() {
            fields.push(Field::from_old(old))
        }

        Paragraph { fields }
    }
}

pub type Control = Vec<Paragraph>;

pub fn control_from_string(s: String) -> Result<Control, Box<dyn std::error::Error>> {
    let mut control: Control = Vec::new();

    for parag in parse_str(&s).unwrap().iter() {
        control.push(Paragraph::from_old(parag));
    }

    Ok(control)
}
