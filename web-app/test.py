import json

from requests import get, post


def test_quiz_post_request():
    quiz_post_request_body = {
        "id": 500,
        "questions": "hello",
        "name": "Quiz name"
    }

    url = "http://127.0.0.1:8080/api/user/1/quiz"
    return post(url, data=quiz_post_request_body).text


def test_quiz_get_request():
    url = "http://127.0.0.1:8080/api/user/1/quiz"
    return get(url).text


# get('http://127.0.0.1:8080/api/quiz')
print(test_quiz_post_request())
print(test_quiz_get_request())
