## What is mrCache?
_______________

The goal of mrCache is to allow for fast setup and integration with a microservice built to handle redis caching. 
Aside from the time spent starting up the redis server and mrCache, the majority of setup should be done with integrating with mrCache's API.
The cache will have a gRPC interface first and I may look at adding a REST interface later on if people want it.

## Building mrCache
_______________

I've built with the following versions.

    Rust: 1.74.0
    Redis: 7.2
    Docker: 24.0.7

Add build instructions.

## Future Features
_______________

This is based on community response and my own ideas. I'm open to suggestions and PRs.
For instance, I have a very limited amount of redis commands implemented out of the box, but would be open to adding further commands if desired.
