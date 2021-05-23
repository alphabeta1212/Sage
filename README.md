# Discord bot for Book Club management 

Contains features especially for Book club management. Features include: 

* Fetch Summary of given Book title 
* Add list of owned books for a user
* fetch wanted books from collected data 
* Daily Famous Quotes

## Running environment
This bot runs as a worker dyno in heroku therefore it has to intiated after being deployed using the following command:
  * `heroku ps:scale worker=1`

Set the Environment variable `MONGO_URI`, `API_TOKEN` and `NYT_TOKEN` to their respective uri and api-keys. 

MongoDB databases should have a particular schema designed for this bot. You can find it inside the db_handler crate.
After setting all the environment variable's the bot should run.

