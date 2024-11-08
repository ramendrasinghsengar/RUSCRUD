use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::time;
use ic_cdk::caller;
use ic_cdk_macros::*;
use serde::Serialize;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashSet};

// Message structure remains the same
#[derive(CandidType, Serialize, Deserialize, Clone)]
struct Message {
    id: u64,
    author: Principal,
    content: String,
    created_at: u64,
    updated_at: Option<u64>,
    likes: u32,
    replies: Vec<u64>,
    parent_id: Option<u64>,
}

// Custom error type for better error handling
#[derive(CandidType, Serialize, Deserialize)]
enum MessageError {
    NotFound,
    EmptyContent,
    Unauthorized,
    InvalidParent,
    AlreadyDeleted,
}

// Input type for message updates
#[derive(CandidType, Deserialize)]
struct UpdateMessageInput {
    content: String,
}

// Thread-local storage remains the same
thread_local! {
    static MESSAGE_STORE: RefCell<BTreeMap<u64, Message>> = RefCell::new(BTreeMap::new());
    static NEXT_ID: RefCell<u64> = RefCell::new(1);
    static AUTHOR_MESSAGE_COUNT: RefCell<BTreeMap<Principal, u32>> = RefCell::new(BTreeMap::new());
    static DELETED_MESSAGES: RefCell<HashSet<u64>> = RefCell::new(HashSet::new());
}

// CREATE - Improved create_message function
#[update]
fn create_message(content: String, parent_id: Option<u64>) -> Result<Message, MessageError> {
    // Validate content
     if content.trim().is_empty() {
        return Err(MessageError::EmptyContent);
    }
    if parent_id.is_some() && !MESSAGE_STORE.with(|store| store.borrow().contains_key(&parent_id.unwrap())) {
        return Err(MessageError::InvalidParent);
    }

    let caller = caller();
    let id = NEXT_ID.with(|counter| {
        let current = *counter.borrow();
        *counter.borrow_mut() = current + 1;
        current
    });

    let message = Message {
        id,
        author: caller,
        content,
        created_at: time(),
        updated_at: None,
        likes: 0,
        replies: Vec::new(),
        parent_id,
    };

    // Update reply chain for parent message
    if let Some(parent_id) = parent_id {
        MESSAGE_STORE.with(|store| {
            let mut store = store.borrow_mut();
            if let Some(parent_message) = store.get_mut(&parent_id) {
                parent_message.replies.push(id);
            }
        });
    }

    // Update author stats
    AUTHOR_MESSAGE_COUNT.with(|count| {
        let mut count = count.borrow_mut();
        *count.entry(caller).or_insert(0) += 1;
    });

    MESSAGE_STORE.with(|store| {
        store.borrow_mut().insert(id, message.clone());
    });

    Ok(message)
}

// READ - Improved message retrieval functions
#[query]
fn get_message(id: u64) -> Result<Message, MessageError> {
    MESSAGE_STORE.with(|store| {
        store.borrow()
            .get(&id)
            .cloned()
            .ok_or(MessageError::NotFound)
    })
}

#[query]
fn get_messages_by_author(author: Principal) -> Vec<Message> {
    MESSAGE_STORE.with(|store| {
        store.borrow()
            .values()
            .filter(|msg| msg.author == author)
            .cloned()
            .collect()
    })
}

// UPDATE - Improved update function
#[update]
fn update_message(id: u64, input: UpdateMessageInput) -> Result<Message, MessageError> {
    if input.content.trim().is_empty() {
        return Err(MessageError::EmptyContent);
    }

    MESSAGE_STORE.with(|store| {
        let mut store = store.borrow_mut();
        let message = store.get_mut(&id).ok_or(MessageError::NotFound)?;
        
        // Only author can update their message
        if message.author != caller() {
            return Err(MessageError::Unauthorized);
        }

        message.content = input.content;
        message.updated_at = Some(time());
        
        Ok(message.clone())
    })
}

// DELETE - New delete function
#[update]
fn delete_message(id: u64) -> Result<(), MessageError> {
    MESSAGE_STORE.with(|store| {
        let mut store = store.borrow_mut();
        
        // First, verify the message exists and caller is authorized
        let message = store.get(&id).ok_or(MessageError::NotFound)?;
        if message.author != caller() {
            return Err(MessageError::Unauthorized);
        }

        // Clone necessary data before removing the message
        let parent_id = message.parent_id;
        let author = message.author;

        // Check if already deleted
        DELETED_MESSAGES.with(|deleted| {
            if deleted.borrow().contains(&id) {
                return Err(MessageError::AlreadyDeleted);
            }
            deleted.borrow_mut().insert(id);
            Ok(())
        })?;

        // Remove message first
        store.remove(&id);

        // Update parent's replies if parent exists
        if let Some(parent_id) = parent_id {
            if let Some(parent) = store.get_mut(&parent_id) {
                parent.replies.retain(|&reply_id| reply_id != id);
            }
        }

        // Update author message count
        AUTHOR_MESSAGE_COUNT.with(|count| {
            let mut count = count.borrow_mut();
            if let Some(author_count) = count.get_mut(&author) {
                *author_count = author_count.saturating_sub(1);
            }
        });

        Ok(())
    })
}

// Utility function to check if message is deleted
#[query]
fn is_message_deleted(id: u64) -> bool {
    DELETED_MESSAGES.with(|deleted| deleted.borrow().contains(&id))
}

// Export Candid
ic_cdk::export_candid!();