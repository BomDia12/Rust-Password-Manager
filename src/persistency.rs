use crate::types::Entry;

pub fn save_data_to_disk(_data: &Vec<Entry>) {
    println!("Data Saved!")
}

pub fn read_data_from_disk(data: &mut Vec<Entry>) {
    for i in mock_data() {
        data.push(i)
    }
}

pub fn strigify_data() {}

pub fn decode_data() {}

fn mock_data() -> Vec<Entry> {
    vec![
        Entry {
            domain: String::from("Domínio teste"),
            username: String::from("Usuário"),
            password: String::from("Senha")
        }
    ]
}