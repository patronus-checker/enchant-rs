extern crate enchant;

fn main() {
    let word = "borken";
    let lang = "en_US";

    println!("Enchant version {}\n", enchant::version());

    let mut broker = enchant::Broker::new();

    let providers = broker.list_providers();
    for provider in providers {
        println!("{:?}", provider);
    }

    let dicts = broker.list_dicts();
    for dict in dicts {
        println!("{:?}", dict);
    }

    if !broker.dict_exists(lang) {
        println!("Dictionary {} does not exist.", lang);
    } else {
        match broker.request_dict(lang) {
            Ok(dict) => {
                println!("lang: {}", dict.get_lang());
                println!("provider_name: {}", dict.get_provider_name());
                println!("provider_desc: {}", dict.get_provider_desc());
                println!("provider_file: {}\n", dict.get_provider_file());

                if dict.check(word).expect("Could not check the word") {
                    println!("{} was found in the {} dictionary.", word, lang);
                } else {
                    println!("{} was not found in the {} dictionary", word, lang);
                    let suggestions = dict.suggest(word);
                    if !suggestions.is_empty() {
                        println!("Suggestions: {:?}", suggestions);
                    }
                }
            }
            Err(msg) => println!("{}", msg),
        }
    }
}
