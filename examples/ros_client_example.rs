extern crate ros;

fn main() {
    let uri = std::os::getenv("ROS_MASTER_URI").unwrap();
    let m = ros::comms::master::Master::new(uri.as_slice());

    println!("{:?}", m.get_system_state("/foobar"));

    println!("{:?}", m.get_uri("/foobar"));

    println!("{:?}", m.lookup_node("foobar", "rosout"));
}
