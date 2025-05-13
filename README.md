> How much data your publisher program will send to the message broker in one run?

The program will send five messages of `UserCreatedEventMessage`. Each message contains two fields: a `user_id` and a `user_name`, both of which are `String` values.

> The url of: "amqp://guest:guest@localhost:5672" is the same as in the subscriber program, what does it mean?

This means that we are connecting to the same RabbitMQ server as the subscriber. However, instead of subscribing to messages, this program publishes messages. The structure of the URL is the same, with:

- The first `guest` is the username.
- The second `guest` is the password.
- `localhost:5672` specifies the hostname and port number where the RabbitMQ server is running.
  - `localhost` refers to my local machine
  - `5672` is the default port used by RabbitMQ connections

This setup allows both publisher and subscriber to communicate via the same broker, enabling event-driven architecture.
