extern crate iron_mq_rust;

use iron_mq_rust::*;
use iron_mq_rust::queue::queue_info::QueueInfo;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_client() {
        let mq = Client::from_env();
    }

    #[test]
    fn create_queue() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test-pull");
        let queue_info = mq.create_queue(&queue_name);
    }

    #[test]
    fn create_queue_with_config() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test-pull");
        let mut config = QueueInfo::new(queue_name.clone());
        let message_timeout: u32 = 120;
        let message_expiration: u32 = 5000;
        config
            .message_timeout(message_timeout.clone())
            .message_expiration(message_expiration.clone());
        
        let queue_info = mq.create_queue_with_config(&queue_name, &config);
        
        assert_eq!(queue_info.message_timeout.unwrap(), message_timeout);
        assert_eq!(queue_info.message_expiration.unwrap(), message_expiration);
    }

    #[test] 
    fn get_queue() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test-pull");
        let q = mq.queue(queue_name.clone());

        assert_eq!(q.name, queue_name);
    }

    #[test] 
    fn get_queue_info() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test-pull");
        let info = mq.create_queue(&queue_name);
        let mut q = mq.queue(queue_name);
        let queue_info = q.info();

        assert_eq!(info.name, queue_info.name);
    }

    #[test]
    fn push_message() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test-pull");
        mq.create_queue(&queue_name);
        let mut q = mq.queue(queue_name.clone());
        let queue_info_before_push = q.info();
        let id = q.push_message("test message");
        let queue_info_after_push = q.info();

        assert!(id.len() > 0);
        assert_eq!(queue_info_before_push.size.unwrap() + 1, queue_info_after_push.size.unwrap());
    }

}
