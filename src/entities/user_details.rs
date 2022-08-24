use comfy_table::{ Table, ContentArrangement };

#[derive(Serialize, Deserialize)]
pub struct UserDetails {
    pub username: String,
    #[serde(rename = "IsAdministrator")]
    pub is_admin: bool,
    #[serde(rename = "IsDisabled")]
    pub is_disabled: bool
}

impl UserDetails {
    pub fn new(username: String, is_admin: bool, is_disabled: bool) -> UserDetails{
        UserDetails{
            username,
            is_admin,
            is_disabled
        }
    }

    pub fn json_print(users: Vec<UserDetails>) {
        println!("{}", serde_json::to_string_pretty(&users).unwrap());
    }

    pub fn table_print(users: Vec<UserDetails>) {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Username", "Admin", "Disabled"]);
        for user in users {
            table.add_row(vec![user.username, user.is_admin.to_string(), user.is_disabled.to_string()]);
        }
        println!("{table}");
    }
}