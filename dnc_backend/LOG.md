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

### Dec 21, 2025
* inserted migration for Admin to access all menu items in setup.

### April 7, 2026
* adding "Expired" VerificationStatus row to represent system-expired verifications.

### April 8, 2026
* Fixed get_approval_code_for_verification_id() to have checks before releasing approval code.
  - approval_code is now 9 characters AAA-BBB-CCC.
  - checks include not ALREADY having the same count(dentist+member+service_date) >=3.
* Cleaned up get_master_list_for_endorsement vs get_master_list_members_for_endorsement_id.

### April 9, 2026
* migrations for creation and alteration, as well as generations for 
  - tooth_service_type (First time, Root Cleaning, Retreatment)
  - tooth_surface, (Distal, Facial, Incisal, Lingual, Mesial, etc) and 
  - verification 
* added API for get_tooth_service_types() and get_tooth_surfaces()