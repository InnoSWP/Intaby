code = localStorage.getItem("code");
name = localStorage.getItem("name");
let score = 0;
ajax(`http://51.250.99.184:8080/games/${code}`, "GET", response_reg)
function response_reg(data){
    data = JSON.parse(data)

    console.log(data)
 

    for (let i = 0; i < data.leaderboard.length; i++){
        if (data.leaderboard[i].player == name){
            score = data.leaderboard[i].score;

        }
    }
    addUser(data.leaderboard, data.leaderboard.length);
    document.getElementsByClassName("congr")[0].innerHTML = "Congratulations, " + name;
    document.getElementsByClassName("user-result_points")[0].innerHTML = Math.round((score*100))/100 +" points";
    

   

    console.log(document.getElementsByClassName("congr"))
}

function addUser(data, number) {
    var wrapper = document.querySelector('tbody')
    console.log(wrapper)
    for (let index = 0; index < number; index++) {
        var element = createUser(index, data[index])
        wrapper.appendChild(element)
    }
}

function createUser(index, data) {
    var element = document.createElement('tr')

    var userNumber = document.createElement('td')
    userNumber.classList.add("leaderboard_table-user-place")
    userNumber.textContent = index + 1

    var nickname = document.createElement('td')
    nickname.classList.add("leaderboard_table-user-name")
    nickname.textContent = data.player

    var icon = document.createElement('td')
    icon.classList.add("leaderboard_table-user-icon")

    var image = document.createElement('img')

    var score = document.createElement('td')
    score.classList.add("leaderboard_table-user-score")
    score.textContent = Math.round(data.score*100)/100


    switch (index) {
        case 0:
            image.classList.add("leaderboard_1st-place-icon")
            image.src = "./images/leaderboard_1st-place.svg"
            break
        case 1:
            image.classList.add("leaderboard_2nd-place-icon")
            image.src = "./images/leaderboard_2nd-place.svg"
            break
        case 2:
            image.classList.add("leaderboard_3rd-place-icon")
            image.src = "./images/leaderboard_3rd-place.svg"
            break
        default:
            image.classList.add("leaderboard_student-icon")
            image.src = "./images/leaderboard_student.svg"
            break
    }

    icon.appendChild(image)
    element.appendChild(userNumber)
    element.appendChild(icon)
    element.appendChild(nickname)
    element.appendChild(score)
    return element
}