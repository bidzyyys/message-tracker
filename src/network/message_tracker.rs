use crate::network::message::Message;

/// MessageTracker tracks a configurable fixed amount of messages.
/// Messages are stored first-in-first-out.  
/// Duplicate messages should not be stored in the queue.
pub trait MessageTracker {
    /// Add will add a message to the tracker, deleting the oldest message if necessary
    fn add(&mut self, message: Message);
    /// Delete will delete message from tracker
    fn delete(&mut self, id: &String) -> Option<Message>;
    /// Get returns a message for a given ID.  Message is retained in tracker
    fn get(&self, id: &String) -> Option<Message>;
    /// Messages returns messages in FIFO order
    fn get_all(&self) -> Vec<Message>;
}
