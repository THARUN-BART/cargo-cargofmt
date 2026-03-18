use toml_edit::{DocumentMut, Item, Table, value};

pub fn format(doc: &mut DocumentMut) {
    let package = doc.get("package").cloned();
    let dependencies = doc.get("dependencies").cloned();

    let mut others = Vec::new();
    for (k, v) in doc.iter() {
        if k != "package" && k != "dependencies" {
            others.push((k.to_string(), v.clone()));
        }
    }

    let sorted_dependencies = dependencies.map(|item| {
        if let Item::Table(dep_table) = item {
            let mut entries: Vec<(String, Item)> = dep_table
                .iter()
                .map(|(k, v)| (k.to_string(), v.clone()))
                .collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));

            let mut new_table = Table::new();

            for (k, v) in entries {
                if let Some(val) = v.as_value() {
                    if let Some(s) = val.as_str() {
                        new_table[&k] = value(s); 
                    } else {
                        new_table[&k] = value(val.clone());
                    }
                } else {
                    new_table[&k] = v;
                }
            }

            Item::Table(new_table)
        } else {
            item
        }
    });

    let mut new_doc = DocumentMut::new();

    if let Some(pkg) = package {
        new_doc["package"] = pkg;
    }
    
    for (k, v) in others {
        new_doc[&k] = v;
    }

    if let Some(dep) = sorted_dependencies {
        new_doc["dependencies"] = dep;
    }

    *doc = new_doc;
}
