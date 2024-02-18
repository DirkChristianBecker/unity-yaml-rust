use crate::Yaml;

fn print_indent(indent: usize) {
    for _ in 0..indent {
        print!("    ");
    }
}

pub fn dump_node(doc: &Yaml, indent: usize) {
    match *doc {
        Yaml::Array(ref v) => {
            // print!("Array: ");
            for x in v {
                dump_node(x, indent + 1);
            }
        }

        Yaml::Hash(ref h) => {
            for (k, v) in &h.map {
                // print!("Hash: ");
                print_indent(indent);
                println!("{:?}:", k);
                dump_node(v, indent + 1);
            }
        }

        Yaml::Alias(ref a) => {
            print_indent(indent);
            println!("{:?}", a);
        }

        Yaml::Original(ref o) => {
            println!("{:?}", o.as_str());
        }

        Yaml::DocumentMeta(t, id) => {
            println!("Meta: {} {}", t, id);
        }
        
        _ => {
            print_indent(indent);
            println!("{:?}", doc);
        }
    }
}
