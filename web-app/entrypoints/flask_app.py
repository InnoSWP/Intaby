from flask import Flask, request
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker

import config
from domain import model
from adapters import orm, repository
from service_layer import services
from entrypoints.api import api_initialization

orm.start_mappers()
# get_session = sessionmaker(bind=create_engine(config.get_postgres_uri()))
# repository.initialize_repo(get_session())
app = Flask(__name__)
api = api_initialization.api_initialization(app)


@app.route('/')
def index_page():
    return "<p>Hello, world</p>"
