# Iggy Builder 

This branch replicates an issue with deleting stream and topics on iggy.

## Steps to reproduce:

1) Clone branch

   `git clone --branch stream_delete_issue  https://github.com/marvin-hansen/iggy_builder.git `

3) Run example

   `cargo run --example stream-basic `

3) Run the example 2-3 times to see that messages stop receiving.

When the example code receives messages, it will print them to the console i.e.

```text
Build iggy client and connect it.
Build iggy producer & consumer
Start message stream
Send a test message
###################
Message received: Hello Iggy
###################
```

However, after the second run, when the stream and topic was deleted, the message stream will no longer receive messages
and you see the following log:

```text
Build iggy client and connect it.
Build iggy producer & consumer
Start message stream
Send a test message
Stop the message stream, cleanup, and shutdown iggy client
```

You also see errors in the iggy server log i.e.

```text
 Error receiving data from channel: receiving on a closed channel
2025-02-10T05:26:25.042744Z ERROR ThreadId(13) err_trail::tracing_log_stub: STREAMING_SYSTEMS - consumer group not found for group_id: consumer-group-test_stream-test_topic
2025-02-10T05:26:25.046316Z ERROR ThreadId(13) server::streaming::persistence::task: STREAMING_PERSISTENCE - Error receiving data from channel: receiving on a closed channel
2025-02-10T05:26:27.944191Z ERROR ThreadId(15) err_trail::tracing_log_stub: STREAMING_SYSTEMS - consumer group not found for group_id: consumer-group-test_stream-test_topic
2025-02-10T05:26:27.947179Z ERROR ThreadId(15) server::streaming::persistence::task: STREAMING_PERSISTENCE - Error receiving data from channel: receiving on a closed channel
2025-02-10T05:33:25.040432Z ERROR ThreadId(14) err_trail::tracing_log_stub: STREAMING_SYSTEMS - consumer group not found for group_id: consumer-group-test_stream-test_topic
```
