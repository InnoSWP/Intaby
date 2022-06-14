from sqlalchemy import Column, MetaData, Integer, String, ForeignKey, Text, Enum, Table, Boolean
from sqlalchemy.orm import registry
from sqlalchemy.orm import mapper

from domain import model

# Заглушка
# To distinguish between question types
# class QuestionTypes:
#     pass


# Base = declarative_base()
mapper_registry = registry()
metadata = mapper_registry.metadata

# relations between tables:
#   - To find quizzes: search quizzes table by user_id
#   - To find questions: search question table by quiz_id
#   - To find answers: search answers table by question_id

# TODO: proper comments for tables

# Table for users
user_table = Table("users", metadata,
                   Column('id', Integer, primary_key=True, autoincrement=True),
                   Column('nickname', Text, unique=True),
                   Column('email', Text, unique=True),
                   Column('password', Text))

# Table for quizzes
quiz_table = Table("quizzes", metadata,
                   Column('user_id', Integer, ForeignKey('user.id')),
                   Column('name', Text))

# Table for questions
question_table = Table("questions", metadata,
                       Column('id', Integer, primary_key=True, autoincrement=True),
                       Column('quiz_id', Integer, ForeignKey('quiz.id')),
                       Column('type', Enum(model.QuestionTypes)),
                       Column('description', Text))

# Table for answers
answer_table = Table("answers", metadata,
                     Column('id', Integer, primary_key=True, autoincrement=True),
                     Column('question_id', ForeignKey('question.id')),
                     Column('text', Text),
                     Column('correct_answer', Boolean))


def start_mappers():
    user_mapper = mapper_registry.map_imperatively(model.User, user_table)
    question_mapper = mapper_registry.map_imperatively(model.Question, question_table)
    answer_mapper = mapper_registry.map_imperatively(model.Answer, answer_table)


if __name__ == '__main__':
    start_mappers()
# Draft
# class User(Base):
#     __tablename__ = 'user'
#
#     id = Column(Integer, primary_key=True, autoincrement=True)
#     nickname = Column(Text, unique=True)
#     email = Column(Text, unique=True)
#     password = Column(Text)
#
#
# class Quiz(Base):
#     __tablename__ = 'quiz'
#
#     id = Column(Integer, primary_key=True, autoincrement=True)
#     user_id = Column(Integer, ForeignKey('user.id'))
#     quiz_name = Column(Text)
#
#
# class Question(Base):
#     __tablename__ = 'question'
#
#     id = Column(Integer, primary_key=True, autoincrement=True)
#     quiz_id = Column(Integer, ForeignKey('quiz.id'))
#     type = Column(Enum(QuestionTypes))
#     description = Column(Text)
#     possible_answers = ()
#
#
# class Answers(Base):
#     pass
