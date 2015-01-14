use std::string;
use xmlrpc;

// apis for talking with a ROS master

pub struct Master {
    uri: string::String,
}


pub type List = Vec<(String, Vec<String>)>;
pub type PublisherList = List;
pub type SubscriberList = List;
pub type ServiceList = List;

impl Master {
    pub fn new(uri: &str) -> Master {
        Master { uri: uri.to_string() }
    }

    pub fn get_system_state(&self, caller_id: &str) -> 
        Option<(PublisherList, SubscriberList, ServiceList)> {
        let client = xmlrpc::Client::new(self.uri.as_slice());
        let mut request = xmlrpc::Request::new("getSystemState");
        request = request.argument(&caller_id.to_string()).finalize();
        let response = client.remote_call(&request).unwrap();
        let return_values: (i32, String, (PublisherList, SubscriberList, ServiceList)) = response.result(0).unwrap();
        let (code, msg, (publist,sublist,serlist)) = return_values;
        Some((publist,sublist,serlist))
    }

    pub fn get_uri(&self, caller_id: &str) -> Option<string::String> {
        let client = xmlrpc::Client::new(self.uri.as_slice());
        let mut request = xmlrpc::Request::new("getUri");
        request = request.argument(&caller_id.to_string()).finalize();
        let response = client.remote_call(&request).unwrap();
        let return_values: (i32, String, String) = response.result(0).unwrap();
        let (code, msg, master_uri) = return_values;
        Some(master_uri)
    }

    pub fn lookup_node(&self, caller_id: &str, node_name: &str) -> Option<string::String> {
        let client = xmlrpc::Client::new(self.uri.as_slice());
        let mut request = xmlrpc::Request::new("lookupNode");
        request = request.argument(&caller_id.to_string()).argument(&node_name.to_string()).finalize();
        let response = client.remote_call(&request).unwrap();
        let return_values: (i32, String, String) = response.result(0).unwrap();
        let (code, msg, master_uri) = return_values;
        Some(master_uri)
    }
}
