use serde::{
    de::{Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, SerializeSeq, Serializer},
};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Labels {
    msgtype: Option<String>,
    tags: Vec<String>,
    categories: Vec<String>,
    sections: Vec<String>,
    others: Vec<String>,
}

impl Serialize for Labels {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = if self.msgtype.is_some() { 0 } else { 1 }
            + self.tags.len()
            + self.categories.len()
            + self.others.len();
        let mut seq = serializer.serialize_seq(Some(len))?;
        if let Some(ref msg) = self.msgtype {
            seq.serialize_element(&format!("m.type:{msg:}"))?;
        }
        for (prefix, entries) in [
            ("m.tag", self.tags.iter()),
            ("m.cat", self.categories.iter()),
            ("m.section", self.sections.iter()),
        ] {
            for e in entries {
                seq.serialize_element(&format!("{prefix:}:{e:}"))?;
            }
        }
        for e in self.others.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}

pub struct LabelsVisitor;

impl<'de> Visitor<'de> for LabelsVisitor {
    type Value = Labels;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("List of Strings")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut me = Labels::default();
        while let Some(key) = seq.next_element::<String>()? {
            if let Some((prefix, res)) = key.split_once(':') {
                match prefix {
                    // first has priority
                    "m.type" if me.msgtype.is_none() => me.msgtype = Some(res.to_owned()),
                    "m.tag" => me.tags.push(res.to_owned()),
                    "m.section" => me.sections.push(res.to_owned()),
                    "m.cat" => me.categories.push(res.to_owned()),
                    _ => me.others.push(key),
                }
            } else {
                me.others.push(key)
            }
        }
        Ok(me)
    }
}

impl<'de> Deserialize<'de> for Labels {
    fn deserialize<D>(deserializer: D) -> Result<Labels, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(LabelsVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::Labels;
    #[test]
    fn smoketest() -> Result<(), serde_json::Error> {
        let labels = Labels {
            msgtype: Some("m.message".to_owned()),
            tags: vec!["dog".to_owned(), "animal".to_owned(), "carnivor".to_owned()],
            categories: vec!["animals".to_owned()],
            sections: vec!["work".to_owned()],
            others: vec!["whatever".to_owned(), "with:other:test".to_owned()],
        };
        let ser = serde_json::to_string(&labels)?;
        println!("Serialized: {ser:}");

        let after: Labels = serde_json::from_str(&ser)?;
        assert_eq!(labels, after);
        Ok(())
    }

    #[test]
    fn first_type_has_priority() -> Result<(), serde_json::Error> {
        let labels = Labels {
            msgtype: Some("m.message".to_owned()),
            tags: vec!["dog".to_owned(), "animal".to_owned(), "carnivor".to_owned()],
            categories: vec!["animals".to_owned()],
            sections: vec!["work".to_owned()],
            others: vec!["m.type:whatever".to_owned(), "with:other:test".to_owned()],
        };
        let ser = serde_json::to_string(&labels)?;
        println!("Serialized: {ser:}");

        let after: Labels = serde_json::from_str(&ser)?;
        assert_eq!(labels, after);
        Ok(())
    }
}
