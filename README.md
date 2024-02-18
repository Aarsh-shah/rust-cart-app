## This is a cart application, which will focus on adding all the microservices 

- [x] Mongodb
- [] Auth
- [] Scylla
- [] Kafka
- [x] Cache - redis for the price of the items
- [x] ChatGPT - getting recipe from ingredients
- [x] API Integration for dynamic price fetching
- [] logging
- [] Adding a streaming ml job? 
- [] Frontend?
- [] graph ql
- [] Geospatial index
- [] Time series
- [x] Github
- [x] Lint


### GET API
Get all the grocery items present 

curl --location --request GET 'localhost:3030/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain'

{"apple":6}

### POST API 

- Being use for adding values to an existing object, if doesnt' exist, we create a new object

curl --location --request POST 'localhost:3030/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
"name": "apple",
"quantity": 3
}'

Added to grcoery list

### UPDATE API
Used to update the value of a specific item 

curl --location --request PUT 'localhost:3030/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
"name": "apple",
"quantity": 5
}'

Updated value of items from the grocery list

### DELETE API
Used to delete a specific item from the list

curl --location --request DELETE 'localhost:3030/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
"name": "apple"
}'

Removed item from grocery list

### GET A SINGLE ITEM
This sets the value in the local map after the data is being fetched from the db server

curl --location --request GET 'localhost:3030/v1/groceries/appldfsae' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain'  

<html lang="English"><body><p style="color:red;">Value for item appldfsae</p><p>30</p></body></html>

### GET USING REDIS ITEM
This is used for fetching the value via openfoodfacts api, and store it in api

curl --location --request GET 'localhost:3030/v1/nutrition/3017624010701' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain'

If found: Energy Value for item 3017624010701 is 2255
If not found: Didn't found in redis: New value for code 3017624010701 is 2255

### GET RECIPE OF THE INGREDIENTS
Fetches all the ingredients from the current cart
Ex: Apple, sugar, chocolate

curl --location --request GET 'localhost:3030/v1/recipe' \   
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain'

---------------
## RESPONSE
Here is a recipe for chocolate-covered apples:

Ingredients:
- 4 medium apples
- 1 cup of semisweet chocolate chips
- 2 tablespoons of sugar

Instructions:
1. Wash and dry the apples. Remove the stems and insert a popsicle stick or wooden skewer into the top of each apple.
2. In a double boiler, melt the chocolate chips until smooth.
3. Dip each apple into the melted chocolate, turning to coat evenly. Allow any excess chocolate to drip off.
4. Place the chocolate-covered apples on a baking sheet lined with parchment paper.
5. In a small bowl, mix together the sugar and any leftover chocolate. Drizzle the mixture over the apples for added sweetness.
6. Refrigerate the chocolate-covered apples for at least 1 hour, or until the chocolate hardens.
7. Enjoy your delicious and festive chocolate-covered apples!
--------------

# Export data to csv 

### Using the barcode, we are updating the energy, no need for the name right now 

### ChatGPT - will get expired in may, might have to change acc/get new creds