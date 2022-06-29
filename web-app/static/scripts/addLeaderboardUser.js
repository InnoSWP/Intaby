window.onload = addUser(10)

function addUser(number) {
    var wrapper = document.querySelector('tbody')
    console.log(wrapper)
    for (let index = 0; index < number; index++) {
        var element = createUser(index)
        wrapper.appendChild(element)
    }
}

function createUser(index) {
    var element = document.createElement('tr')

    var userNumber = document.createElement('td')
    userNumber.classList.add("leaderboard_table-user-place")
    userNumber.textContent = index + 1

    var nickname = document.createElement('td')
    nickname.classList.add("leaderboard_table-user-name")
    nickname.textContent = "nickname"

    var icon = document.createElement('td')
    icon.classList.add("leaderboard_table-user-icon")

    var image = document.createElement('img')

    var score = document.createElement('td')
    score.classList.add("leaderboard_table-user-score")
    score.textContent = "score"


    switch (index) {
        case 0:
            image.classList.add("leaderboard_1st-place-icon")
            image.src = "/web-app/static/images/leaderboard_1st-place.svg"
            break
        case 1:
            image.classList.add("leaderboard_2nd-place-icon")
            image.src = "/web-app/static/images/leaderboard_2nd-place.svg"
            break
        case 2:
            image.classList.add("leaderboard_3rd-place-icon")
            image.src = "/web-app/static/images/leaderboard_3rd-place.svg"
            break
        default:
            image.classList.add("leaderboard_student-icon")
            image.src = "/web-app/static/images/leaderboard_student.svg"
            break
    }

    icon.appendChild(image)
    element.appendChild(userNumber)
    element.appendChild(icon)
    element.appendChild(nickname)
    element.appendChild(score)
    return element
}