# Talkback Listener

To use webhook listener (also called `talkback`), run the host application in a separate shell window passing in the following talkBack configuration:

```
cargo run -- --host 127.0.0.1:8000 
       --role agent 
       --wallet-type basic 
       --wallet-config "{\"fileName\":\"basicwallet.json\"}" 
       --talk-back http 
       --talk-back-config http://localhost:5001
```

Next, start the webhook listener in its own shell passing in the same value passed in the `--talk-back-config` argument, above.  In the webhook directory:
```
cargo run http://localhost:5001
```

Output will appear in the console.  Here is an example:
```
- 2 ----------------------------------------------------------------------- 2 ---
/topic/subtopic
        {
  "object": {
    "a": "b",
    "c": "d"
  },
  "array": [
    1,
    2
  ],
  "string": "Hello World"
}
```