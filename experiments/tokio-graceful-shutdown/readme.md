# Tokio Graceful Shutdown

This is a simple example of how to gracefully shut down a Tokio server.

It follows the article ["Graceful Shutdown"](https://tokio.rs/tokio/topics/shutdown) from the
Tokio [topics page](https://tokio.rs/tokio/topics).

To see this pattern in action you can see my one of examples, [load-generator](../load-generator), or you can see an
official example in [mini-redis](https://github.com/tokio-rs/mini-redis)

---

### Article Preview

```
Graceful Shutdown

The purpose of this page is to give an overview of how to properly implement shutdown in asynchronous applications.

There are usually three parts to implementing graceful shutdown:
1. Figuring out when to shut down.
2. Telling every part of the program to shut down.
3. Waiting for other parts of the program to shut down.
```