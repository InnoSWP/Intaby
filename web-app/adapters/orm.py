from sqlalchemy import Column, MetaData, Integer, String, ForeignKey, Text, Enum, Table, Boolean
from sqlalchemy.orm import declarative_base
from sqlalchemy.orm import mapper


# from domain.model import QuestionTypes

# Заглушка
# To distinguish between question types
class QuestionTypes:
    pass


# Base = declarative_base()

metadata = MetaData()

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
                       Column('type', Enum(QuestionTypes)),
                       Column('description', Text))

# Table for answers
answer_table = Table("Answer", metadata,
                     Column('id', Integer, primary_key=True, autoincrement=True),
                     Column('question_id', ForeignKey('question.id')),
                     Column('correct_answer'), Boolean)

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
