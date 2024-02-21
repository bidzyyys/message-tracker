use crate::network::message::Message;
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
            self.index.insert(message.id.clone(), index);
        }
    }
}

impl MessageTracker for MessageStore {
    fn add(&mut self, message: Message) {
        let message_id = message.id.clone();

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
                    None
                }
            }
        }
    }

    fn get(&self, id: &str) -> Option<Message> {
        // Check if the message_id exists in the index
        self.index
            .get(id)
            .map(|&queue_index| self.queue[queue_index].clone())
    }

    fn get_all(&self) -> Vec<Message> {
        self.queue.clone().into()
    }
}
