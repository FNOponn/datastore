# oponn

## GOAL:

Build a template for an app that can be loaded into an AWS EC2 instance via code-build.

The app should do the following:

1. Pull data from an outside api and load it into a struct
2. Upload data from struct to an outside datastore (Atlas-mongodb initially)

3. Finally, serve the data from a Datastore struct via a GraphQL API

   - Create a Redis Cache layer in front of every call to MongoDb, where we check the cache first: "books_INSERT ID HERE"

   READ

   - Check if cache record exists, and if so, pull from cache and return (skip Mongodb)

   WRITE

   - Update record in MongoDB
   - If the update is successful, check if cache record exists and if so, update it

   - Optional Optimization: Don't update mongo directly, but send the update request into an external queue, and return response to user right after updating cache

4. Convert schema
   Implement multiple table cache + normalization

   4.1 Implement a one to many relationship within the Datastore. We want a collection of bookstore
   that have multiple books. Add a column to book FK that references the PK of the book store within
   another collection. TEST: Given an array of books, find the bookstores that correlate to each book.

   Bookstore name should be unique. Make it complicated.

   4.2 Implement a cache that reflects this new structure (one to many implementation). Cache should utilize collections with
   index to keep track of which items were modified and need to be updated in the separate collection(table).

5. The above app should come with a basic AWS infrastructure for an EC2 instance
   in an ECS connected to a load-balancer. The app should auto-deploy from github
   via codepipeline on every merge to main branch.

6. Include a perf analysis (load-testing) for the app.

7. Endpoint for client consumption via GraphQL.

//Cache normalization

//Get a batch result, cache it attached to a key related to requesting function.
//Then, when doing an update on a single record, check those batch results for existence of said record,
//and update those batches with new record version

//Create Collection of apples, Collection of oranges (cache + db), both of which contain different breeds.

//Request fruit that matches X size,
//Request fruit of color Y

//Cache these two above responses (Also need to cache
//the tables queried by the request).

//Apple orange size
//Apple orange color

//Tables ZSET
// zset tables [
// "apples:apple_orange_size", "apple:apple_orange_colour", "oranges:apple_orange_size", "oranges:apple_orange_colour"]

//Update an Apple record.
//Do a ZRange on tables lexical match apples\* and loop over results and query Redis for each key.
//Check if update apple exists, if so, update it!

//Apple orange acidity
//Store cached response + update zset tables

//How to recouncile the new Apple record with two cached responses?
