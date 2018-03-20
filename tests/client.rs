extern crate iron_mq_rust;

use iron_mq_rust::*;

#[cfg(test)]
mod tests {
    use super::*;

    static HOST: &str = "host";
    static PROJECT_ID: &str = "project_id";
    static TOKEN: &str = "auth token";
    static QUEUE_NAME: &str = "queue name";

    #[test] 
    fn init_client() {
        let mq = Client::new(HOST, PROJECT_ID, TOKEN);
        let base_path = format!("https://{}/3/projects/{}/", HOST, PROJECT_ID);

        assert!(mq.base_path == base_path);
    }

    #[test] 
    fn get_queue_info() {
        let mut mq = Client::new(HOST, PROJECT_ID, TOKEN);
        let queue_info = mq.get_queue_info(QUEUE_NAME);

        assert!(queue_info.len() > 0);
        assert!(!queue_info.contains("Queue not found"));
        assert!(!queue_info.contains("Invalid project/token combination"));
    }

}