use comfy_table::{ Table, ContentArrangement };

#[derive(Serialize, Deserialize)]
pub struct UserDetails {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ServerId")]
    serverid: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Policy")]
    pub policy: Policy
}  

// Struct to contain the Policy information that is a part of the user details.
#[derive(Serialize, Deserialize)]
pub struct Policy {
    #[serde(rename = "AuthenticationProviderId")]
    pub auth_provider_id: String,
    #[serde(rename = "PasswordResetProviderId")]
    pub pass_reset_provider_id: String,
    #[serde(rename = "IsAdministrator")]
    is_admin: bool,
    #[serde(rename = "IsDisabled")]
    is_disabled: bool
}

impl UserDetails {
    pub fn json_print(users: Vec<UserDetails>) {
        println!("{}", serde_json::to_string_pretty(&users).unwrap());
    }

    pub fn table_print(users: Vec<UserDetails>) {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(120)
            .set_header(vec!["Username", "Admin", "Disabled"]);
        for user in users {
            table.add_row(vec![user.name, user.policy.is_admin.to_string(), user.policy.is_disabled.to_string()]);
        }
        println!("{table}");
    }
}