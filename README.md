Getting Started
===============

You can start using the IronMQ client by adding it to your Cargo.toml:

```
[dependencies]
iron_mq_rust = {path = "../iron_mq_rust"}
```

Client configuration
----------------
#### Using your iron.json file
Get your iron.json file from [iron.io](https://www.iron.io/) and place it in the root of your project.
```
{
	"project_id":"project id",
    "token":"token",
    "host":"mq-aws-eu-west-1-1.iron.io"
}
```
Then initialize the client ```from_file()``` function:

```
extern crate iron_mq_rust;

use iron_mq_rust::Client;

fn main() {
    let mut client = Client::from_file();
}
```

#### Code configuration
You can also configure the client in your code:
```
extern crate iron_mq_rust;

use iron_mq_rust::Client;

fn main() {
    let host = String::from("host");
    let project_id = String::from("project_id");
    let token = String::from("token");
    let mut client = Client::new(host, project_id, token);
}
```

#### Using environment variables
In this case you can initialize the client using ```from_env()``` function:
```
extern crate iron_mq_rust;

use iron_mq_rust::Client;

fn main() {
    let mut client = Client::from_env();
}
```
Then, you can run your program using environment variables:
```
IRON_HOST=mq-aws-eu-west-1-1.iron.io IRON_PROJECT_ID=project_id IRON_TOKEN=token cargo run
```
Example
-------------------
Simple message pushing example:
```
extern crate iron_mq_rust;

use iron_mq_rust::Client;

fn main() {
    let mut client = Client::from_file();
    let queue_name = String::from("test-pull-queue");
    
    client.create_queue(&queue_name);
    
    let mut queue = client.queue(queue_name);
    let id = queue.push_string("test message");
    println!("{}", id.unwrap());
}
```
**Note:** if you want to push message into an existing queue, skip step of queue creation: ```client.create_queue(&queue_name);```

## Further Links

* [IronMQ Overview](http://dev.iron.io/mq/3/)
* [IronMQ v3 REST/HTTP API](http://dev.iron.io/mq/3/reference/api/)
* [Other Client Libraries](http://dev.iron.io/mq/3/libraries/)

-------------
