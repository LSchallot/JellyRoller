use comfy_table::{ Table, ContentArrangement };

#[derive(Serialize, Deserialize)]
pub struct DeviceRootJson {
    #[serde(rename = "Items")]
    pub items: Vec<DeviceDetails>
}

#[derive(Serialize, Deserialize)]
pub struct DeviceDetails {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "LastUserName")]
    pub lastusername: String
}

impl DeviceDetails {
    pub fn new(id: String, name: String, lastusername: String) -> DeviceDetails {
        DeviceDetails{
            id,
            name,
            lastusername
        }
    }

    pub fn json_print(devices: &[DeviceDetails]) {
        println!("{}", serde_json::to_string_pretty(&devices).unwrap());
    }

    pub fn table_print(devices: &[DeviceDetails]) {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(120)
            .set_header(vec!["Device Id", "Device Name", "Last Used By"]);
        for device in devices {
            table.add_row(vec![&device.id, &device.name, &device.lastusername]);
        }
        println!("{table}");
    }

    
}