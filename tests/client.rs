extern crate iron_mq_rust;

use iron_mq_rust::*;
use iron_mq_rust::queue::queue_info::QueueInfo;
use iron_mq_rust::queue::message::Message;

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
        let queue_name = String::from("test");
        let queue_info = mq.create_queue(&queue_name);
    }

    #[test]
    fn create_queue_with_config() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test");
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
    fn update_queue() {
        let mut mq = Client::from_env();
        let queue_name = String::from("update-test");
        let queue_info = mq.create_queue(&queue_name);
        let mut q = mq.queue(queue_info.name.clone());
        let mut config = QueueInfo::new(queue_info.name);
        let message_timeout: u32 = 180;
        let message_expiration: u32 = 600;
        config
            .message_timeout(message_timeout.clone())
            .message_expiration(message_expiration.clone());

        let updated_info: QueueInfo = q.update(&config).unwrap();

        assert_eq!(updated_info.message_timeout.unwrap(), message_timeout);
        assert_eq!(updated_info.message_expiration.unwrap(), message_expiration);
    }

    #[test]
    fn get_queue() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test");
        let q = mq.queue(queue_name.clone());

        assert_eq!(q.name, queue_name);
    }

    #[test]
    fn get_queue_info() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test");
        let info = mq.create_queue(&queue_name);
        let mut q = mq.queue(queue_name);
        let queue_info = q.info();

        assert_eq!(info.name, queue_info.name);
    }

    #[test]
    fn push_message() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test");
        mq.create_queue(&queue_name);
        let mut q = mq.queue(queue_name.clone());
        let queue_info_before_push = q.info();
        let id = q.push_message(Message::with_body("test message"));
        let queue_info_after_push = q.info();

        assert!(id.unwrap().len() > 0);
        assert_eq!(
            queue_info_before_push.size.unwrap() + 1,
            queue_info_after_push.size.unwrap()
        );
    }

    #[test]
    fn push_messages() {
        let mut messages: Vec<Message> = Vec::new();
        messages.push(Message::new("first", 60));
        messages.push(Message::with_body("second"));
        messages.push(Message::with_body("third"));
        let message_count = messages.len();

        let mut mq = Client::from_env();
        let queue_name = String::from("test-pull-multiply");
        mq.create_queue(&queue_name);
        let mut q = mq.queue(queue_name);

        let queue_info_before_push = q.info();
        let ids = q.push_messages(messages);
        let queue_info_after_push = q.info();

        assert!(ids.unwrap().len() == 3);
        assert_eq!(
            queue_info_before_push.size.unwrap() + message_count,
            queue_info_after_push.size.unwrap()
        );
    }

    #[test]
    fn push_strings() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test-multiply-reserve");
        mq.create_queue(&queue_name);
        let mut q = mq.queue(queue_name.clone());
        let queue_info_before_push = q.info();
        let messages = vec!["One", "Two", "Three"];

        let ids = q.push_strings(messages).unwrap();
        assert_eq!(ids.len(), 3);
    }

    #[test]
    fn get_message() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test");
        mq.create_queue(&queue_name);
        let mut q = mq.queue(queue_name.clone());
        let queue_info_before_push = q.info();
        let id = q.push_message(Message::with_body("test message")).unwrap();
        let message = q.get_message(&id).unwrap();
        assert_eq!(id, message.id.unwrap());
    }

    #[test]
    fn reserve_message() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test-reserve");
        mq.create_queue(&queue_name);
        let mut q = mq.queue(queue_name.clone());
        let queue_info_before_push = q.info();
        let id = q.push_message(Message::with_body("test reserve")).unwrap();
        let message = q.reserve_message();
        assert!(message.is_ok());
    }

    #[test]
    fn reserve_messages() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test-multiply-reserve");
        mq.create_queue(&queue_name);
        let mut q = mq.queue(queue_name.clone());
        let queue_info_before_push = q.info();
        let messages = vec![
            Message::with_body("One"),
            Message::with_body("Two"),
            Message::with_body("Three"),
        ];
        let ids = q.push_messages(messages).unwrap();
        let messages = q.reserve_messages(3);
        assert!(messages.is_ok());
        assert_eq!(messages.unwrap().len(), 3);
        q.delete();
    }

    #[test]
    fn long_poll_messages() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test-multiply-long-poll");
        mq.create_queue(&queue_name);
        let mut q = mq.queue(queue_name.clone());
        let queue_info_before_push = q.info();
        let messages = vec![
            Message::with_body("One"),
            Message::with_body("Two"),
            Message::with_body("Three"),
        ];
        let ids = q.push_messages(messages).unwrap();
        let messages = q.long_poll(3, 30, 10, true);
        assert!(messages.is_ok());
        assert_eq!(messages.unwrap().len(), 3);
        q.delete();
    }

    #[test]
    fn release_message() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test-release");
        mq.create_queue(&queue_name);
        let mut q = mq.queue(queue_name.clone());
        let id = q.push_message(Message::with_body("test message for release")).unwrap();
        let message = q.reserve_message().unwrap();
        let delay = 70;
        let msg = q.release_message(message.clone(), delay);

        assert!(msg.contains("Released"));
        assert!(!q.delete_message(message).contains("Deleted"));
    }

    #[test]
    fn delete_message() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test-message-delete");
        mq.create_queue(&queue_name);
        let mut q = mq.queue(queue_name.clone());
        let m = Message::with_body("message for delete");
        let id = q.push_message(m).unwrap();
        let message = q.reserve_message().unwrap();
        let msg = q.delete_message(message);
        assert!(msg.contains("Deleted"));
        q.delete();   
    }

    #[test]
    fn delete_messages() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test-messages-delete");
        mq.create_queue(&queue_name);
        let mut q = mq.queue(queue_name.clone());
        let messages = vec![
            Message::with_body("One"),
            Message::with_body("Two"),
            Message::with_body("Three"),
        ];
        let ids = q.push_messages(messages).unwrap();
        let messages = q.reserve_messages(3);
        let msg = q.delete_messages(messages.unwrap());
        assert!(msg.contains("Deleted"));
        q.delete();
    }

    #[test]
    fn touch_message_with_timeout() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test-message-touch");
        mq.create_queue(&queue_name);
        let mut q = mq.queue(queue_name.clone());
        let m = Message::with_body("message for touch");
        let id = q.push_message(m).unwrap();
        let message = q.reserve_message().unwrap();
        let new_reservation_id = q.touch_message_with_timeout(message, 120);

        assert!(new_reservation_id.is_ok());
    }

    #[test]
    fn touch_message() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test-message-touch");
        mq.create_queue(&queue_name);
        let mut q = mq.queue(queue_name.clone());
        let m = Message::with_body("message for touch");
        let id = q.push_message(m).unwrap();
        let message = q.reserve_message().unwrap();
        let new_reservation_id = q.touch_message(message);

        assert!(new_reservation_id.is_ok());
    }

    #[test]
    fn peek_messages() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test-messages-peek");
        mq.create_queue(&queue_name);
        let mut q = mq.queue(queue_name.clone());
        let messages = vec![
            Message::with_body("One"),
            Message::with_body("Two"),
            Message::with_body("Three"),
        ];
        let ids = q.push_messages(messages).unwrap();
        let earned_messages = q.peek_messages(3);
        assert!(earned_messages.is_ok());
        assert_eq!(earned_messages.unwrap().len(), 3);
        q.delete();
    }

    #[test]
    fn clear_queue() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test-clear-queue");
        mq.create_queue(&queue_name);
        let mut q = mq.queue(queue_name.clone());
        let messages = vec![
            Message::with_body("One"),
            Message::with_body("Two"),
            Message::with_body("Three"),
        ];
        let ids = q.push_messages(messages).unwrap();
        q.clear();
        let messages_after_clear = q.peek_messages(100).unwrap();
        assert_eq!(messages_after_clear.len(), 0);
    }

    #[test]
    #[should_panic]
    fn delete_queue() {
        let mut mq = Client::from_env();
        let queue_name = String::from("test-delete");
        mq.create_queue(&queue_name);
        let mut q = mq.queue(queue_name);
        q.delete();
        q.info();
    }

}
