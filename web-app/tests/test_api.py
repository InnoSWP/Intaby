import json

from requests import get, post, delete


def test_quizzes_post_request():
    quiz_post_request_body = {
        "questions": {"question1": {"Hello": True, "World": True}, "question2": {"Hello": True, "World": True}},
        "name": "Quiz name"
    }

    url = "http://127.0.0.1:8888/api/user/1/quiz"
    return post(url, data=json.dumps(quiz_post_request_body), headers={"Content-Type": "application/json"}).text


def test_quiz_get_request():
    url = "http://127.0.0.1:8080/api/user/1/quiz/1"
    return get(url).text


def test_quiz_delete_request():
    quiz_post_request_body = {
        "questions": {"question1": {"Hello": True, "World": True}, "question2": {"Hello": True, "World": True}},
        "name": "Quiz name"
    }
    url = "http://127.0.0.1:8888/api/user/1/quiz"

    post(url, data=json.dumps(quiz_post_request_body), headers={"Content-Type": "application/json"})

    return delete(url + "/1").text


def test_quizzes_get_request():
    url = "http://127.0.0.1:8888/api/user/1/quiz"

    return get(url).text


def test_quiz_put_request():
    url = "http://127.0.0.1:8888/api/user/1/quiz"

    quiz_put_request_body = {
        "questions": {"question1": {"Hello": True, "World": True}, "question2": {"Hello": True, "World": True}},
        "name": "Quiz name"
    }

    return post(url, json=json.dumps(quiz_put_request_body), headers={"Content-Type": "application/json"}).text


def test_user_get_request():
    url = "http://127.0.0.1:8080/api/user/1"
    return get(url).text


def test_user_post_request():
    url = "http://127.0.0.1:8888/api/user"

    user_post_request_body = {
        'nickname': 'Il',
        'email': 'il@k.r',
        'password': 'strong_pass'
    }
    return post(url, data=json.dumps(user_post_request_body), headers={"Content-Type": "application/json"}).text


def test_question_get_request():
    url = "http://127.0.0.1:8888/api/quiz/1/question/2"
    return get(url).text


def test_question_post_request():
    url = "http://127.0.0.1:8888/api/quiz/1/question/2"

    question_post_request_body = {"answers": ["Incorrect answer", "Correct answer"]}
    return post(url, data=json.dumps(question_post_request_body), headers={"Content-Type": "application/json"}).text


# print(test_quiz_post_request())
# print(test_quiz_get_request())
print(test_user_post_request())
# print(test_question_get_request())
