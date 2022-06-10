import enum
from dataclasses import dataclass
from datetime import date
from typing import Optional, List, Set, NewType

# Types declaration

# For user
NickName = NewType('NickName', str)
Email = NewType('Email', str)
Password = NewType('Password', str)

# For quiz
QuizId = NewType('QuizId', int)
QuizName = NewType('QuizName', str)

# For question
QuestionAnswer = NewType('QuestionAnswers', str)
QuestionDescription = NewType('QuestionDescription', str)


class Answer:
    def __init__(self, text: QuestionAnswer, is_correct: bool):
        self.text = text
        self.is_correct = is_correct


# Possible types of question in a quiz
class QuestionTypes(enum.Enum):
    poll = "polls"
    quiz = "quiz"


# Question class, to store information about question:
#   question type, description, possible answers, and correct answers
class Question:
    def __init__(self, question_type: QuestionTypes, text: QuestionDescription,
                 answers: List[Answer]):
        self.question_type = question_type
        self.text = text
        self.answers = answers

    # [maybe deleted]
    # return next question
    def next(self):
        pass


# Quiz class,
class Quiz:
    def __init__(self, quiz_id: QuizId, quiz_name: QuizName, questions: [Question]):
        self.quiz_id = quiz_id
        self.quiz_name = quiz_name
        self.questions = questions


# User class, to manage stored in db information through nickname/email
class User:
    def __init__(self, nickname: NickName, email: Email, password: Password, quizzes: [Quiz]):
        self.nickname = nickname
        self.email = email
        self.password = password
        self.quizzes = quizzes
