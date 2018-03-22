extern crate iron_mq_rust;

use iron_mq_rust::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_client() {
        let mq = Client::from_env();
    }

    #[test]
    fn get_queue_info() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test-pull");
        let mut q = mq.queue(queue_name);
        let queue_info = q.info();

        assert!(queue_info.len() > 0);
        assert!(!queue_info.contains("Queue not found"));
        assert!(!queue_info.contains("Invalid project/token combination"));
    }

}
