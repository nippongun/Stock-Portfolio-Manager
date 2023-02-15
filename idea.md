# General idea
This is project is a simple stock portfolio manager that shows the important details of the users stock portfolio. The data is polled from the web with an API, stored in a database, and database interface.

## Design
The design includes simplicity and should aim to learn Rust, programming and APIs.

### APIs
There are plentyful of APIs to choose from. Additonally, there are already many wrappers for the API for Rust thus there is no need to write a new wrapper.
Remember: simplicty over function. Get this project done.
CHoose from here: 
- https://rapidapi.com/blog/best-stock-api/
- https://finnhub.io/
- https://www.alphavantage.co/
https://github.com/notseanray/stock-stalker/blob/master/src/stocks.rs

### DB
Fast and scalable databases are paramount with stock prices. For this, mongoDB is suitable.
Alternatively, sqlite can be used

### Frontend
I have no clue yet. Ask friends to help.
Can be Displayed in the console first.

### Misc
There are many things to manage. API key etc.

#### API keys
Api keys have to be private and somehow passed into the 

### Server framework
With Salvo. 