IronMQ Rust Client
----------------

Getting Started
===============

You can start using IronMQ client by first adding it to your Cargo.toml:

```
[dependencies]
iron_mq_rust = {path = "../iron_mq_rust"}
```

Client configuration
----------------
#### Using iron.json file
Get iron.json file from [iron.io](https://www.iron.io/) and put it to the root of your project.
```
{
	"project_id":"project id",
    "token":"token",
    "host":"mq-aws-eu-west-1-1.iron.io"
}
```
Then initialize client ```from_file()``` function:

```
extern crate iron_mq_rust;

use iron_mq_rust::Client;

fn main() {
    let mut client = Client::from_file();
}
```

#### In code configuration
You can also configure your client in code:
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
In this case you initialize client using ```from_env()``` function:
```
extern crate iron_mq_rust;

use iron_mq_rust::Client;

fn main() {
    let mut client = Client::from_env();
}
```
Then you can run your program using environment variables:
```
IRON_HOST=mq-aws-eu-west-1-1.iron.io IRON_PROJECT_ID=project_id IRON_TOKEN=token cargo run
```
Example
-------------------
Simplest message pushing example:
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
**Note:** if you want push message to existing queue - you can skip this step: ```client.create_queue(&queue_name);```

## Further Links

* [IronMQ Overview](http://dev.iron.io/mq/3/)
* [IronMQ v3 REST/HTTP API](http://dev.iron.io/mq/3/reference/api/)
* [Other Client Libraries](http://dev.iron.io/mq/3/libraries/)

-------------