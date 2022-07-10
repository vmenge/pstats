use serde_json::json;
use sysinfo::{Process, ProcessExt, System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_processes();

    let stats = sys
        .processes()
        .values()
        .into_iter()
        .map(Process::to_json_string)
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", &stats)
}

trait ToJsonString {
    fn to_json_string(&self) -> String;
}

impl ToJsonString for Process {
    fn to_json_string(&self) -> String {
        json!({
            "name": self.name(),
            "cmd": self.cmd().join(" "),
            "cpu": self.cpu_usage(),
            "mem": self.memory(),
            "disk_read": self.disk_usage().read_bytes,
            "disk_write": self.disk_usage().written_bytes
        })
        .to_string()
    }
}
