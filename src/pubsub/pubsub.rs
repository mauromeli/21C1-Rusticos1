use std::collections::HashMap;
use std::sync::mpsc::{Sender};
use std::sync::mpsc;
use std::thread;

#[derive(Debug, Clone)]
pub struct PubSub {
    pub clients: HashMap<String, Sender<String>>,
    pub channels: HashMap<String, Vec<String>>
}

impl PubSub {
    pub fn new() -> PubSub {
        PubSub {
            clients: HashMap::new(),
            channels: HashMap::new()
        }
    }

    pub fn add_client(&mut self, client: Sender<String>, local_adr: String) {
        println!("clientes activos: {:?}", self.clients);
        if self.clients.is_empty() { self.clients = HashMap::new() }
        self.clients.insert(local_adr.clone(), client.clone());
        println!("nuevo cliente: {:?}", local_adr.clone())
    }

    pub fn remove_client(&mut self, client: String) {
        self.clients.remove(&client.clone());
        for subscribed_clients in self.channels.values_mut() {
            for i in 0..subscribed_clients.len() {
                if subscribed_clients[i].clone() == client {
                    subscribed_clients.remove(i);
                }
            }
        }
    }

    pub fn subscribe(&mut self, client: String, channels: Vec<String>)  {

        for channel in channels.clone() {
            //self.counter_map.entry(key.clone()).or_insert(0)
            if let Some(subscribed_clients) = self.channels.get_mut(&channel.clone()) {
                if !subscribed_clients.contains(&client) {
                    subscribed_clients.push(client.clone());
                }
            } else {
                let mut vector = Vec::new();
                vector.push(client.clone());
                self.channels.insert(channel.clone(), vector);
            }
        }

    }


    pub fn unsubscribe_client(&mut self, client: String, channel: String) {
        if let Some(subscribed_clients) = self.channels.get_mut(&channel) {
            for i in 0..subscribed_clients.len() {
                if subscribed_clients[i].clone() == client {
                    subscribed_clients.remove(i);
                }
            }
        }
    }

    pub fn unsubscribe(&mut self, local_add: String, channels: Vec<String>) {
        let actual_channels = self.channels.clone();
        if channels.is_empty() {
            for channel in actual_channels.keys() {
                self.unsubscribe_client(local_add.clone(), channel.clone());
            }
        } else {
            for channel in channels {
                self.unsubscribe_client(local_add.clone(), channel.clone());
            }
        }
    }

    pub fn pub_message(&mut self, channel: String, msg: String ) {
        if let Some(subscribed_clients) = self.channels.get_mut(&channel) {
            for client_address in subscribed_clients {
                if let Some(client) = self.clients.get(client_address) {
                    client.send(msg.clone());
                }
            }
        }
    }


}
