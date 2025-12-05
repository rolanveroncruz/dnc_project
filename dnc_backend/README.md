# Backend

This is the backend of the Dental Network Company's (DNC) Digital Project. It uses axum, tokio,
and sea-orm.


### Nov 30, 2025
Setup the repo and the starter code for this backend.


### Dec 2, 2025
Learning to use sea-orm.
1. Setting up Migrations
   1. run `sea-orm-cli migration init` - this creates a migrations folder `migration`


### Dec 5, 2025
* haven't started trying migrations yet. 
* will follow 02prod book re healthcheck endpoint.
* The plan moving forward is to:
  *   learn enough of SeaORM to be able to write migrations to setup the user+roles+permissions 
      tables with initial data.
  * Then implement the login endpoint, which requires checking the credentials against the database.
  * Refactor the code to automate testing the login endpoint.
  * Initialize the frontend to allow logging in.
  * Setup the containerization and deployment.