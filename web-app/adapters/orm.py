from sqlalchemy import Column, Integer, String, ForeignKey, Text, Enum
from sqlalchemy.orm import declarative_base

from domain.model import QuestionTypes

Base = declarative_base()


class User(Base):
    __tablename__ = 'user'

    id = Column(Integer, primary_key=True, autoincrement=True)
    nickname = Column(Text, unique=True)
    email = Column(Text, unique=True)
    password = Column(Text)


class Quiz(Base):
    __tablename__ = 'quiz'

    id = Column(Integer, primary_key=True, autoincrement=True)
    quiz_name = Column(Text)


class Question(Base):
    __tablename__ = 'question'

    id = Column(Integer, primary_key=True, autoincrement=True)
    quiz_id = Column(Integer, ForeignKey('quiz.id'))
    type = Column(Enum(QuestionTypes))
    description = Column(Text)
    possible_answers = ()
