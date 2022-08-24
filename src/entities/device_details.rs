use comfy_table::{ Table, ContentArrangement };

#[derive(Serialize, Deserialize)]
pub struct DeviceDetails {
    pub id: String,
    name: String,
    lastuser: String
}

impl DeviceDetails {
    pub fn new(id: String, name: String, lastuser: String) -> DeviceDetails {
        DeviceDetails{
            id,
            name,
            lastuser
        }
    }

    pub fn json_print(devices: Vec<DeviceDetails>) {
        println!("{}", serde_json::to_string_pretty(&devices).unwrap());
    }

    pub fn table_print(devices: Vec<DeviceDetails>) {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Device Id", "Device Name", "Last Used By"]);
        for device in devices {
            table.add_row(vec![device.id, device.name, device.lastuser]);
        }
        println!("{table}");
    }

    
}