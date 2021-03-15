# Talkback Listener

To use webhook listener (also called `automation`), run the host application in a separate shell window passing in the following automation configuration:

```
cargo run -- --host 127.0.0.1:8000 
       --role agent 
       --wallet-type basic 
       --wallet-config "{\"fileName\":\"basicwallet.json\"}" 
       --automation http 
       --automation-config http://localhost:5001
```
Note: Alternative to running the command above, you can use the scripts to run the application. 
See the tools [README](../README.md).

Next, start the webhook listener in its own shell passing in the same value passed in the `--automation-config` argument, above.  In the webhook directory:
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