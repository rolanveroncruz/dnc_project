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
* Working on migrations now.
  1. dataobjects are what we want to give permissions to.
  2. permissions are specific actions like 'create,' 'read,' and 'update' on a dataobject.
  3. roles are names for groups of permissions.
  4. role-permissions aggregate permissions into roles.
  5. users are people who have roles.
* Completed migrations, including refactoring the creation of the above tables and the insertion of initial data. 
  This insertion of initial data includes hashing the password.
  
### Dec 8, 2025
* Completed the login endpoint.


Dec 20, 2025
* When login is successful, need to return a menu->activation map, which contains information about which menu items 
* are activated for the user.