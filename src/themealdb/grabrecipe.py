import requests
import json

URL = "https://www.themealdb.com/api/json/v1/1/random.php"

r = requests.get(URL)

if r.status_code == 200:
    data = json.loads(r.content)
    print(data)
