### F1-Intruder

F1 is an async HTTP 1.1 fuzzer. It is- to put it bluntly- a Burpsuite Intruder clone. This is for 
everyone who would like to use the burp suite intruder tool within the 
community edition, but without the throttling. 

It does have it's differences, but it is designed to largely be the same. Even a
Burpsuite plugin to send a http request that is in burp, directly to the F1-Intruder.

### What's different?

- The rust binary responsible for the networking is written to be asynchronous. The payloads are sent non-sequentially. 
- F1 will opt to use keep-alive connections whenever possible to speed things up. 


### Todo/Issues:
- Implement optimizations for the data table which shows the http data. It is currently very slow to sort with larger amounts of string data.
- Strip accept-encoding from requests for now, implement encodings later.
- Implement client support for servers that will not maintain keep-alive connections. Still currently works, but it's slow. 
- Used lots of unwrap() in the rust code. Need to iron out some of them.
