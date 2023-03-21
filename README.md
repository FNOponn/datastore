# oponn

## GOAL:

Build a template for an app that can be loaded into an AWS EC2 instance via code-build.

The app should do the following:

1. Pull data from an outside api and load it into a struct
2. Upload data from struct to an outside datastore (Atlas-mongodb initially)
   3.1 Finally, serve the data from the struct/mongodb via a GraphQL API
   .2 Create a Redis Cache layer in front of every call to MongoDb, where we check the cache first:
   READ
   .1 check if cache record exists, and if so, pull from cache and return (skip Mongodb)
   WRITE
   .2 update record in MongoDB
   .3 if .2 is successful, check if cache record exists and if so, update it
   3.3 (optional) for 3.2.3 above, don't update mongo directly, but send the update request into an external queue, and return response to user right after updating cache
3. The above app should come with a basic AWS infrastructure for an EC2 instance
   in an ECS connected to a load-balancer. The app should auto-deploy from github
   via codepipeline on every merge to main branch.

4. Include a perf analysis (load-testing) for the app.