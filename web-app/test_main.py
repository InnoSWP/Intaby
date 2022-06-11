from flask import Flask

from adapters import orm
from domain import model
from entrypoints.api import api_initialization

orm.start_mappers()
app = Flask(__name__)
api = api_initialization.api_initialization(app)

if __name__ == '__main__':
    app.run(port=8080, host='127.0.0.1')
