use crate::network::message::Message;
use core::panic;
use std::collections::{HashMap, VecDeque};

/// MessageTracker tracks a configurable fixed amount of messages.
/// Messages are stored first-in-first-out.  
/// Duplicate messages should not be stored in the queue.
pub trait MessageTracker {
    /// Add will add a message to the tracker, deleting the oldest message if necessary
    fn add(&mut self, message: Message);
    /// Delete will delete message from tracker
    fn delete(&mut self, id: &str) -> Option<Message>;
    /// Get returns a message for a given ID.  Message is retained in tracker
    fn get(&self, id: &str) -> Option<Message>;
    /// Messages returns messages in FIFO order
    fn get_all(&self) -> Vec<Message>;
}

struct MessageStore {
    queue: VecDeque<Message>,
    // Mapping from message ID to queue index, works as a cache
    index: HashMap<String, usize>,
    fifo_size: usize,
}

impl MessageStore {
    #[allow(dead_code)]
    fn new(fifo_size: usize) -> Self {
        MessageStore {
            queue: VecDeque::new(),
            index: HashMap::new(),
            fifo_size,
        }
    }

    fn update_indices(&mut self, skip: usize) {
        for (index, message) in self.queue.iter().skip(skip).enumerate() {
            self.index.insert(message.id.clone(), index + skip);
        }
    }

    fn exists(&self, id: &str) -> bool {
        self.get(id).is_some()
    }
}

impl MessageTracker for MessageStore {
    fn add(&mut self, message: Message) {
        let message_id = message.id.clone();

        if self.exists(&message_id) {
            return;
        }

        // Enqueue message at the back of the queue
        self.queue.push_back(message);

        // If the queue size exceeds the configured FIFO size
        // Remove the oldest message
        if self.queue.len() > self.fifo_size {
            if let Some(removed_message) = self.queue.pop_front() {
                // Remove the oldest message from the indices cache
                self.index.remove(&removed_message.id);
                // Update indices cache for all elements
                self.update_indices(0);
            }
        } else {
            // Update the indices cache with the queue position of the new message
            self.index.insert(message_id, self.queue.len() - 1);
        }
    }

    fn delete(&mut self, id: &str) -> Option<Message> {
        // Check if the message_id exists in the index
        match self.index.remove(id) {
            None => None,
            Some(queue_index) => {
                // Remove the message from the queue using the index
                if let Some(removed_message) = self.queue.remove(queue_index) {
                    // Update the index for the remaining messages
                    self.update_indices(queue_index);
                    Some(removed_message)
                } else {
                    panic!("It should never happen")
                }
            }
        }
    }

    fn get(&self, id: &str) -> Option<Message> {
        // Check if the message_id exists in the index
        let msg = self
            .index
            .get(id)
            .map(|&queue_index| self.queue[queue_index].clone());
        match msg {
            Some(m) if m.id != id => panic!("Bad cache value for message: {id}"),
            _ => msg,
        }
    }

    fn get_all(&self) -> Vec<Message> {
        self.queue.clone().into()
    }
}

#[cfg(test)]
mod test {
    use crate::network::message::Message;

    use super::{MessageStore, MessageTracker};

    fn generate_msg_id(n: usize) -> String {
        format!("someID{}", n)
    }

    fn generate_peer_id(n: usize) -> String {
        format!("somePeerID{}", n)
    }

    fn generate_message(n: usize) -> Message {
        Message {
            id: generate_msg_id(n),
            peer_id: generate_peer_id(n),
            data: vec![0, 1, 1],
        }
    }

    fn get_tracker(size: usize) -> impl MessageTracker {
        MessageStore::new(size)
    }

    #[test]
    fn add_get_then_all_messages() {
        let length = 5;
        let mut mt = get_tracker(length);

        for i in 0..5 {
            let msg = generate_message(i);
            mt.add(msg.clone());

            let msg_from_queue = mt.get(&msg.id);
            assert!(msg_from_queue.is_some());
            assert_eq!(msg, msg_from_queue.unwrap());
        }

        assert_eq!(
            mt.get_all(),
            vec![
                generate_message(0),
                generate_message(1),
                generate_message(2),
                generate_message(3),
                generate_message(4)
            ]
        )
    }

    #[test]
    fn add_get_then_all_messages_delete_some() {
        let length = 5;

        let mut mt = get_tracker(length);

        for i in 0..length {
            mt.add(generate_message(i));
            let msg = mt.get(&generate_msg_id(i));
            assert!(msg.is_some());
        }

        assert_eq!(
            mt.get_all(),
            vec![
                generate_message(0),
                generate_message(1),
                generate_message(2),
                generate_message(3),
                generate_message(4),
            ]
        );

        for i in 0..(length - 2) {
            let removed = mt.delete(&generate_message(i).id);
            assert!(removed.is_some());
            assert_eq!(generate_message(i), removed.unwrap())
        }

        assert_eq!(
            mt.get_all(),
            vec![generate_message(3), generate_message(4),]
        );
    }

    #[test]
    fn not_full_with_duplicates() {
        let length = 5;
        let mut mt = get_tracker(length);

        for i in 0..(length - 1) {
            mt.add(generate_message(i));
        }

        for _ in 0..(length - 1) {
            mt.add(generate_message(length - 2));
        }

        assert_eq!(
            mt.get_all(),
            vec![
                generate_message(0),
                generate_message(1),
                generate_message(2),
                generate_message(3),
            ]
        )
    }

    #[test]
    fn not_full_with_duplicates_from_other_peers() {
        let length = 5;
        let mut mt = get_tracker(length);

        for i in 0..(length - 1) {
            mt.add(generate_message(i));
        }

        for _ in 0..(length - 1) {
            let mut msg = generate_message(length - 2);
            msg.peer_id = "somePeerID0".into();
            mt.add(msg);
        }

        assert_eq!(
            mt.get_all(),
            vec![
                generate_message(0),
                generate_message(1),
                generate_message(2),
                generate_message(3),
            ]
        );
    }

    #[test]
    fn overflow_and_cleanup() {
        let length = 5;
        let mut mt = get_tracker(length);

        for i in 0..(length * 2) {
            mt.add(generate_message(i));
        }

        assert_eq!(
            mt.get_all(),
            vec![
                generate_message(5),
                generate_message(6),
                generate_message(7),
                generate_message(8),
                generate_message(9),
            ]
        );
    }

    #[test]
    fn overflow_and_cleanup_with_duplicate() {
        let length = 5;
        let mut mt = get_tracker(length);

        for i in 0..(length * 2) {
            mt.add(generate_message(i));
        }

        for i in length..(length * 2) {
            mt.add(generate_message(i));
        }

        assert_eq!(
            mt.get_all(),
            vec![
                generate_message(5),
                generate_message(6),
                generate_message(7),
                generate_message(8),
                generate_message(9),
            ]
        )
    }

    #[test]
    fn empty_tracker_delete() {
        let length = 5;
        let mut mt = get_tracker(length);
        assert!(mt.delete("bleh").is_none());
    }

    #[test]
    fn empty_tracker_get() {
        let length = 5;
        let mt = get_tracker(length);
        assert!(mt.get("bleh").is_none());
    }

    #[test]
    fn add_get_delete_mixed() {
        let length = 5;
        let mut mt = get_tracker(length);

        for i in 0..6 {
            let msg = generate_message(i);
            mt.add(msg.clone());

            let msg_from_queue = mt.get(&msg.id);
            assert!(msg_from_queue.is_some());
            assert_eq!(msg, msg_from_queue.unwrap());
        }

        assert_eq!(mt.get_all().len(), 5);

        assert!(mt.get(&generate_msg_id(0)).is_none());
        for i in 1..6 {
            assert_eq!(mt.get(&generate_msg_id(i)).unwrap(), generate_message(i));
        }

        // Remove msg and check update
        let remove_idx = 2;
        let remove_id = generate_msg_id(remove_idx);
        mt.delete(&remove_id);
        assert!(mt.get(&remove_id).is_none());
        assert_eq!(
            mt.get(&generate_msg_id(remove_idx + 1)).unwrap(),
            generate_message(remove_idx + 1)
        );

        // Add again and check if exist
        mt.add(generate_message(remove_idx));
        assert_eq!(mt.get(&remove_id).unwrap(), generate_message(remove_idx));

        // Check order
        assert_eq!(
            mt.get_all(),
            vec![
                generate_message(1),
                generate_message(3),
                generate_message(4),
                generate_message(5),
                generate_message(remove_idx),
            ]
        )
    }
}
